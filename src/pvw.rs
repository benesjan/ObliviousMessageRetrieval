use fhe_math::zq::Modulus;
use itertools::{izip, Itertools};
use ndarray::{Array, Array1, Array2, Axis};
use rand::{
    distributions::{Distribution, Uniform},
    CryptoRng, RngCore,
};
use serde::{Deserialize, Serialize};
use statrs::distribution::Normal;

#[derive(Clone, Debug)]
pub struct PVWParameters {
    pub n: usize,
    pub m: usize,
    pub ell: usize,
    pub variance: f64,
    pub q: u64,
}

impl Default for PVWParameters {
    fn default() -> Self {
        Self {
            n: 450,
            m: 16000,
            ell: 4,
            variance: 1.3,
            q: 65537,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PVWCiphertext {
    pub a: Vec<u64>,
    pub b: Vec<u64>,
}

pub struct PublicKey {
    a: Array2<u64>,
    b: Array2<u64>,
    params: PVWParameters,
}

impl PublicKey {
    pub fn encrypt<R: RngCore + CryptoRng>(&self, m: &[u64], rng: &mut R) -> PVWCiphertext {
        debug_assert!(m.len() == self.params.ell);

        let error = Uniform::new(0u64, 2)
            .sample_iter(rng)
            .take(self.params.m)
            .collect_vec();

        let q = Modulus::new(self.params.q).unwrap();
        let ae = Array1::from_vec(
            self.a
                .outer_iter()
                .map(|a_n_m| {
                    let mut r = a_n_m.to_vec();
                    q.mul_vec(&mut r, &error);
                    q.reduce(r.iter().sum::<u64>())
                })
                .collect(),
        );

        let t = m.iter().map(|v| {
            if *v == 1 {
                q.reduce((3 * self.params.q) / 4)
            } else {
                q.reduce(self.params.q / 4)
            }
        });
        let be = Array1::from_vec(
            izip!(self.b.outer_iter(), t)
                .map(|(b_ell_m, ti)| {
                    let mut r = b_ell_m.to_vec();
                    q.mul_vec(&mut r, &error);
                    q.add(q.reduce(r.iter().sum::<u64>()), ti)
                })
                .collect(),
        );

        PVWCiphertext {
            a: ae.to_vec(),
            b: be.to_vec(),
        }
    }
}

pub struct PVWSecretKey {
    pub key: Array2<u64>,
    pub params: PVWParameters,
}

impl PVWSecretKey {
    pub fn random<R: RngCore + CryptoRng>(params: &PVWParameters, rng: &mut R) -> PVWSecretKey {
        let q = Modulus::new(params.q).unwrap();

        let sk = Array::from_shape_vec(
            (params.ell, params.n),
            q.random_vec(params.n * params.ell, rng),
        )
        .unwrap();

        PVWSecretKey {
            key: sk,
            params: params.clone(),
        }
    }

    pub fn public_key<R: RngCore + CryptoRng>(&self, rng: &mut R) -> PublicKey {
        let q = Modulus::new(self.params.q).unwrap();

        let a = Array::from_shape_vec(
            (self.params.n, self.params.m),
            q.random_vec(self.params.n * self.params.m, rng),
        )
        .unwrap();

        // sk * a;
        let distr = Normal::new(0.0, self.params.variance).unwrap();
        let error = Array::from_shape_vec(
            (self.params.ell, self.params.m),
            q.reduce_vec_i64(
                &distr
                    .sample_iter(rng)
                    .take(self.params.ell * self.params.m)
                    .map(|v| v.round() as i64)
                    .collect_vec(),
            ),
        )
        .unwrap();

        let mut ska = izip!(self.key.outer_iter(), error.outer_iter())
            .flat_map(|(key_ell_n, e_ell_m)| {
                let key_ell_n = key_ell_n.as_slice().unwrap();
                let ska_ell_m = izip!(a.axis_iter(Axis(1)), e_ell_m.iter())
                    .map(|(m_n, e_value)| {
                        let mut r = m_n.to_vec();
                        q.mul_vec(&mut r, key_ell_n);
                        let r = (r.iter().sum::<u64>()) + *e_value;
                        r
                    })
                    .collect_vec();
                ska_ell_m
            })
            .collect_vec();
        q.reduce_vec(&mut ska);
        let ska = Array::from_shape_vec((self.params.ell, self.params.m), ska).unwrap();

        PublicKey {
            a,
            b: ska,
            params: self.params.clone(),
        }
    }

    pub fn decrypt(&self, ct: PVWCiphertext) -> Vec<u64> {
        let q = Modulus::new(self.params.q).unwrap();

        izip!(ct.b.iter(), self.key.outer_iter())
            .map(|(b_ell, k_ell_n)| {
                let mut r = ct.a.clone();
                q.mul_vec(&mut r, &k_ell_n.to_vec());
                let d = q.sub(*b_ell, q.reduce(r.iter().sum::<u64>()));
                (d >= self.params.q / 2) as u64
            })
            .collect()
    }

    pub fn decrypt_shifted(&self, ct: PVWCiphertext) -> Vec<u64> {
        let q = Modulus::new(self.params.q).unwrap();

        izip!(ct.b.iter(), self.key.outer_iter())
            .map(|(b_ell, k_ell_n)| {
                let mut r = ct.a.clone();
                q.mul_vec(&mut r, &k_ell_n.to_vec());

                // shift value left by q/4 so that
                // indices encrypting 0 are near value 0.
                let d = q.sub(
                    q.sub(*b_ell, q.reduce(r.iter().sum::<u64>())),
                    self.params.q / 4,
                );

                // Now values encrypting zero should be in range
                // q - 850 < v < 850 with high probability
                !(self.params.q - 850 <= d || d <= 850) as u64
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::thread_rng;

    #[test]
    fn encrypt() {
        let mut rng = thread_rng();
        let params = PVWParameters::default();
        for _ in 0..10 {
            let sk = PVWSecretKey::random(&params, &mut rng);
            let pk = sk.public_key(&mut rng);

            let distr = Uniform::new(0u64, 2);
            let m = distr
                .sample_iter(rng.clone())
                .take(params.ell)
                .collect_vec();
            let ct = pk.encrypt(&m, &mut rng);

            let d_m = sk.decrypt_shifted(ct);

            assert_eq!(m, d_m)
        }
    }

    #[test]
    fn check_probs() {
        let params = PVWParameters::default();

        let mut rng = thread_rng();
        let sk = PVWSecretKey::random(&params, &mut rng);
        let pk = sk.public_key(&mut rng);

        let sk1 = PVWSecretKey::random(&params, &mut rng);
        let pk1 = sk1.public_key(&mut rng);

        let mut count = 0;
        let mut count1 = 0;
        let observations = 1000;
        for _ in 0..observations {
            let ct = pk.encrypt(&[0, 0, 0, 0], &mut rng);
            let ct1 = pk1.encrypt(&[0, 0, 0, 0], &mut rng);

            if sk.decrypt_shifted(ct) == vec![0, 0, 0, 0] {
                count += 1;
            }

            if sk.decrypt_shifted(ct1) == vec![0, 0, 0, 0] {
                count1 += 1;
            }
        }
        assert!((count as f64 / observations as f64) == 1.0);
        assert!((count1 as f64 / observations as f64) == 0.0);
    }
}

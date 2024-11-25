use rand::Rng;
use crate::utils::{mod_add, mod_inv, mod_mul, mod_pow};

pub struct FeldmanVSS {
    pub secret: i128,
    pub total_shares: i128,
    pub threshold: i128,
    pub shares: Vec<(i128, i128)>,
    pub commitments: Vec<i128>,
    pub p: i128,
    pub g: i128,
}

impl FeldmanVSS {
    pub fn init(secret: i128, total_shares: i128, threshold: i128,p : i128, g : i128) -> Self {
        if threshold > total_shares {
            panic!("Threshold cannot be greater than total shares");
        }
        if secret < 0 {
            panic!("Secret must be a non-negative number");
        }
        FeldmanVSS {
            secret,
            total_shares,
            threshold,
            shares: Vec::new(),
            commitments: Vec::new(),
            p,
            g,
        }
    }
    
    pub fn reconstruct_secret(&self, shares: &Vec<(i128, i128)>) -> Result<i128, String> {
        if shares.len() < self.threshold as usize {
            return Err(String::from("Not enough shares"));
        }
    
        let mut secret = 0;
        for (i, &(x_i, y_i)) in shares.iter().enumerate() {
            let mut numerator = 1;
            let mut denominator = 1;
            
            for (j, &(x_j, _)) in shares.iter().enumerate() {
                if i != j {
                    numerator = mod_mul(numerator, x_j, self.p);
                    let diff = mod_add(x_j, -x_i, self.p);
                    denominator = mod_mul(denominator, diff, self.p);
                }
            }
            
            let denominator_inv = mod_inv(denominator, self.p)?;
            let term = mod_mul(y_i, mod_mul(numerator, denominator_inv, self.p), self.p);
            secret = mod_add(secret, term, self.p);
        }
        
        Ok(secret)
    }

    pub fn verify_share(&self, x: i128, y: i128, commitments: &Vec<i128>) -> bool {
        let lhs = mod_pow(self.g, y, self.p);
        let mut rhs = commitments[0];
        let mut x_power = x;
        for i in 1..commitments.len() {
            let term = mod_pow(commitments[i], x_power, self.p);
            rhs = mod_mul(rhs, term, self.p);
            x_power = mod_mul(x_power, x, self.p);
        }
        lhs == rhs
    }

    
    pub fn generate_commitments(&self, coefficients: &Vec<i128>) -> Vec<i128> {
        coefficients
            .iter()
            .map(|&coeff| {
                let comm = mod_pow(self.g, coeff, self.p);
                comm
            })
            .collect()
    }
    pub fn generate_shares(&mut self) -> (Vec<(i128, i128)>, Vec<i128>) {
        let mut rng = rand::thread_rng();
        let mut coefficients = vec![self.secret];
        for _ in 1..self.threshold {
            let coeff = rng.gen_range(0..self.p);
            coefficients.push(coeff);
        }
        let commitments: Vec<i128> = self.generate_commitments(&coefficients);
        let mut shares = Vec::new();
        for x in 1..=self.total_shares {
            let mut y = coefficients[0];
            let mut x_power = 1;
            for &coeff in coefficients.iter().skip(1) {
                x_power = mod_mul(x_power, x, self.p);
                let term = mod_mul(coeff, x_power, self.p);
                y = mod_add(y, term, self.p);
            }
            shares.push((x, y));
        } 
        self.shares = shares.clone();
        self.commitments = commitments.clone();
        (shares, commitments)
    }
}
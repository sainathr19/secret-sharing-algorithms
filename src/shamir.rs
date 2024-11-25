use rand::Rng;
use crate::utils::{mod_add, mod_inv, mod_mul};

pub struct ShamirSecretSharing {
    pub secret: i128,
    pub total_shares: i128,
    pub threshold: i128,
    pub shares: Vec<(i128, i128)>,
    pub p: i128,
}

impl ShamirSecretSharing {
    pub fn init(secret: i128, total_shares: i128, threshold: i128) -> Self {
        if threshold > total_shares {
            panic!("Threshold cannot be greater than total shares");
        }
        if secret < 0 {
            panic!("Secret must be a non-negative number");
        }
        ShamirSecretSharing {
            secret,
            total_shares,
            threshold,
            shares: Vec::new(),
            p: 2147483647,
        }
    }
    pub fn calculate_y(&self, x: i128, coefficients: &Vec<i128>) -> i128 {
        let mut y = 0;
        let mut x_power = 1;
        
        for &coeff in coefficients {
            y = mod_add(y,mod_mul(coeff, x_power, self.p), self.p);
            x_power = mod_mul(x_power, x, self.p);
        }
        
        y
    }

    pub fn generate_shares(&mut self) -> Vec<(i128, i128)> {
        let mut rng = rand::thread_rng();
        let mut coefficients = vec![self.secret];

        for _ in 1..self.threshold {
            let coeff = rng.gen_range(1..self.p);
            coefficients.push(coeff);
        }
        
        let mut shares = Vec::new();
        for x in 1..=self.total_shares {
            let y = self.calculate_y(x, &coefficients);
            shares.push((x, y));
        }
        
        self.shares = shares.clone();
        shares
    }

    pub fn reconstruct_secret(&self, shares: &Vec<(i128, i128)>) -> Result<i128, String> {
        if shares.len() < self.threshold as usize {
            return Err(String::from("Not enough shares"));
        }
    
        let mut secret = 0;
        
        for (i, &(x_i, y_i)) in shares.iter().enumerate() {
            let mut term = y_i;
            
            for (j, &(x_j, _)) in shares.iter().enumerate() {
                if i != j { 
                    term = mod_mul(term, x_j, self.p);      
                    let diff = mod_add(x_i,-x_j,self.p);
                    let inverse = match mod_inv(diff, self.p) {
                        Ok(inv) => inv,
                        Err(_) => return Err(String::from("Failed to compute modular inverse")),
                    };
                    
                    term = mod_mul(term, inverse, self.p);
                }
            }            
            secret = mod_add(secret,term,self.p);
        }        
        Ok(secret)
    }
}
pub fn mod_add(a: i128, b: i128, p: i128) -> i128 {
    let a_norm = a.rem_euclid(p);
    let b_norm = b.rem_euclid(p);
    let sum = (a_norm + b_norm) % p;
    sum.rem_euclid(p)
}

pub fn mod_mul(a: i128, b: i128, p: i128) -> i128 {
    let a_norm = a.rem_euclid(p);
    let b_norm = b.rem_euclid(p);
    let product = (a_norm as i128 * b_norm as i128) % (p as i128);
    (product as i128).rem_euclid(p)
}

pub fn mod_inv(a: i128, p: i128) -> Result<i128, String> {
    if p <= 0 {
        return Err("Modulus must be positive".to_string());
    }
    let a_norm = a.rem_euclid(p);
    
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (p, a_norm);
    
    while new_r != 0 {
        let quotient = r / new_r;
        let temp_t = t - quotient * new_t;
        t = new_t;
        new_t = temp_t;
        let temp_r = r - quotient * new_r;
        r = new_r;
        new_r = temp_r;
    }
    
    if r > 1 {
        return Err("No modular inverse exists".to_string());
    }
    Ok(t.rem_euclid(p))
}

pub fn mod_pow(base: i128, exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    if exp < 0 {
        let pos_exp = -exp;
        let base_inv = mod_inv(base, modulus).unwrap_or(0);
        return mod_pow(base_inv, pos_exp, modulus);
    }
    
    let mut result = 1;
    let mut base = base.rem_euclid(modulus);
    let mut exp = exp;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp >>= 1;
    }
    
    result
}


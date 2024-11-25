#[cfg(test)]
mod tests {
    use crate::{utils::mod_pow, vss::FeldmanVSS};

    #[test]
    fn test_feldman_vss_share_generation() {
        let secret = 12345;
        let total_shares = 5;
        let threshold = 3;
        let p = 104729;
        let g = 3;

        let mut feldman_vss = FeldmanVSS::init(secret, total_shares, threshold, p, g);
        let (shares, commitments) = feldman_vss.generate_shares();

        assert_eq!(shares.len(), total_shares as usize, "Incorrect number of shares");
        assert_eq!(commitments.len(), threshold as usize, "Incorrect number of commitments");

        let g_to_secret = mod_pow(g, secret, p);
        assert_eq!(commitments[0], g_to_secret, "First commitment does not match the secret");
    }

    #[test]
    fn test_feldman_vss_verify_share() {
        let secret = 5;
        let total_shares = 3;
        let threshold = 2;
        let p = 23;
        let g = 2;

        let mut feldman_vss = FeldmanVSS::init(secret, total_shares, threshold, p, g);
        let (shares, commitments) = feldman_vss.generate_shares();
        for &(x, y) in &shares {
            assert!(
                feldman_vss.verify_share(x, y, &commitments),
                "Share verification failed"
            );
        }
    }

    #[test]
    fn test_feldman_vss_reconstruct_secret() {
        let secret = 54321;
        let total_shares = 7;
        let threshold = 3;
        let p = 104729;
        let g = 3;

        let mut feldman_vss = FeldmanVSS::init(secret, total_shares, threshold, p, g);
        let (shares, _) = feldman_vss.generate_shares();

        let shares_subset = shares[0..threshold as usize].to_vec();
        let reconstructed_secret = feldman_vss
            .reconstruct_secret(&shares_subset)
            .expect("Failed to reconstruct secret");

        assert_eq!(reconstructed_secret, secret, "Reconstructed secret does not match original");
    }

    #[test]
    fn test_feldman_vss_reconstruct_secret_with_insufficient_shares() {
        let secret = 98765;
        let total_shares = 5;
        let threshold = 4;
        let p = 999983;
        let g = 5;

        let mut feldman_vss = FeldmanVSS::init(secret, total_shares, threshold, p, g);
        let (shares, _) = feldman_vss.generate_shares();

        let shares_subset = shares[0..(threshold as usize - 1)].to_vec();
        let result = feldman_vss.reconstruct_secret(&shares_subset);

        assert!(
            result.is_err(),
            "Reconstruction should fail with insufficient shares"
        );
    }
}

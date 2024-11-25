#[cfg(test)]
mod tests {
    use crate::shamir::ShamirSecretSharing;

    
    #[test]
    fn test_generate_and_reconstruct_secret() {
        let secret = 12345;
        let total_shares = 5;
        let threshold = 3;

        let mut sss = ShamirSecretSharing::init(secret, total_shares, threshold);
        let shares = sss.generate_shares();
        assert_eq!(shares.len(), total_shares as usize);
        let reconstructed_secret = sss.reconstruct_secret(&shares[..threshold as usize].to_vec()).unwrap();
        assert_eq!(reconstructed_secret, secret);
        let reconstructed_secret_all = sss.reconstruct_secret(&shares).unwrap();
        assert_eq!(reconstructed_secret_all, secret);
    }

    #[test]
    fn test_insufficient_shares() {
        let secret = 9876;
        let total_shares = 5;
        let threshold = 3;

        let mut sss = ShamirSecretSharing::init(secret, total_shares, threshold);
        let shares = sss.generate_shares();

        let result = sss.reconstruct_secret(&shares[..(threshold - 1) as usize].to_vec());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Not enough shares");
    }

    #[test]
    fn test_edge_case_threshold_equals_total_shares() {
        let secret = 54321;
        let total_shares = 5;
        let threshold = 5;

        let mut sss = ShamirSecretSharing::init(secret, total_shares, threshold);
        let shares = sss.generate_shares();
        let reconstructed_secret = sss.reconstruct_secret(&shares).unwrap();
        assert_eq!(reconstructed_secret, secret);
    }

    #[test]
    fn test_edge_case_single_share() {
        let secret = 1111;
        let total_shares = 1;
        let threshold = 1;

        let mut sss = ShamirSecretSharing::init(secret, total_shares, threshold);
        let shares = sss.generate_shares();
        let reconstructed_secret = sss.reconstruct_secret(&shares).unwrap();
        assert_eq!(reconstructed_secret, secret);
    }
}

mod vss;
mod shamir;
mod utils;
mod tests;
use shamir::ShamirSecretSharing;
use vss::FeldmanVSS;
fn main() {
    let secret = 5; 
    let total_shares = 3;
    let threshold = 2; 
    let prime = 23; 
    let g = 2;

    // Shamir Secret Sharing
    let mut shamir = ShamirSecretSharing::init(secret, total_shares, threshold);
    let shares = shamir.generate_shares();
    println!("{:?}",&shares);
    let recovered_secret  = shamir.reconstruct_secret(shares[0..3].to_vec().as_ref());
    match recovered_secret {
        Ok(secret) =>{
            println!("Reconstructed secret is : {}",secret);
        }
        Err(err)=>{
            println!("Error : {}",err);
        }
    }

    // FeldMan VSS
    let mut vss = FeldmanVSS::init(secret, total_shares, threshold, prime, g);
    let (shares, commitments) = vss.generate_shares();

    println!("\nShares:");
    for (i, share) in shares.iter().enumerate() {
        println!("Share {}: {:?}", i+1, share);
    }

    println!("\nVerification:");
    for &(x, y) in &shares {
        println!("\nVerifying share ({}, {})", x, y);
        let valid = vss.verify_share(x, y, &commitments);
        println!("Valid: {}", valid);
    }
    let reconstruction_shares = shares[0..threshold as usize].to_vec();
    match vss.reconstruct_secret(&reconstruction_shares) {
        Ok(recovered) => println!("\nRecovered secret: {}", recovered),
        Err(e) => println!("\nError: {}", e),
    }
}

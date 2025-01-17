mod blockchain;

use blockchain::secret;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mnemonic = secret::generate_secret::<32>().await?;
    println!("Mnemonic: {:?}", mnemonic);
    let entropy = secret::mnemonic_to_entropy(mnemonic).await?;
    println!("Entropy: {:?}", entropy);

    Ok(())
}

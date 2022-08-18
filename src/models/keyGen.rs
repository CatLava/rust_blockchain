use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message, Signing, All, SecretKey, PublicKey};
use secp256k1::hashes::sha256;


pub fn sign_message(secretKey: SecretKey, publicKey: PublicKey, message: String) {
    let hashed_messaged = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());

}

#[derive(Debug)]
pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey
}

impl Wallet {
    pub fn generate_wallet_keys() -> Wallet {
        let secp: Secp256k1<All> = Secp256k1::new();
        let (secret_key, pub_key) = secp.generate_keypair(&mut OsRng);
        let wallet = Wallet {
            private_key: secret_key,
            public_key: pub_key
        };
        wallet
    }
}
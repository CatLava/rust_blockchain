// This uses crate seen here
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message, Signing, All, SecretKey, PublicKey};
use secp256k1::hashes::sha256;
use secp256k1::ecdsa::Signature;
use serde::{Deserialize, Serialize};




#[derive(Debug, Clone, Copy)]
pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey
}

#[derive(Debug, Clone, Hash)]
pub struct BlockchainMessage {
    pub message: String,
    pub hashed_message: Message,
    pub signed_hash: Signature,
    pub pub_key: PublicKey
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
    // This is regardless of what happens, this signs whatever transaction
    pub fn sign_message(self, message: String) -> BlockchainMessage  {
        let secp: Secp256k1<All> = Secp256k1::new();
        let hashed_message = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        let sig = secp.sign_ecdsa(&hashed_message, &self.private_key);
        //let (fake_secret_key, fake_pub_key) = secp.generate_keypair(&mut OsRng);
        //println!("{:?}", hashed_message);
        //println!("{:?}", sig);
        //println!("{:?}", secp.verify_ecdsa(&hashed_message, &sig, &fake_pub_key));
        return BlockchainMessage { message: message, hashed_message: hashed_message, signed_hash: sig, pub_key: self.public_key }

    }
}
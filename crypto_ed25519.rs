use ed25519_dalek::{
    Signer, Verifier, Signature, SigningKey, VerifyingKey,
    SecretKey, PublicKey, Seed
};
use rand::rngs::OsRng;
use hex::{encode, decode};

pub struct Ed25519Crypto {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Ed25519Crypto {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = VerifyingKey::from(&signing_key);
        
        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn from_seed(seed_hex: &str) -> Result<Self, String> {
        let seed_bytes = decode(seed_hex).map_err(|e| e.to_string())?;
        if seed_bytes.len() != 32 {
            return Err("Seed must be 32 bytes".to_string());
        }
        let seed = Seed::from_slice(&seed_bytes);
        let signing_key = SigningKey::from_seed(&seed);
        let verifying_key = VerifyingKey::from(&signing_key);
        
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    pub fn sign_message(&self, message: &[u8]) -> String {
        let signature: Signature = self.signing_key.sign(message);
        encode(signature.to_bytes())
    }

    pub fn verify_message(
        &self,
        message: &[u8],
        signature_hex: &str
    ) -> Result<bool, String> {
        let sig_bytes = decode(signature_hex).map_err(|e| e.to_string())?;
        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|_| "Invalid signature format")?;
        
        match self.verifying_key.verify(message, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn get_public_key_hex(&self) -> String {
        encode(self.verifying_key.to_bytes())
    }

    pub fn get_private_key_hex(&self) -> String {
        encode(self.signing_key.to_bytes())
    }
}

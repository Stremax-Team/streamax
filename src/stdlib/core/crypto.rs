use sha2::{Sha256, Digest};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use crate::core::{Error, Result};

/// Hash functions
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(&mut output);
    output
}

/// Key pair for digital signatures
pub struct KeyPair {
    inner: Keypair,
}

impl KeyPair {
    /// Generate a new random key pair
    pub fn generate() -> Result<Self> {
        use rand::rngs::OsRng;
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        Ok(KeyPair { inner: keypair })
    }
    
    /// Create from secret key
    pub fn from_secret(secret: &[u8]) -> Result<Self> {
        let secret = SecretKey::from_bytes(secret)
            .map_err(|_| Error::CryptoError("Invalid secret key".into()))?;
        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        Ok(KeyPair { inner: keypair })
    }
    
    /// Get public key
    pub fn public_key(&self) -> PublicKey {
        self.inner.public
    }
    
    /// Sign message
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.inner.sign(message)
    }
}

/// Verify signature
pub fn verify(
    public_key: &PublicKey,
    message: &[u8],
    signature: &Signature,
) -> bool {
    public_key.verify(message, signature).is_ok()
}

/// Generate random bytes
pub fn random_bytes(len: usize) -> Vec<u8> {
    use rand::{thread_rng, RngCore};
    let mut rng = thread_rng();
    let mut bytes = vec![0u8; len];
    rng.fill_bytes(&mut bytes);
    bytes
}

/// Constant time comparison
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// HMAC
pub fn hmac(key: &[u8], message: &[u8]) -> [u8; 32] {
    use hmac::{Hmac, Mac, NewMac};
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_varkey(key)
        .expect("HMAC can take key of any size");
    mac.update(message);
    mac.finalize().into_bytes().into()
}

/// Password hashing
pub fn hash_password(password: &[u8], salt: &[u8]) -> Vec<u8> {
    use argon2::{self, Config};
    
    let config = Config::default();
    argon2::hash_encoded(password, salt, &config)
        .unwrap()
        .into_bytes()
}

/// Verify password hash
pub fn verify_password(password: &[u8], hash: &[u8]) -> bool {
    use argon2::verify_encoded;
    
    verify_encoded(
        std::str::from_utf8(hash).unwrap(),
        password,
    ).unwrap_or(false)
}
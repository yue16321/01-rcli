use crate::cli::TextSignFormat;
use crate::{gen_pass, read_content};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::io::Read;

pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}
pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> anyhow::Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}
impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = key[..32].try_into()?;
        Ok(Self::new(key))
    }
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let key = gen_pass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes() == sign)
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = key[..32].try_into()?;
        Ok(Self::new(key))
    }
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(&key),
        }
    }
    fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = key[..32].try_into()?;
        let key = VerifyingKey::from_bytes(&key)?;
        Ok(Self { key })
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let buf = read_content(reader)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> anyhow::Result<bool> {
        let buf = read_content(reader)?;
        let sign = (&sign[..64]).try_into()?;
        let signature = Signature::from_bytes(sign);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> anyhow::Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };
    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sign: &[u8],
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(reader, sign)
}
pub fn process_text_key_generate(
    format: TextSignFormat,
) -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

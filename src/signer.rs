use cosmrs::bip32::{Language, Mnemonic, XPrv};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::crypto::PublicKey;
use cosmrs::AccountId;
use hex::decode;
use rand_core::OsRng;

use crate::error::Result;

/// Struct representing a signer with mnemonic, public address, private key, and public key.
pub struct Signer {
    pub mnemonic: Option<String>,
    pub public_address: AccountId,
    pub private_key: SigningKey,
    pub public_key: PublicKey,
}

impl std::fmt::Debug for Signer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signer").finish()
    }
}

impl Signer {
    /// Loads a signer from a mnemonic phrase.
    ///
    /// # Arguments
    ///
    /// * `phrase` - The mnemonic phrase.
    /// * `prefix` - The prefix for the account ID.
    /// * `derivation` - The optional derivation path.
    ///
    /// # Returns
    ///
    /// A Result containing the private key, public key, and public address or an error.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the mnemonic is invalid
    /// - the derivation path is invalid
    /// - the prefix is invalid
    fn load_from_mnemonic(
        phrase: &str,
        prefix: &str,
        derivation: Option<&str>,
        password: Option<&str>,
    ) -> Result<(SigningKey, PublicKey, AccountId)> {
        let derivation = derivation.unwrap_or("m/44'/118'/0'/0/0");

        let mnemonic = Mnemonic::new(phrase, Language::English)?;
        let pri = XPrv::derive_from_path(
            mnemonic.to_seed(password.unwrap_or("")),
            &derivation.parse()?,
        )?;
        let private_key = SigningKey::from(pri);
        let public_key = private_key.public_key();
        let public_address = public_key.account_id(prefix)?;

        Ok((private_key, public_key, public_address))
    }

    /// Generates a new signer with a random mnemonic.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix for the account ID.
    /// * `derivation` - The optional derivation path.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance of Signer or an error.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the derivation path is invalid
    /// - the prefix is invalid
    pub fn generate(
        prefix: &str,
        derivation: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self> {
        let mnemonic = Mnemonic::random(OsRng, Language::English);
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(mnemonic.phrase(), prefix, derivation, password)?;

        Ok(Signer {
            mnemonic: Some(mnemonic.phrase().to_string()),
            public_address,
            private_key,
            public_key,
        })
    }

    /// Creates a signer from a private key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key as a hex string.
    /// * `prefix` - The prefix for the account ID.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance of Signer or an error.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the private key is invalid
    /// - the prefix is invalid
    pub fn from_pkey(private_key: &str, prefix: &str) -> Result<Self> {
        let private_key = SigningKey::from_slice(decode(private_key)?.as_slice())?;
        let public_key = private_key.public_key();
        let public_address = public_key.account_id(prefix)?;

        Ok(Signer {
            mnemonic: None,
            public_address,
            private_key,
            public_key,
        })
    }

    /// Creates a signer from a mnemonic phrase.
    ///
    /// # Arguments
    ///
    /// * `phrase` - The mnemonic phrase.
    /// * `prefix` - The prefix for the account ID.
    /// * `derivation` - The optional derivation path.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance of Signer or an error.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the mnemonic is invalid
    /// - the derivation path is invalid
    /// - the prefix is invalid
    pub fn from_mnemonic(
        phrase: &str,
        prefix: &str,
        derivation: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self> {
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(phrase, prefix, derivation, password)?;

        Ok(Signer {
            mnemonic: Some(phrase.to_string()),
            public_address,
            private_key,
            public_key,
        })
    }
}

/// Struct representing a Gevulot signer.
#[derive(Debug)]
pub struct GevulotSigner(pub Signer);

impl GevulotSigner {
    /// Creates a GevulotSigner from a mnemonic phrase.
    ///
    /// # Arguments
    ///
    /// * `mnemonic` - The mnemonic phrase.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance of GevulotSigner or an error.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the mnemonic is invalid
    /// - the derivation path is invalid
    /// - the prefix is invalid
    pub fn from_mnemonic(mnemonic: &str, password: Option<&str>) -> Result<Self> {
        let signer = Signer::from_mnemonic(mnemonic, "gvlt", None, password)?;
        Ok(GevulotSigner(signer))
    }

    /// Creates a GevulotSigner from entropy.
    ///
    /// # Arguments
    ///
    /// * `entropy` - A 32-byte array representing the entropy.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance of GevulotSigner or an error.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the entropy is invalid
    /// - the mnemonic is invalid
    /// - the derivation path is invalid
    /// - the prefix is invalid
    pub fn from_entropy(entropy: &[u8; 32], password: Option<&str>) -> Result<Self> {
        let mnemonic = bip32::Mnemonic::from_entropy(*entropy, bip32::Language::English);
        let signer = Signer::from_mnemonic(mnemonic.phrase(), "gvlt", None, password)?;
        Ok(GevulotSigner(signer))
    }

    /// Generates a random GevulotSigner.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance of GevulotSigner or an error.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// - the entropy is invalid
    /// - the mnemonic is invalid
    /// - the derivation path is invalid
    /// - the prefix is invalid
    pub fn random() -> Result<Self> {
        let entropy = [0u8; 32];
        let signer = Self::from_entropy(&entropy, None)?;
        Ok(signer)
    }

    /// Returns the public address of the signer.
    ///
    /// # Returns
    ///
    /// A reference to the AccountId representing the public address.
    pub fn address(&self) -> &AccountId {
        &self.0.public_address
    }
}

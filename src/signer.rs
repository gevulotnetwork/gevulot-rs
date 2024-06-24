use cosmrs::bip32::{Language, Mnemonic, XPrv};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::crypto::PublicKey;
use cosmrs::AccountId;
use hex::decode;
use rand_core::OsRng;

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
    fn load_from_mnemonic(
        phrase: &str,
        prefix: &str,
        derivation: Option<&str>,
    ) -> Result<(SigningKey, PublicKey, AccountId), Box<dyn std::error::Error>> {
        let derivation = if let Some(derivation) = derivation {
            derivation
        } else {
            "m/44'/118'/0'/0/0"
        };

        let mnemonic = Mnemonic::new(phrase, Language::English)?;
        let pri = XPrv::derive_from_path(&mnemonic.to_seed(""), &derivation.parse()?)?;
        let private_key = SigningKey::from(pri);
        let public_key = private_key.public_key();
        let public_address = public_key.account_id(prefix)?;

        Ok((private_key, public_key, public_address))
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - we cannot parse the derivation
    /// - if the prefix is bad
    pub fn generate(
        prefix: &str,
        derivation: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mnemonic = Mnemonic::random(OsRng, Language::English);
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(mnemonic.phrase(), prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(mnemonic.phrase().to_string()),
            public_address,
            private_key,
            public_key,
        })
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - the private key is invalid
    /// - we cannot parse the derivation
    /// - if the prefix is bad
    pub fn from_pkey(
        private_key: &str,
        prefix: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - mnemonic is invalid
    /// - we cannot parse the derivation
    /// - if the prefix is bad
    pub fn from_mnemonic(
        phrase: &str,
        prefix: &str,
        derivation: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(phrase, prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(phrase.to_string()),
            public_address,
            private_key,
            public_key,
        })
    }
}
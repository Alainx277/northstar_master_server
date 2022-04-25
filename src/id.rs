use std::fmt;

use thiserror::Error;

// TODO: Replace with UUID if okay for api
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct UniqueId([u8; 16]);

impl UniqueId {
    /// Creates a new unique identifier.
    /// The identifier is cryptographically secure if the provided [`rand::Rng`] is.
    pub fn new(mut rng: impl rand::Rng) -> Self {
        let mut id = [0u8; 16];
        rng.fill(&mut id);
        Self(id)
    }

    pub fn existing(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = [0u8; 32];
        hex::encode_to_slice(&self.0, &mut output).unwrap();
        f.write_str(std::str::from_utf8(&output).unwrap())?;
        Ok(())
    }
}

/// Possible errors when deserializing a [`UniqueId`].
#[derive(Error, Debug)]
enum Error {
    #[error("id is not hexadecimal")]
    NotHex,
    #[error("id is not 16 bytes long")]
    InvalidLength,
}

impl<'de> serde::Deserialize<'de> for UniqueId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UniqueIdVisitor)
    }
}

struct UniqueIdVisitor;

impl<'de> serde::de::Visitor<'de> for UniqueIdVisitor {
    type Value = UniqueId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("16 byte hexadecimal id")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut id = [0u8; 16];
        hex::decode_to_slice(v, &mut id)
            .map_err(|err| match err {
                hex::FromHexError::InvalidStringLength => Error::InvalidLength,
                _ => Error::NotHex,
            })
            .map_err(serde::de::Error::custom)?;

        Ok(UniqueId(id))
    }
}

impl serde::Serialize for UniqueId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut output = [0u8; 32];
        hex::encode_to_slice(&self.0, &mut output).unwrap();
        serializer.serialize_str(std::str::from_utf8(&output).unwrap())
    }
}

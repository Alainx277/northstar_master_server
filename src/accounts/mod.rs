use once_cell::sync::OnceCell;
pub use repository::AccountRepository;
pub use routes::{routes, with_accounts};
use serde::{Deserialize, Serialize};

mod handlers;
mod repository;
mod routes;

/// The unique identifier for an account
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AccountId(pub u64);

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// As sqlite doesn't explicitly support unsigned integers, we convert to and from an signed 64 bit integer
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for AccountId {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        (self.0 as i64).encode_by_ref(buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for AccountId {
    fn decode(
        value: <sqlx::Sqlite as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(i64::decode(value)? as u64))
    }
}

impl sqlx::Type<sqlx::Sqlite> for AccountId {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        i64::type_info()
    }
}

fn default_persistent_data() -> &'static [u8] {
    static INSTANCE: OnceCell<Vec<u8>> = OnceCell::new();
    INSTANCE
        .get_or_init(|| {
            std::fs::read("default.pdata")
                .expect("Unable to read default player data, is default.pdata missing?")
        })
        .as_slice()
}

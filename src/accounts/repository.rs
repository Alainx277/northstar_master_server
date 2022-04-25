use std::{borrow::Cow, net::IpAddr};

use crate::{id::UniqueId, Database};

#[derive(sqlx::FromRow)]
pub struct PersistenceAuthData {
    pub current_server: Option<UniqueId>,
    pub last_auth_ip: IpAddr,
}

pub struct AccountRepository {
    database: Database,
}

impl AccountRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn exists(&self, id: u32) -> Result<bool, sqlx::Error> {
        Ok(
            sqlx::query!(r#"SELECT 1 as none FROM accounts WHERE id = ?"#, id)
                .fetch_optional(&self.database)
                .await?
                .is_some(),
        )
    }

    pub async fn create(&self, id: u32) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO accounts (id) VALUES (?)"#, id)
            .execute(&self.database)
            .await?;
        Ok(())
    }

    pub async fn create_token(&self, id: u32, ip: IpAddr) -> Result<UniqueId, sqlx::Error> {
        let token = UniqueId::new(&mut rand::thread_rng());
        let raw_token = &token.bytes()[..];
        let ip = ip.to_string();
        let now = chrono::Utc::now();

        sqlx::query!(
            r#"UPDATE accounts SET token = ?, token_created = ?, last_auth_ip = ? WHERE id = ?"#,
            raw_token,
            now,
            ip,
            id
        )
        .execute(&self.database)
        .await?;
        Ok(token)
    }

    pub async fn authenticate(&self, id: u32, token: UniqueId) -> Result<bool, sqlx::Error> {
        let raw_token = &token.bytes()[..];
        let result = sqlx::query!(
            r#"SELECT token_created as "token_created!" FROM accounts WHERE id = ? AND token = ?"#,
            id,
            raw_token
        )
        .fetch_optional(&self.database)
        .await?;

        if let Some(entry) = result {
            let created: chrono::NaiveDateTime = entry.token_created;
            let age = chrono::Utc::now()
                .naive_utc()
                .signed_duration_since(created);
            // TODO: Move this into a constant
            if age < chrono::Duration::days(1) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub async fn get_name(&self, id: u32) -> Result<Option<String>, sqlx::Error> {
        Ok(
            sqlx::query!(r#"SELECT username FROM accounts WHERE id = ?"#, id)
                .fetch_one(&self.database)
                .await?
                .username
        )
    }

    pub async fn get_data(&self, id: u32) -> Result<Cow<'static, [u8]>, sqlx::Error> {
        Ok(
            sqlx::query!(r#"SELECT persistent_data FROM accounts WHERE id = ?"#, id)
                .fetch_one(&self.database)
                .await?
                .persistent_data
                .map_or_else(|| super::default_persistent_data().into(), |v| v.into()),
        )
    }

    pub async fn set_data(&self, id: u32, data: &[u8]) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE accounts SET persistent_data = ? WHERE id = ?"#,
            data,
            id
        )
        .execute(&self.database)
        .await?;
        Ok(())
    }

    pub async fn get_auth(&self, id: u32) -> Result<PersistenceAuthData, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT current_server, last_auth_ip as "last_auth_ip!" FROM accounts
            WHERE id = ? AND last_auth_ip IS NOT NULL"#,
            id
        )
        .fetch_one(&self.database)
        .await?;
        Ok(PersistenceAuthData {
            current_server: row
                .current_server
                .map(|d| UniqueId::existing(d.try_into().unwrap())),
            last_auth_ip: row.last_auth_ip.parse().unwrap(),
        })
    }

    pub async fn join_server(&self, id: u32, server_id: &UniqueId) -> Result<(), sqlx::Error> {
        let raw_server_id = &server_id.bytes()[..];
        sqlx::query!(
            r#"UPDATE accounts SET current_server = ? WHERE id = ?"#,
            raw_server_id,
            id
        )
        .execute(&self.database)
        .await?;
        Ok(())
    }
}

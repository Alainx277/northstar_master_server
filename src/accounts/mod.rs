use once_cell::sync::OnceCell;
pub use repository::AccountRepository;
pub use routes::{routes, with_accounts};

mod handlers;
mod repository;
mod routes;

fn default_persistent_data() -> &'static [u8] {
    static INSTANCE: OnceCell<Vec<u8>> = OnceCell::new();
    INSTANCE
        .get_or_init(|| {
            std::fs::read("default.pdata")
                .expect("Unable to read default player data, is default.pdata missing?")
        })
        .as_slice()
}

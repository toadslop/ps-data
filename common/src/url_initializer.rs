use url::Url;

use crate::keys::{
    DATABASE_HOST_KEY, DATABASE_NAME_KEY, DATABASE_PASSWORD_KEY, DATABASE_PORT_KEY,
    DATABASE_PROTOCOL_KEY, DATABASE_USER_KEY, DEFAULT_DB_URL,
};

pub struct UrlInitializer;

impl UrlInitializer {
    pub fn init_database_url() -> Url {
        let mut url = Url::parse(DEFAULT_DB_URL).expect("the default db url to be parseable");

        if let Ok(protocol) = std::env::var(DATABASE_PROTOCOL_KEY) {
            url.set_scheme(&protocol).unwrap();
        }

        if let Ok(protocol) = std::env::var(DATABASE_HOST_KEY) {
            url.set_host(Some(&protocol)).unwrap();
        }

        if let Ok(user) = std::env::var(DATABASE_USER_KEY) {
            url.set_username(&user).unwrap();
        }

        if let Ok(password) = std::env::var(DATABASE_PASSWORD_KEY) {
            url.set_password(Some(&password)).unwrap();
        }

        if let Ok(port) = std::env::var(DATABASE_PORT_KEY) {
            let port = port
                .parse::<u16>()
                .expect("the database port to parseable to u16");
            url.set_port(Some(port)).unwrap();
        };
        println!("{}", url);
        if let Ok(name) = std::env::var(DATABASE_NAME_KEY) {
            url.set_path(&name);
        }

        url
    }
}

pub mod proxies;

use crate::proxies::language::Language1Proxy;
use crate::proxies::software::Software1Proxy;
use crate::proxies::storage::{Actions1Proxy, Proposal1Proxy};
use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant;
use zbus::Error;

pub struct Client {
    pub connection: Connection,
}

impl Client {
    pub fn new() -> Result<Client, Error> {
        let connection = Connection::system()?;
        Ok(Client { connection })
    }

    pub fn get_storage(&self) -> StorageClient {
        StorageClient::new(&self.connection)
    }

    pub fn get_software(&self) -> SoftwareClient {
        SoftwareClient::new(&self.connection)
    }

    pub fn get_language(&self) -> LanguageClient {
        LanguageClient::new(&self.connection)
    }
}

#[derive(Debug)]
pub struct StorageDevice {
    pub name: String,
    pub description: String,
}

pub struct LanguageClient<'a> {
    pub connection: &'a Connection,
}

#[derive(Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
}

// TODO: What about keeping the proxy instead of the connection?
// TODO: Each client could keep its own connection, or share the same.
pub struct SoftwareClient<'a> {
    pub connection: &'a Connection,
}

impl<'a> SoftwareClient<'a> {
    pub fn new(connection: &'a Connection) -> SoftwareClient {
        SoftwareClient { connection }
    }

    pub fn products(&self) -> Result<Vec<Product>, Error> {
        let proxy = Software1Proxy::new(&self.connection)?;
        let base_products = proxy.available_base_products()?;

        let products = base_products
            .into_iter()
            .map(|(id, name, attrs)| {
                let description = match attrs.get("description") {
                    Some(value) => value.try_into().unwrap_or("Unknown"),
                    _ => "Unknown",
                };
                Product {
                    id,
                    name,
                    description: description.to_string(),
                }
            })
            .collect();
        Ok(products)
    }

    pub fn select_product(&self, product_id: &str) -> Result<(), Error> {
        let proxy = Software1Proxy::new(&self.connection)?;
        proxy.select_product(&product_id)
    }
}

// TODO: What about keeping the proxy instead of the connection?
// TODO: Each client could keep its own connection, or share the same.
pub struct StorageClient<'a> {
    pub connection: &'a Connection,
}

impl<'a> StorageClient<'a> {
    pub fn new(connection: &'a Connection) -> StorageClient {
        StorageClient { connection }
    }

    pub fn available_devices(&self) -> Result<Vec<StorageDevice>, Error> {
        let proxy = Proposal1Proxy::new(&self.connection)?;
        let available_devices = proxy.available_devices()?;

        let devices = available_devices
            .into_iter()
            .map(|(name, description, _)| StorageDevice { name, description })
            .collect();
        Ok(devices)
    }

    pub fn calculate(&self, devices: &Vec<String>) -> Result<u32, Error> {
        let proxy = Proposal1Proxy::new(&self.connection)?;
        let mut options = HashMap::new();
        options.insert("CandidateDevices", zvariant::Value::new(devices.clone()));
        proxy.calculate(options)
    }

    pub fn actions(&self) -> Result<Vec<HashMap<String, zvariant::OwnedValue>>, Error> {
        let proxy = Actions1Proxy::new(&self.connection)?;
        proxy.all()
    }
}

#[derive(Debug)]
pub struct Language {
    pub id: String,
    pub name: String,
}

impl<'a> LanguageClient<'a> {
    pub fn new(connection: &'a Connection) -> LanguageClient {
        LanguageClient { connection }
    }

    pub fn available_languages(&self) -> Result<Vec<Language>, Error> {
        let proxy = Language1Proxy::new(&self.connection)?;
        let available_languages = proxy.available_languages()?;

        let languages = available_languages
            .into_iter()
            .map(|(id, name, _)| Language { id, name })
            .collect();
        Ok(languages)
    }

    pub fn select(&self, lang_ids: Vec<&str>) -> Result<(), Error> {
        let proxy = Language1Proxy::new(&self.connection)?;
        proxy.to_install(&lang_ids)
    }

    pub fn selected(&self) -> Result<Vec<Language>, Error> {
        let proxy = Language1Proxy::new(&self.connection)?;
        let selected_languages = proxy.marked_for_install()?;
        let available_languages = self.available_languages()?;

        let languages = available_languages
            .into_iter()
            .filter(|l| selected_languages.contains(&l.id))
            .collect();
        Ok(languages)
    }
}

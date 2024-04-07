#![allow(dead_code)]

use std::io;
use serde::Deserialize;
use piglog::prelude::*;
use piglog::*;

use crate::library::{ self, * };

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct ManagerConfig {
    pub many_args: bool,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            many_args: true,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct Manager {
    pub add: String,
    pub remove: String,
    pub sync: String,
    pub upgrade: String,
    pub config: ManagerConfig,
    pub plural_name: String,
}

impl Manager {
    pub fn add(&self, items: &[String]) -> Result<(), io::Error> {
        let many = self.config.many_args;

        crate::hook::run_hook_and_return_if_err!(format!("pre_{}_add", self.plural_name));

        if many {
            self.add_raw(&items.join(" "))?;
        }

        else {
            for i in items {
                self.add_raw(i)?;
            }
        }

        crate::hook::run_hook_and_return_if_err!(format!("post_{}_add", self.plural_name));

        Ok(())
    }

    pub fn remove(&self, items: &[String]) -> Result<(), io::Error> {
        let many = self.config.many_args;

        crate::hook::run_hook_and_return_if_err!(format!("pre_{}_remove", self.plural_name));

        if many {
            self.remove_raw(&items.join(" "))?;
        }

        else {
            for i in items {
                self.remove_raw(i)?;
            }
        }

        crate::hook::run_hook_and_return_if_err!(format!("post_{}_remove", self.plural_name));

        Ok(())
    }

    fn add_raw(&self, items: &str) -> Result<(), io::Error> {
        if items.trim() == "" {
            return Ok(());
        }

        match run_command(sed(self.add.as_str(), "#:?", items).as_str()) {
            true => info!("Successfully added {}!", self.plural_name),
            false => {
                error!("Failed to add {}!", self.plural_name);

                return Err(custom_error(format!("Failed to add {}!", self.plural_name).as_str()));
            },
        };

        Ok(())
    }

    fn remove_raw(&self, items: &str) -> Result<(), io::Error> {
        if items.trim() == "" {
            return Ok(());
        }

        match run_command(sed(self.remove.as_str(), "#:?", items).as_str()) {
            true => info!("Successfully removed {}!", self.plural_name),
            false => {
                error!("Failed to remove {}!", self.plural_name);

                return Err(custom_error(format!("Failed to remove {}!", self.plural_name).as_str()));
            },
        };

        Ok(())
    }

    pub fn sync(&self) -> Result<(), io::Error> {
        crate::hook::run_hook_and_return_if_err!(format!("pre_{}_sync", self.plural_name));

        match run_command(self.sync.as_str()) {
            true => info!("Successfully installed repositories!"),
            false => {
                error!("Failed to sync repositories!");

                return Err(custom_error("Failed to sync repositories!"));
            },
        };

        crate::hook::run_hook_and_return_if_err!(format!("post_{}_sync", self.plural_name));

        Ok(())
    }

    pub fn upgrade(&self) -> Result<(), io::Error> {
        crate::hook::run_hook_and_return_if_err!(format!("pre_{}_upgrade", self.plural_name));

        match run_command(self.upgrade.as_str()) {
            true => info!("Successfully upgraded {}!", self.plural_name),
            false => {
                error!("Failed to upgrade {}!", self.plural_name);

                return Err(custom_error(format!("Failed to upgrade {}!", self.plural_name).as_str()));
            },
        };

        crate::hook::run_hook_and_return_if_err!(format!("post_{}_upgrade", self.plural_name));

        Ok(())
    }

    pub fn set_plural_name(&mut self, pn: &str) {
        self.plural_name = pn.to_string();
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            add: String::new(),
            remove: String::new(),
            sync: String::new(),
            upgrade: String::new(),
            config: ManagerConfig::default(),
            plural_name: String::from("ITEMS FROM UNKNOWN MANAGER"),
        }
    }
}

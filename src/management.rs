#![allow(dead_code)]

use std::io;
use serde::Deserialize;
use piglog::prelude::*;
use piglog::*;
use fspp::*;

use crate::library::{ self, * };
use crate::places;

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
            true => info!("Synced manager successfully! ('{}')", self.plural_name),
            false => {
                error!("Failed to sync manager! ('{}')", self.plural_name);

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

pub fn load_manager(man: &str) -> Result<Manager, io::Error> {
    let path = places::base_user().add_str(&format!("managers/{man}.toml"));

    let man_string = match file::read(&path) {
        Ok(o) => o,
        Err(e) => {
            piglog::fatal!("Failed to read manager file! ({man})");
            piglog::note!("If this error shows up, it is possible the file is missing. ({})", path.to_string());

            return Err(e);
        },
    };

    let manager: Manager = match toml::from_str(&man_string) {
        Ok(o) => o,
        Err(e) => {
            piglog::fatal!("Failed to deserialize manager! ({man})");
            piglog::fatal!("Error: {e:#?}");

            return Err(io::Error::new(io::ErrorKind::Other, "Failed to deserialize manager!"));
        },
    };

    Ok(manager)
}

pub fn managers() -> Result<Vec<String>, io::Error> {
    let path = places::base_user().add_str("managers");

    let man_list: Vec<String> = directory::list_items(&path)?
        .into_iter()
        .map(|x| x.basename().replace(".toml", ""))
        .collect();

    Ok(man_list)
}

pub fn sync_all() -> Result<(), io::Error> {
    let m_all = managers()?;
    let m_len = m_all.len();

    info!("Syncing {} managers...", m_len);

    for m in m_all {
        let man = load_manager(&m)?;

        man.sync()?;
    }

    success!("All {} managers synced successfully!", m_len);

    Ok(())
}

pub fn upgrade_all(sync_before_upgrade: bool) -> Result<(), io::Error> {
    if sync_before_upgrade {
        sync_all()?;
    }

    let m_all = managers()?;
    let m_len = m_all.len();

    info!("Upgrading {} managers...", m_len);

    for m in m_all {
        let man = load_manager(&m)?;

        man.upgrade()?;
    }

    success!("All {} managers upgraded successfully!", m_len);

    Ok(())
}

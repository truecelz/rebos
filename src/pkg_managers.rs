#![allow(dead_code)]

pub mod flatpak {
    use crate::pkg_managers_boilerplate::macros::*;

    pkg_manager_boilerplate!();

    enable_mode::install!();
    enable_mode::uninstall!();
    enable_mode::upgrade!();

    fn get_manager() -> Result<PackageManager, io::Error> {
        return Ok(PackageManager {
            install: String::from("flatpak install #:?"),
            remove: String::from("flatpak uninstall #:?"),
            upgrade: String::from("flatpak upgrade"),
            plural_name: String::from("flatpaks"),
            ..PackageManager::default()
        });
    }
}

pub mod cargo {
    use crate::pkg_managers_boilerplate::macros::*;

    pkg_manager_boilerplate!();

    enable_mode::install!();
    enable_mode::uninstall!();

    fn get_manager() -> Result<PackageManager, io::Error> {
        return Ok(PackageManager {
            install: String::from("cargo install #:?"),
            remove: String::from("cargo uninstall #:?"),
            plural_name: String::from("crates"),
            ..PackageManager::default()
        });
    }
}

pub mod pkg_manager {
    use fspp::*;

    use crate::pkg_managers_boilerplate::macros::*;
    use crate::error;
    use crate::library::*;
    use crate::places;

    pkg_manager_boilerplate!();

    enable_mode::install!();
    enable_mode::uninstall!();
    enable_mode::upgrade!();
    enable_mode::sync!();

    fn get_manager() -> Result<PackageManager, io::Error> {
        let mut package_manager: PackageManager = match toml::from_str(match file::read(&places::base_user().add_str("pkg_manager.toml")) {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to read package manager TOML file!");
                return Err(e);
            },
        }.as_str()) {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to parse package manager TOML file!");
                error!("TOML Error: {:?}", e);
    
                return Err(custom_error("Failed to parse TOML file!"));
            },
        };
    
        package_manager.set_plural_name("packages");
    
        return Ok(package_manager);
    }
}

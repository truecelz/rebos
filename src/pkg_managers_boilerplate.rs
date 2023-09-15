#![allow(dead_code)]

pub mod macros {
    macro_rules! pkg_manager_boilerplate {
        () => {
            use std::io;
            use crate::convert::*;
            use crate::package_management::PackageManager;
    
            enum ManageMode {
                Install(Vec<String>),
                Remove(Vec<String>),
                Upgrade,
                Sync,
            }
            
            fn pkg_manage(mode: ManageMode) -> Result<(), io::Error> {
                #[allow(unreachable_patterns)]
                match mode {
                    ManageMode::Install(ref p) => {
                        if p.len() == 0 {
                            return Ok(());
                        }
            
                        if p[0].trim() == "" {
                            return Ok(());
                        }
                    },
                    ManageMode::Remove(ref p) => {
                        if p.len() == 0 {
                            return Ok(());
                        }
            
                        if p[0].trim() == "" {
                            return Ok(());
                        }
                    },
                    _ => {}, // If the mode does not have a package list, don't do the ZERO check.
                };
            
                let manager: PackageManager = match get_manager() {
                    Ok(o) => o,
                    Err(e) => return Err(e),
                };
            
                match mode {
                    ManageMode::Install(pkgs) => {
                        let pkgs_string = string_vec_to_string(&pkgs, " ");
            
                        match manager.install(pkgs_string.as_str()) {
                            Ok(_o) => {},
                            Err(e) => return Err(e),
                        };
                    },
                    ManageMode::Remove(pkgs) => {
                        let pkgs_string = string_vec_to_string(&pkgs, " ");
            
                        match manager.remove(pkgs_string.as_str()) {
                            Ok(_o) => {},
                            Err(e) => return Err(e),
                        };
                    },
                    ManageMode::Upgrade => match manager.upgrade() {
                        Ok(_o) => {},
                        Err(e) => return Err(e),
                    },
                    ManageMode::Sync => match manager.sync() {
                        Ok(_o) => {},
                        Err(e) => return Err(e),
                    },
                };
            
                return Ok(());
            }
        };
    }

    pub mod enable_mode {
        #![allow(unused_macros)]

        macro_rules! install {
            () => {
                pub fn install(pkgs: &Vec<String>) -> Result<(), io::Error> {
                    return pkg_manage(ManageMode::Install(pkgs.clone()));
                }
            };
        }

        macro_rules! uninstall {
            () => {
                pub fn uninstall(pkgs: &Vec<String>) -> Result<(), io::Error> {
                    return pkg_manage(ManageMode::Remove(pkgs.clone()));
                }
            };
        }

        macro_rules! upgrade {
            () => {
                pub fn upgrade() -> Result<(), io::Error> {
                    return pkg_manage(ManageMode::Upgrade);
                }
            };
        }

        macro_rules! sync {
            () => {
                pub fn sync() -> Result<(), io::Error> {
                    return pkg_manage(ManageMode::Sync);
                }
            };
        }

        pub(crate) use install;
        pub(crate) use uninstall;
        pub(crate) use upgrade;
        pub(crate) use sync;
    }

    pub(crate) use pkg_manager_boilerplate;
}

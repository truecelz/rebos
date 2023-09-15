#![allow(dead_code)]

pub mod macros {
    macro_rules! build_package_manager {
        (
            $built_field_name: expr,
            $curr_field_name: expr,
            $package_manager: ident,
            $summary_name: expr,
            $summary_entries: ident
        ) => {
            let built_string = string_vec_to_string(&$built_field_name, "\n");
            let curr_string = string_vec_to_string(&$curr_field_name, "\n");

            let diffs = history(&built_string, &curr_string);

            let mut to_install: Vec<String> = Vec::new();
            let mut to_remove: Vec<String> = Vec::new();

            for i in diffs.iter() {
                match i.mode {
                    HistoryMode::Add => to_install.push(i.line.to_string()),
                    HistoryMode::Remove => to_remove.push(i.line.to_string()),
                };
            }

            match $package_manager::install(&to_install) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            match $package_manager::uninstall(&to_remove) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };

            $summary_entries.push(($summary_name, diffs));
        };
    }

    macro_rules! build_package_manager_first_time {
        (
            $package_manager: ident,
            $curr_field_name: expr
        ) => {
            match $package_manager::install(&$curr_field_name) {
                Ok(_o) => {},
                Err(e) => return Err(e),
            };
        };
    }

    pub(crate) use build_package_manager;
    pub(crate) use build_package_manager_first_time;
}

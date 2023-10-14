#![allow(unused_imports)]

macro_rules! run_hook_and_return_if_err {
    (
        $hook_name: expr
    ) => {
        if crate::filesystem::path_exists(format!("{}/hooks/{}", crate::places::base_user(), $hook_name).as_str()) == true {
            crate::info!("Running hook: {}", $hook_name);

            match library::run_command(format!("{}/hooks/{}", crate::places::base_user(), $hook_name).as_str()) {
                true => crate::info!("Successfully ran hook: {}", $hook_name),
                false => {
                    crate::error!("Failed to run hook: {}", $hook_name);

                    return Err(library::custom_error("Failed to run hook!"));
                },
            };
        }
    }
}

pub(crate) use run_hook_and_return_if_err;

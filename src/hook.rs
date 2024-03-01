#![allow(unused_imports)]

macro_rules! run_hook_and_return_if_err {
    (
        $hook_name: expr
    ) => {
        if crate::places::base_user().add_str("hooks").add_str(&format!("{}", $hook_name)).exists() {
            crate::info!("Running hook: {}", $hook_name);

            match library::run_command(&crate::places::base_user().add_str("hooks").add_str(&format!("{}", $hook_name).replace(" ", "_")).to_string()) {
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

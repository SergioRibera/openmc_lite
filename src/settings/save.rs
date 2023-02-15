use std::fs::File;

use preferences::security::{SecurePreferences, SecurityManager};

use crate::data::{config_path, OPENMC_SECURE_KEY};

pub fn save_settings<D: SecurePreferences>(data: D, file_name: &str) -> bool {
    let manager = SecurityManager::new(OPENMC_SECURE_KEY, None);
    let mut conf_dir = config_path("");
    conf_dir.push(file_name);
    let mut file_conf = File::create(conf_dir).unwrap();
    data.save_to(&manager, &mut file_conf).is_ok()
}

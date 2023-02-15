use std::fs::File;

use preferences::security::{SecurePreferences, SecurityManager};

use crate::data::{config_path, OPENMC_SECURE_KEY};

pub fn load_settings<A: SecurePreferences + Default>(file_name: &str) -> A {
    let manager = SecurityManager::new(OPENMC_SECURE_KEY, None);
    let mut conf_dir = config_path("");
    conf_dir.push(file_name);
    if conf_dir.exists() {
        let mut file_conf = File::open(conf_dir).unwrap();
        A::load_from(&manager, &mut file_conf).unwrap()
    } else {
        A::default()
    }
}

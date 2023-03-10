use mc_downloader::prelude::{DownloadData, DownloaderService};

use crate::data::config_path;

const ICONS: [&str; 29] = [
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Amethyst_Shard_JE2_BE1.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Apple_JE3_BE3.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Armor_Stand_(item)_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Arrow_Loaded_Crossbow.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Bamboo_(item)_JE1_BE1.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Barrier_(item)_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Beetroot_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Bell_(item)_JE1_BE1.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Blaze_Powder_JE2_BE1.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Blaze_Rod_JE1_BE1.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Book_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Bottle_o'_Enchanting_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Bread_JE3_BE3.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Brick_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Cake_(item)_JE3_BE3.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Campfire_(item)_JE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Compass_JE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Cooked_Chicken_JE3_BE3.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Cooked_Mutton_JE3_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Cookie_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Dark_Oak_Boat_(item)_JE2_BE3.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Egg_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Elytra_(item)_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Emerald_JE3_BE3.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Enchanted_Book_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/End_Crystal_(item)_JE2_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Ender_Pearl_JE3_BE2.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Firework_Loaded_Crossbow.png",
    "https://raw.githubusercontent.com/SergioRibera/openmc_lite/151f7ca9e2d7e56545e69b47c2a8b39b1ac82d82/assets/res/Golden_Apple_JE2_BE2.png",
];

pub fn create_downloads_icons() -> Vec<DownloadData> {
    let binding = config_path("icons");

    ICONS
        .iter()
        .map(|d| {
            let file_name = d.split('/').last().unwrap();
            let mut path = binding.clone();
            path.push(file_name);
            let path = path.to_str().unwrap();
            DownloadData::new(d, path)
        })
        .collect()
}

pub fn create_icons_svc() -> DownloaderService {
    let binding = config_path("icons");
    let path = binding.to_str().unwrap();

    DownloaderService::new(path)
        .with_downloads(create_downloads_icons())
        .to_owned()
}

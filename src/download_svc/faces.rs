use mc_downloader::prelude::{DownloadData, DownloaderService};

use crate::data::config_path;

const FACES: [&str; 46] = [
    "https://minecraftfaces.com/wp-content/bigfaces/big-allay-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-axolotl-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-blaze-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-cave-spider-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-chicken-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-cow-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-creeper-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-dolphin-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-drowned-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-elder-guardian-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-enderman-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-endermite-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-evoker-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-frog-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-ghast-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-glow-squid-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-guardian-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-husk-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-killer-rabbit-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-llama-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-magma-cube-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-mooshroom-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-panda-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-phantom-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-pig-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-polar-bear-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-rabbit-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-ravager-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-sheep-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-shulker-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-skeleton-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-slime-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-snowgolem-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-spider-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-steve-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-stray-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-tadpole-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-turtle-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-vex-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-villager-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-vindicator-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-warden-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-wither-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-wither-skeleton-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-zombie-face.png",
    "https://minecraftfaces.com/wp-content/bigfaces/big-zombie-pigman-face.png",
];

pub fn create_faces_downloads() -> Vec<DownloadData> {
    let binding = config_path("faces");

    FACES
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

pub fn create_faces_svc() -> DownloaderService {
    let binding = config_path("faces");
    let path = binding.to_str().unwrap();

    DownloaderService::new(path)
        .with_downloads(create_faces_downloads())
        .to_owned()
}

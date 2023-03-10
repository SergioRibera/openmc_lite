use mc_downloader::prelude::{DownloadData, DownloaderService};

use crate::data::config_path;

pub fn create_downloads_icons() -> Vec<DownloadData> {
    let mut binding = config_path("icons");
    binding.push("icons.zip");
    let path = binding.to_str().unwrap();

    vec![
DownloadData::new("https://github.com/sammwyy/OpenMC/raw/8e7adf492d18347d2cd117157db55260939416a2/assets/default_icons.zip", path)
    ]
}

pub fn create_icons_svc() -> DownloaderService {
    let mut binding = config_path("icons");
    binding.push("icons.zip");
    let path = binding.to_str().unwrap();

    DownloaderService::new(path)
        .with_downloads(create_downloads_icons())
        .to_owned()
}

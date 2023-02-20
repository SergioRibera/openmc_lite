use mc_downloader::prelude::{DownloadData, DownloaderService, Progress};

use crate::data::config_path;

pub fn create_icons_svc() -> DownloaderService {
    let mut binding = config_path("icons");
    binding.push("icons.zip");
    let path = binding.to_str().unwrap();

    let downloads = vec![
DownloadData::new("https://github.com/sammwyy/OpenMC/raw/8e7adf492d18347d2cd117157db55260939416a2/assets/default_icons.zip", path)
    ];
    DownloaderService::new(path)
        .with_downloads(downloads)
        .to_owned()
}

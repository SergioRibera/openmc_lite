mod faces;
mod icons;

use std::sync::mpsc::{Receiver, SyncSender};

pub use faces::*;
pub use icons::*;
use log::info;
use mc_downloader::prelude::{DownloaderService, Reporter};

use crate::data::config_path;

pub fn download_extra_resources() -> DownloaderService {
    let cfg_path = config_path("");
    let icons = create_downloads_icons();
    let faces = create_faces_downloads();

    let downloads = [icons, faces].concat();

    info!("Files to Download: {}", downloads.len());

    DownloaderService::new(cfg_path.to_str().unwrap())
        .with_downloads(downloads)
        .with_parallel_requests(50)
        .to_owned()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownloadProgressMessage {
    Setup(u64),
    Update(u64, u64),
    End,
}

#[derive(Clone)]
pub struct DownloadProgress {
    curr_progress: u64,
    max_progress: u64,
    sender: SyncSender<DownloadProgressMessage>,
}

impl DownloadProgress {
    pub fn new() -> (Self, Receiver<DownloadProgressMessage>) {
        let (sender, recv) = std::sync::mpsc::sync_channel::<DownloadProgressMessage>(1);
        (
            Self {
                curr_progress: 0,
                max_progress: 0,
                sender,
            },
            recv,
        )
    }
}

impl Reporter for DownloadProgress {
    fn setup(&mut self, max_progress: u64) {
        info!("Setup Reporter: {max_progress}");
        self.max_progress = max_progress;
        self.sender
            .send(DownloadProgressMessage::Setup(max_progress))
            .unwrap();
    }

    fn progress(&mut self, current: u64) {
        self.curr_progress += current;
        info!(
            "Setup progress\nIncoming: {current}\nCurrent: {}\nMax: {}",
            self.curr_progress, self.max_progress
        );
        if current > 0 {
            self.sender
                .send(DownloadProgressMessage::Update(
                    self.curr_progress,
                    self.max_progress,
                ))
                .unwrap();
        }
    }

    fn done(&mut self) {
        info!("Done progress");
        self.sender.send(DownloadProgressMessage::End).unwrap();
    }
}

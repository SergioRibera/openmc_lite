mod icons;

use std::sync::mpsc::{Receiver, SyncSender};

pub use icons::*;
use log::info;
use mc_downloader::prelude::Reporter;

#[derive(Clone, Copy, PartialEq, Eq)]
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
        info!(
            "Setup progress\nCurr: {current}\nMax: {}",
            self.max_progress
        );
        if current > 0 {
            self.curr_progress += current;
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

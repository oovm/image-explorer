use ie_windows::utils::enable_thumbnails;
use crate::IEWorkspace;

impl IEWorkspace {
    pub fn run(&self) {
        enable_thumbnails().unwrap()
    }
}
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::i32;
use std::convert::{From, TryFrom};
use std::marker::PhantomData;

use chrono::{DateTime, Utc};

use threadpool::{ThreadPool, Builder};

use crate::download_handler::DownloadHandle;


pub struct Handle;

#[derive(Copy, Clone)]
pub enum DownloadStatus{
    
}

impl From<u8> for DownloadStatus {
    fn from(status_number: u8) -> DownloadStatus {
        unimplemented!();
    }
}

impl Into<u8> for DownloadStatus {
    fn into(self) -> u8{
        unimplemented!();
    }
}

pub struct Download<'a> {
    pub status: DownloadStatus,
    pub download_size: u64, //Bytes
    pub current_download_size: u64,
    pub time_start: DateTime<Utc>,
    pub time_finished: DateTime<Utc>,
    pub time_stamp: DateTime<Utc>,
    pub url: String,
    pub download_path: String,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Download<'a> {
    pub fn new(url: String, path: String) -> Self{
        
        unimplemented!();
    }

    pub fn to_bytes(&self) -> &[u8] {
        let number: i32 = 1234;

        let bytes = number as u8 as char;
        unimplemented!();
    }
}

pub struct Scheduler<'a> {
    download_queue: VecDeque<Download<'a>>,
    thread_pool : ThreadPool,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Scheduler<'a> {
    pub fn new(num_threads: usize) -> Self {
        unimplemented!();
    }

    pub fn schedule(&mut self, download: Weak<Download>) -> Handle {
        unimplemented!();
    }

}

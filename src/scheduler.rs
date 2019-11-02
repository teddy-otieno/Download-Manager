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

    pub fn to_generate_bytes(&self) -> Vec<u8> {

        let download: &Download = self;

        let mut download_bytes: Vec<u8> = Vec::new();
        
        //The first byte is the status
        download_bytes.push(download.status.into());

        //The next 8 bytes
        download_bytes.extend_from_slice(&download.download_size.to_ne_bytes());

        //The next 8bytes will be the current download size;
        download_bytes.extend_from_slice(&download.current_download_size.to_ne_bytes());

        //The next 8bytes will be the start of the download
        download_bytes.extend_from_slice(&download.time_start.timestamp().to_ne_bytes());

        //The next 8 bytes will store the time finished
        download_bytes.extend_from_slice(&download.time_finished.timestamp().to_ne_bytes());

        //The next 512 bytes will be to store the download ur
        download_bytes.extend_from_slice(&download.url.as_bytes());

        //The next 512 bytes will be to store the download_path
        download_bytes.extend_from_slice(&download.download_path.as_bytes());


        download_bytes
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

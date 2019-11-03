use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::i32;
use std::convert::{From, TryFrom};
use std::marker::PhantomData;

use chrono::{DateTime, Utc};

use threadpool::{ThreadPool, Builder};

use crate::download_handler::DownloadHandle;


#[derive(Debug, )]
pub enum SchedulerError<'a>{
    StringLengthTooLarge(&'a str),
}

pub struct Handle;

#[derive(Copy, Clone)]
pub enum DownloadStatus{
    Queued,    
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

fn convert_char_slice_to_u8(char_slice: &Vec<char>) -> Vec<u8> {
    /*
    * 1 char = 4 bytes
    * The total size = 512 * 4 = 2048
    */
    let converted_bytes : Vec<u8> = Vec::with_capacity(2048);

    for (index, element) in char_slice.into_iter().enumerate() {
        let char_bytes: [u8; 4] = [0; 4];
        element.encode_utf8(&mut char_bytes);

        for byte in char_bytes.into_iter() {
            converted_bytes.push(*byte);
        }
    }

    return converted_bytes

}

pub struct Download<'a> {
    pub status: DownloadStatus,
    pub download_size: Option<u64>, //Bytes
    pub current_download_size: Option<u64>,
    pub time_start: Option<DateTime<Utc>>,
    pub time_finished: Option<DateTime<Utc>>,
    pub time_stamp: DateTime<Utc>, 
    pub url: Vec<char>, // 2048 bytes
    pub download_path: Vec<char>, //2048 bytes
    phantom: PhantomData<&'a ()>,
}

impl<'a> Download<'a> {
    pub fn new(url: String, path: String) -> Result<Self, SchedulerError<'a>>{
        
        //Making sure the string doesnt go beyond 512 chars
        if url.len() > 512 || path.len() > 512 {
            return Err(SchedulerError::StringLengthTooLarge("The url or path len is too large"));
        }

        Ok(
            Self {
                status: DownloadStatus::Queued,
                //TODO:
                download_size: None,
                current_download_size: None,
                time_start: None,
                time_finished: None,
                time_stamp: Utc::now(),
                url: url.chars().collect(),
                download_path: path.chars().collect(),
                phantom: PhantomData,
            }
        )
    }

    pub fn to_generate_bytes(&self) -> Vec<u8> {

        let download: &Download = self;

        let mut download_bytes: Vec<u8> = Vec::new();
        
        //The first byte is the status
        download_bytes.push(download.status.into());

        //The next 8 bytes
        download_bytes.extend_from_slice(match download.download_size {
            Some(size) => &size.to_ne_bytes(),
            None => &[0u8; 8]
            }
        );

        //The next 8bytes will be the current download size; TODO:
        download_bytes.extend_from_slice(match download.current_download_size {
            Some(size) => &size.to_ne_bytes(),
            None => &[0u8; 8]
            }
        );

        //The next 8bytes will be the start of the download
        download_bytes.extend_from_slice( match download.time_start {
            Some(time) => &time.timestamp().to_ne_bytes(),
            None => &[0u8; 8],
            }
        );

        //The next 8 bytes will store the time finished
        download_bytes.extend_from_slice( match download.time_finished {
            Some(time) => &time.timestamp().to_ne_bytes(),
            None => &[0u8; 8]
        });

        //The next 2048 bytes
        download_bytes.extend(convert_char_slice_to_u8(&download.url));

        //The next 2048 bytes
        download_bytes.extend(convert_char_slice_to_u8(&download.download_path));


        //Total number of bytes 4129
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


#[cfg(test)]
mod tests {

    use super::Download;    

    #[test]
    fn test_char_to_bytes_conversion() {

        let test_download: Download = Download::new(String::from("foo"), String::from("bar")).unwrap();

        assert_eq!(test_download.to_generate_bytes().len(), 4129);
    }
}
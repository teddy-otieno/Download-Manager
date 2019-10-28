use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::rc::{Rc, Weak};
use std::marker::PhantomData;

use crate::scheduler::Scheduler;
use crate::scheduler::Download;
use crate::scheduler::Handle;

const database_path : &'static str = "~/Desktop/download_records";

//Store the download records to the filesystem

struct Database<'a>{
    download_records: Vec<Rc<Download<'a>>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Database<'a> {
    fn new() -> Self {
       //Load the file and read its contents
        let file: File = match File::open(database_path) {
            Ok(file) => file,
            Err(_) => {
                //Incase the file does not exist,
                //Just create a file and return
                File::create(database_path).ok().unwrap()
            }
        };

        let mut buf_reader: BufReader<File> = BufReader::new(file);
        
        Self::parse_file(buf_reader.buffer());
        unimplemented!();
    }

    fn add_record(&mut self,download: Rc<Download<'a>>) {
        self.download_records.push(download);
        //TODO: Write the record to the database

        /*
         * Binary File format
         * magic number
         * 
         * entry table  - Hold the pointers to chunks in memory
         *
         *
         * First data
         *
         */

        let download_record_bytes: Vec<u8> = Self::convert_download_to_bytes(self.download_records.get(self.download_records.len()).unwrap());

        let file: File = OpenOptions::new()
            .write(true)
            .open(database_path)
            .unwrap();

    }

    fn convert_download_to_bytes(download_ref: &Rc<Download>) -> Vec<u8> {
        let download: &Download = &*(*download_ref);

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

        download_bytes.extend_from_slice(&download.time_finished.timestamp().to_ne_bytes());

        //The next 512 bytes will be to store the download ur
        download_bytes.extend_from_slice(&download.url.as_bytes());

        //The next 512 bytes will be to store the download_path
        download_bytes.extend_from_slice(&download.download_path.as_bytes());


        download_bytes
    }
    
    fn parse_file(buffer: &[u8]) -> Vec<Download>{

        unimplemented!();
    }
}

pub struct DownloadHandle<'a> {
    context: Weak<Download<'a>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> DownloadHandle<'a> {
    pub fn new(download_ref: &Rc<Download>) -> Self{
        unimplemented!();
    }
}

pub struct DownloadHandler<'a>{
    scheduler: Scheduler<'a>,
    downloads: Vec<Weak<Download<'a>>>,
    database: Database<'a>,
}

impl<'a> DownloadHandler<'a>{
    pub fn new() -> Self{

        unimplemented!();
    }

    pub fn add_download(&mut self, url: String, filesystem_path: String) -> Result<Handle, String> {
        let download: Rc<Download> = Rc::new(Download::new(url, filesystem_path));

        self.downloads.push(Rc::downgrade(&download));
        self.database.add_record(download);

        Ok(self.scheduler.schedule(Weak::clone(self.downloads.get(self.downloads.len()).unwrap()))) //Get the last element added
    }
}

struct DonwloadServer<'a>{
    //The scheduler will add the jobs to a job que.
    //Downloads will only be threads at a time.
    scheduler: Scheduler<'a>,
    phantom: PhantomData<&'a ()>
}

impl<'a> DonwloadServer<'a> {
    fn new() -> Self {
        unimplemented!();
    }

    fn add_job(url: String, path: String) {
       unimplemented!(); 
    }
}

use std::fs::{File, OpenOptions};
use std::io::{BufReader, SeekFrom};
use std::io::prelude::*;
use std::rc::{Rc, Weak};
use std::mem::size_of;
use std::marker::PhantomData;
use std::u64;

use crate::scheduler::Scheduler;
use crate::scheduler::Download;
use crate::scheduler::Handle;

const database_path : &'static str = "~/Desktop/download_records";

//Store the download records to the filesystem


/*
 *
 * Database 
 * Table
 *  8 - number of entries
 *  8 - starting position_of_first entry
 *   -
 *   -
 *   - entries
 *   -
 *   - upto the first actual entry
 */

enum DatabaseError<'a> {
    DbOpen(&'a str),
    DbInternal(&'a str),
    DbNotification(&'a str),
}


struct DownloadEntries {
    //Maximum string length
    download_url: String,
    filesystem_download_path: String,
    //The number of bytes from the last_table_location_to to where it was appended,
    file_system_entry_position: u64,
}

impl DownloadEntries {
    fn new(download_url: String, filesystem_download_path: String, file_system_location: u64) -> Self {
        Self {
            download_url: download_url,
            filesystem_download_path: filesystem_download_path,
            file_system_entry_position: file_system_location,
        }
    }

    fn from_bytes(entry_bytes: &[u8]) {
        unimplemented!();
    }
}

//Download_url + position on disk(usize 8bytes) * 100_000
//Looks a little too big
//Just incase we need to do some compression
const MAXIMUM_ENTRIES : u64 = 10_000;
const MAXIMUM_TABLE_SIZE : u64 = (512 + 8) * MAXIMUM_ENTRIES;

struct DbTable<'a>{
    //Initial pointer of the table
    //
    //The first_bytes_will contain the position last element in the table
    //The total number of entries in the database
    table_entries: Vec<DownloadEntries>,
    db_file: &'a str,
    position_to_the_first_entry: u64,
}

impl<'a> DbTable<'a> {
    fn new(file_path: &'a str ) -> Self {
        Self {
            table_entries: Vec::new(),
            db_file: file_path,
            position_to_the_first_entry: 0,
        }
    }

    fn create_table(&self) -> Result<(), DatabaseError> {
        //set the total number of entries
        //set the position to the first_element
        //Position to the last element
        let mut db_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(self.db_file).unwrap();

        let new_seek_cursor_position: u64 = match db_file.seek(SeekFrom::Start(0)) {
            Ok(position) => position,
            Err(_) => return Err(DatabaseError::DbInternal("Failed to set file position")),
        };
        
        let number_of_entries_bytes: [u8; size_of::<u64>()] = 0u64.to_ne_bytes();

        db_file.write(&number_of_entries_bytes).unwrap();

        //Delete the current file object and create a new one in append mode
        drop(db_file);

        db_file = OpenOptions::new()
                    .append(true)
                    .open(self.db_file).unwrap();
        
        let total_table: Vec<u8> = vec![0; MAXIMUM_TABLE_SIZE as usize];

        let cursor: u64 = match db_file.seek(SeekFrom::Start(8)) {
            Ok(position) => position,
            Err(_) => return Err(DatabaseError::DbInternal("Failed to set file position"))
        };

        //Managed to create_table
        db_file.write(&total_table).unwrap();

        Ok(())
    }

    fn load_entries(&mut self) ->Result<(), DatabaseError> {
        //Check position of the last element u64
        //Get the total number of entries
        //I'll Loop through the entries
        //Tests are working okay
        //
        let mut db_file: File = OpenOptions::new()
            .read(true)
            .open(self.db_file).unwrap();

        let position: u64 = match db_file.seek(SeekFrom::Start(0)){
            Ok(position) => position,
            Err(_) => return Err(DatabaseError::DbInternal("Failed to set file position")),
        };

        if position == 0 {
            panic!("Error while reading the table entries");
            //return Err(DatabaseError::DbInternal("Error while reading the table entries"));
        }

        let number_of_entries_bytes: [u8; size_of::<usize>()] = [0; size_of::<usize>()];

        let entry_count: u64 = u64::from_ne_bytes(number_of_entries_bytes);

        if(entry_count == 0) {
            return Err(DatabaseError::DbNotification("The database if empty, to tables found"))
        }


        //If its now empty the go to the first element
        //Load the table info,
        //Then load the Download data from the end of the disk
        let new_table_entries_buffer: Vec<u8> = (vec![0; entry_count as usize]);

        match db_file.seek(SeekFrom::Start(size_of::<u64>() as u64)){
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::DbInternal("Failed to set file position")),
        };

        db_file.read(&mut new_table_entries_buffer.as_mut_slice());


        /*
        match self.db_file.read(&mut last_element_position_pre_read_buffer) {
            Ok(_) => (),
            Err(_) => return Err(DatabaseError::DbInternal("Failed to write to the last_element_position_pre_buffer"))
        };
        */

        Ok(())

    }
}
        

struct Database<'a>{
    download_records: Vec<Rc<Download<'a>>>,
    file_system_table: DbTable<'a>,
    file: File,
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

        let _download_record_bytes: Vec<u8> = (*download).to_generate_bytes();

        self.download_records.push(download);

        let file: File = OpenOptions::new()
            .write(true)
            .open(database_path)
            .unwrap();

        //Create the downoad_table


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




//
//Tests
//-----------------------------------------------------------------------------------------------------------------------------------------------------------
//

#[cfg(test)]
mod tests{
    use std::fs::File;
    use std::fs::OpenOptions;
    use super::DbTable;

    const database_location: &'static str = "/home/teddy/database.bin";

    fn test_db_load_entries() {
        let mut db_table = DbTable::new(database_location);

        db_table.load_entries();
    }

    #[test]
    fn test_db_create_tables() {
        let mut db_table = DbTable::new(database_location);

        db_table.create_table();
    }

}

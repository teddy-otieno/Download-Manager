///Download manager
///
///Due to lack of download managers in the linux world
///I have decided to make my own
extern crate threadpool;
extern crate chrono;

mod scheduler;
mod download_handler;
mod listener;
mod tests;

use download_handler::DownloadHandler;

fn main() {
    //Load Config
    //Parse config
    //Setup the UI
    //Check for incomplete downloads
    //Procedd donwloads on seperate thread.
    //
    let mut download_handler = DownloadHandler::new();
    
    let download_handle = download_handler.add_download("hello".to_string(), "hello".to_string());

}


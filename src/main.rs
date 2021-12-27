use lib_peparser::lib_peparser::hello;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::*;

/*calc hash values*/
// use ring::Digest::{Context, Digest, SHA256};

/*load external libs*/
mod lib_peparser;
// mod lib_searchengine;

enum SearchEngineFileExtension{
    Dummy (bool),
    PeFile (bool),
    Jpeg (bool),
    Pdf (bool),
    //TODO add all common filetypes
}

#[derive(Debug)]
enum SearchEngineFileHashes{
    // FileHashSha1(),
    // FileHashSha2(),
}
impl SearchEngineFileHashes{
    fn calculate_sha1() /*-> Result<Digest>*/{

    }
}

/*SearchEngineFile


*/
#[derive(Debug)]
struct SearchEngineFile{
    //TODO add File with multiple file headers
    FileName: String,
    FileExtension: String,
    FileSize: u16,
    // parse meta data creation time, modified time ....
    // FileCreationDate, 
    FileHashes: SearchEngineFileHashes,
    IsMalicious: bool,



}

#[derive(Debug)]
struct SearchEngine{

    se_root_dir: String,
    se_current_dir: String,
    // se_file: SearchEngineFile,
    /*TODO Check OS */
    // Scantime
    // se_scantime_begin: ,
    // se_scantime_end: ,
    // se_file_extension: SearchEngineExtension,
    //TODO search filter
    // se_filter: enum SearchEngineFilter,
    
}

impl SearchEngine{
    fn new() -> Self{
        Self {
            se_root_dir: String::from("C:\\"),
            //TODO Read User from WIndows API (%USERPROFILE%) and Linux 
            // for directory scan
            se_current_dir: String::from("C:\\Users\\%USERPROFILE%"),
            }
    }
    fn debug(&self){
        println!("{:?}",self);
    }
    fn start_search(&self){
        // let path = Path::new(&self.se_current_dir);
        let search_dir=String::from("C:\\Users\\Ivan");

         let path = Path::new(&search_dir);

        match fs::read_dir(path) { 
            Err(err) => println!("{:?}", err.kind()),
            Ok(path) =>
                for file in path{
                    println!("{:?}",file.unwrap().path());
                }
    }
    fn scan_file(){

    }

    }
}


fn main() {

    let s1=SearchEngine::new();
    s1.debug();
    s1.start_search();


    println!(
        "User's Name            whoami::realname():    {}",
        whoami::realname()
    );
     // println!("Hello, world!");
}

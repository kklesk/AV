use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;


// 
use walkdir::WalkDir;

/*calc hash values*/
use ring::digest::{Context, Digest, SHA256, SHA512};
use std::io::{BufReader, Read, Write};
use blake3::*;
// use data_encoding::HEXUPPER;

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
struct SearchEngineFile{
    //TODO add File with multiple file headers
    file_name: String,
    file_extension: String,
    file_size: u16,
    // parse meta data creation time, modified time ....
    // FileCreationDate, 
    file_hashes: Vec<Digest>,
    is_malicious: bool,
    is_directory: bool,
}

impl SearchEngineFile{

    fn new() -> Self{
        Self{
            file_name: "".to_string(),
            file_extension: "".to_string(),
            file_size: 0,
            file_hashes: Vec::new(),
            is_malicious: false,
            is_directory: false,
        }    
    }
    fn debug(&self){
        println!("Search Engine File {:?}",self);
    }
 }


#[derive(Debug)]
struct SearchEngine{
    se_root_dir: String,
    se_current_dir: String,
    se_file: SearchEngineFile,
    // se_file: HashMap<String, SearchEngineFile>,
    //TODO get all system devices => look for root device on windows or linux and create default root entrypoint for the OS
    /*TODO Check OS */
    //TODO GET OS User
    //TODO recursive search
    //TODO is Directory, is File, is ...
    //TODO Scantime
    // se_scantime_begin: ,
    // se_scantime_end: ,
    // se_file_extension: SearchEngineExtension,
    //TODO search filter
    // se_filter: enum SearchEngineFilter,
    
}

impl SearchEngine{
    fn new() -> Self{
        Self {
            se_root_dir: String::from("C:\\Users\\Ivan\\development\\test_instance\\testfolder_files\\"),
            //TODO Read User from WIndows API (%USERPROFILE%) and Linux 
            // for directory scan
            se_current_dir: String::from("C:\\Users\\%USERPROFILE%"),
            se_file: SearchEngineFile::new(),
        }
    }
    fn debug(&self){
        println!("Search Engine {:?}",self);
        // println!("Search Engine {:?}",self.se_file.debug());
    }

    fn start_search(&mut self){

        for entry in WalkDir::new(&self.se_root_dir).into_iter().filter_map(|e| e.ok()){
            //DEBUG Print 
            // println!("{:?}",entry.file_name());
            if entry.path().is_file(){
                self.calculate_hashes(&entry.path())
            }
        }
        // let initial_walk_dir = Walkdir::new(&self.se_root_dir).into_iter().filter_map(|e| e.Ok());
    }
    fn calculate_hashes(&mut self, absolute_path_to_file: &Path){

        let current_file = match File::open(absolute_path_to_file){
            Ok(file) => file,
            // Err(err) => panic!("ERROR: while read file in function calculate_hashes {}",err),
            Err(err) => panic!("ERROR: while read file in function calculate_hashes {}",err),
    
        };
        let mut buf_reader = BufReader::new(current_file);
        let mut ctx = Context::new(&SHA256);
        let mut buffer = [0;4096];
        loop {
            let count = match buf_reader.read(&mut buffer){
                Ok(count) => count,
                Err(err) => panic!("ERROR: while calculate checksum{}",err)
            };
            if count == 0 {
                break;
            }
            ctx.update(&buffer[..count]);
        }
        let hash = ctx.finish();
        // Push gen hash value into SearchEngineFile Hash vector 
        self.se_file.file_hashes.push(hash);
        //DEBUG Print
        // println!("{:?}", hash);
}
    fn scan_file(){

    }

}

fn main() {
    println!("hello");
    let mut s1=SearchEngine::new();
    s1.start_search();
    // s1.start_search();
    s1.debug();
}
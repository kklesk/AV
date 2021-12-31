use lib_peparser::lib_peparser::hello;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;

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
    FileName: String,
    FileExtension: String,
    FileSize: u16,
    // parse meta data creation time, modified time ....
    // FileCreationDate, 
    FileHashes: Vec<Digest>,
    IsMalicious: bool,
}

impl SearchEngineFile{

    fn new() -> Self{
        Self{
            FileName: "".to_string(),
            FileExtension: "".to_string(),
            FileSize: 0,
            FileHashes: Vec::new(),
            IsMalicious: false,
        }    
    }
    fn debug(&self){
        println!("Search Engine File {:?}",self);
    }
    // fn calculate_hashes(&mut self, path_str: String) -> (Digest){
        //TODO replace this function with a template function
    fn calculate_hashes(&mut self, path_str: String) {
    
        let mut path = Path::new(&path_str);
        // let mut path = Path::new("C:\\Users\\Ivan\\development\\AV\\abc.bin");
        
        let current_file = match File::open(&path){
            Err(e) => panic!("Error in calculate_hashes: {}",e),
            Ok(file) => file,
        };
        let mut buf_reader = BufReader::new(current_file);
        let mut ctx = Context::new(&SHA256);
        let mut buffer = [0; 4096];
        loop { 
            let count =  match buf_reader.read(& mut buffer){
                Err(e) => panic!{"Err {}", e},
                Ok(count) => count,
            };
            if count == 0 {
                break;
            }
            ctx.update(&buffer[..count]);
        }

        //Test cases 
        self.FileSize = 10;
        let hash = ctx.finish();
        self.FileHashes.push(hash);

    }
}

#[derive(Debug)]
struct SearchEngine{

    se_root_dir: String,
    se_current_dir: String,
    se_file: SearchEngineFile,
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
            se_file: SearchEngineFile::new(),
        }
    }
    fn debug(&self){
        println!("Search Engine {:?}",self);
        println!("Search Engine {:?}",self.se_file.debug());
    }
    fn start_search(& mut self){
        let path = Path::new(&self.se_root_dir);
        // let search_dir=String::from("C:\\Users\\Ivan");

        //  let path = Path::new(&search_dir);
        
        //Test cases 
        // self.se_file.FileSize = 10;
        self.se_file.calculate_hashes("C:\\Users\\Ivan\\development\\AV\\abc.bin".to_string());

        match fs::read_dir(path) { 
            Err(err) => println!("{:?}", err.kind()),
            Ok(path) =>
                for file in path{
                    println!("{:?}",file.unwrap().path());
                    // self.se_file.calculate_hashes(file);      
                }
        }
    }

    fn scan_file(){

    }

}

fn main() {



//    let e = SearchEngineFileHashes::calculate_sha2();

    let mut s1=SearchEngine::new();
    s1.start_search();
    s1.debug();

}

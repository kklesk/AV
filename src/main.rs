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
enum SearchEngineFileHashes{
    Sha1(Digest),
    Sha2(Digest),
}
impl SearchEngineFileHashes{
    fn calculate_sha1() /*-> Result<digest> */{

    } 
    fn calculate_sha2() -> Digest{
        //TODO add dyn path variable E0277
        // let mut file_path = Path::new(path);
        //for debbuging, write static string 
        let file_path = Path::new("C:\\Users\\Ivan\\development\\AV\\abc.bin");
        let mut context = Context::new(&SHA256);
        let mut buffer = [0; 4096];

        let mut file = match File::open(&file_path){
            Err(err) => panic!("could not open file {} for generate sha256 sum", err),
            Ok(file) => file,
        };
        let buffer_reader = BufReader::new(file);

        loop { 
            let count = buffer_reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            context.update(&buffer[..count]);
        }
        let hash = context.finish();
        println!("{:?}", hash);
        hash
        // Ok(hash)
        // blake3 library test
        // let mut hasher = blake3::Hasher::new();
        // let mut calc_sha2 = [0;1000];
        // let mut output_reader = hasher.finalize_xof();
        // output_reader.fill(&mut calc_sha2);
        // println!("{:?}",calc_sha2);
        // // let mut output_reader = blake3::Hash(b"hello"); 
        // // Hasher::hasher.finalzie_xof();
    
    }
    fn calculate_ssdeep() /*-> Result<digest> */{

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

impl SearchEngineFile{

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



   let e = SearchEngineFileHashes::calculate_sha2();

    // let s1=SearchEngine::new();
    // s1.debug();
    // s1.start_search();



    // let hash1 = blake3::hash(b"foobarbaz");
    // println!("{:?}",hash1);


    // let fp = Path::new("C:\\Users\\Ivan\\development\\AV\\abc.bin");
    // let f = File::open(fp);
    // let mut buf_reader = BufReader::new(f);



    // let mut hasher = blake3::Hasher::new();

    // let mut calc_sha2 = [0;1000];
    // let mut output_reader = hasher.finalize_xof();
    // output_reader.fill(&mut calc_sha2);
    // println!("{:?}",calc_sha2);

    // println!(
    //     "User's Name            whoami::realname():    {}",
    //     whoami::realname()
    // );
     // println!("Hello, world!");

}

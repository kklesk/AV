use std::collections::HashMap;
use std::path::{Display, Path};
use std::{fmt, fs, panic};
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
    //magicbyte Vec array for unsigned 8 bytes X MagicBytes
    //file_magic_byte: Vec<[u8;X]>
    // file_magic_byte: Vec<[u8;4]>,
    file_magic_byte: Vec<String>,
    //TODO create hashmap
    file_name: Vec<String>,
    //TODO create hashmap
    file_extension: String,
    //TODO create hashmap
    file_size: Vec<u64>,
    // parse meta data creation time, modified time ....
    // FileCreationDate, 
    //TODO create hashmap
    //md5 size = 16;sha1 size= 20; sha256 size = 32
    file_hashes: Vec<Digest>,
    is_malicious: bool,
    is_directory: bool,
}

impl SearchEngineFile{
 
    fn new() -> Self{
        Self{
            file_magic_byte: Vec::new(),
            file_name: Vec::new(),
            file_extension: "".to_string(),
            file_size: Vec::new(),
            file_hashes: Vec::new(), 
            is_malicious: false,
            is_directory: false,
        }    
    }
    fn debug(&self){
        println!("Search Engine File {:#?}",self);
    }
 }


#[derive(Debug)]
struct SearchEngine<T>{
    se_root_dir: String,
    se_second_root_dir: T,
    se_current_dir: String,
    se_file: SearchEngineFile,
    //TODO create hashmap
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

impl<T> fmt::Display for SearchEngine<T>
    where
        T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"se_second_root_dir is:{}",self.se_second_root_dir)
    }

}

impl <T> SearchEngine<T>{
    fn new(root_dir: T) -> Self{
        Self {
            se_root_dir: String::from("C:\\Users\\Ivan\\development\\test_instance\\testfolder_files\\"),
            se_second_root_dir: root_dir,
            //TODO Read User from WIndows API (%USERPROFILE%) and Linux 
            // for directory scan
            se_current_dir: String::from("C:\\Users\\%USERPROFILE%"),
            se_file: SearchEngineFile::new(),
        }
    }
        fn set_root_dir(&mut self,  new_root_dir: T){
            self.se_second_root_dir = new_root_dir;
        }

   // fn debug(&self){
     //   println!("Search Engine {:#?}",self);
    //}

    fn start_search(&mut self){

        for entry in WalkDir::new(&self.se_root_dir).into_iter().filter_map(|e| e.ok()){
            //DEBUG Print 
            // println!("{:?}",entry.file_name());
            if entry.path().is_file(){
                self.calculate_hashes(&entry.path());
                self.calculate_size(&entry.path());
                self.calculate_extension(&entry.path());
                self.calculate_magic_byte(&entry.path());
                //TODO create methods for name
                // self.se_file.file_name.push(entry.path()); 
            }
        }
        // let initial_walk_dir = Walkdir::new(&self.se_root_dir).into_iter().filter_map(|e| e.Ok());
    }
    // calculate the first X bytes for each file -> to get magic bytes
    fn calculate_magic_byte(&mut self, absolute_path_to_file: &Path){

        // to parameterize the x bytes
        // buffer: unsigned 8byte array
        let mut buffer:[u8;4] = [0;4];
        // let mut buffer_to_str;
  
        let mut current_file = match File::open(absolute_path_to_file){
            Ok(file) => file,
            Err(err) => panic!("Err: while reading magic bytes {:?}", err),
        };
        // check is magicbyte available
        match Read::read(&mut current_file, &mut buffer){
            Ok(current_file) => current_file,
            Err(err) => panic!("Err: while reading magic bytes {:?}", err),
        };
        //Debug Print
        println!("In Function calculate_magic_byte {:#06X?} for fiil: {:?}",buffer, absolute_path_to_file.file_name());
        //TODO push magicbyte into self.se_file.file_extension Vec!!
        // self.se_file.file_extension.push(buffer);
        // buffer_to_str = std::str::from_utf8(&mut buffer);

        let magic_bytes_in_string = match std::str::from_utf8(&mut buffer){
            Ok(magic_bytes_in_string) => magic_bytes_in_string.to_string() ,
            Err(err) => panic!("Err: while set magic byte to struct {:?}", err),
        };
        self.se_file.file_magic_byte.push( magic_bytes_in_string);

    }

    fn calculate_hashes(&mut self, absolute_path_to_file: &Path){

        let current_file = match File::open(absolute_path_to_file){
            Ok(file) => file,
            // Err(err) => panic!("ERROR: while read file in function calculate_hashes {}",err),
            Err(err) => panic!("ERROR: while read file in function calculate_hashes {}",err),
    
        };
        // generate hash values for each file
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

    fn calculate_size(&mut self, path: &Path){
        self.se_file.file_size.push(fs::metadata(path).unwrap().len());
        //DEBUG print
        // println!("{}",self.se_file.file_size);
    }
    //TODO
    fn calculate_extension(&mut self, path: &Path){

        //self.se_file.file_extension.push(path.extension())

        // self.se_file.file_size.push(path.extension());
        // println!("{:#?}",path.extension());
        // println!("{:#?}",fs::metadata(path));
        // match path.extension(){
        //     Ok(extension) =>self.se_file.file_name.push(extension),
        //     Err(e) => panic!("Error: While reading extension "),
        // }

    }

    fn scan_file(){

    }

}

fn main() {
    let path = Path::new("C:\\Users\\Ivan\\development\\");
    let mut s1=SearchEngine::new(path);
    //println!("{:#?}",s1.se_second_root_dir);
    s1.start_search();
    //s1.calculate_extension();
    // s1.start_search();
    // s1.debug();

    // println!("#\n#\n#\n#\n");
    // s1.se_file.debug();
}
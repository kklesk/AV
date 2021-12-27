use lib_peparser::lib_peparser::hello;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::*;
use std::fs;
mod lib_peparser;
mod lib_searchengine;



fn main() {

    let s1=lib_searchengine::SearchEngine::new();
    s1.debug();
    s1.start_search();

     // println!("Hello, world!");
}

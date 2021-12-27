pub mod lib_searchengine{

    enum SearchEngineExtension{
        Dummy (bool),
        Pe_file (bool),
        Jpeg (bool),
        Pdf (bool),
        //TODO add all common filetypes
    }
    
    enum SearchEngineIsInfected{
        IsInfected(bool),
    }
    
    #[derive(Debug)]
    struct SearchEngine{
    
        se_root_dir: String,
        se_current_dir: String,
        // se_file_extension: SearchEngineExtension,
        se_is_infected: bool,
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
                se_is_infected: false,
                }
        }
        fn debug(&self){
            println!("{:?}",self);
        }
        fn start_search(&self){
            // let path = Path::new(&self.se_current_dir);
            let p1=String::from("C:\\Users\\Ivan");
    
             let path = Path::new(&p1);
    
            match fs::read_dir(path) { 
                Err(why) => println!("{:?}", why.kind()),
                Ok(path) =>
                    for file in path{
                        println!("{:?}",file.unwrap().path());
                    }
        }
    
        }
    }

}
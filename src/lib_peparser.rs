/*
#[cfg(test)]
*/
pub mod lib_peparser{

    use std::u32;
    
    
    /// Header for PE files
    /// https://docs.microsoft.com/en-us/windows/win32/debug/pe-format
    /// COFF File Header (Object and Image)
    
    pub struct PeImageFileHdr{
    
        magic: u32,
        machine: u16,
        number_of_section: u16,        
        time_date_stamp: u32,
        pointer_to_symbol: u32,
        size_of_optional_header: u16,
        characteristics: u16,
    }
    // File Headers 32 bit compiled
    pub struct PeImageOptionalFileHdr32{
        magic: u16,
        major_linker_version: u8,
        minor_linker_version : u8,
        size_of_code: u16,
        size_of_initialized_data: u16,
        size_of_uinitialized_data: u16,
        address_of_entry_point: u16,
        base_of_code: u16,
        image_base: u16,
        section_alignment: u32,
        file_alignment: u32,
        major_operating_system_version: u8,
        minor_operating_system_version: u8,    
        major_image_version: u16,
        minor_image_version: u16,    
        major_subsystem_version: u8,
        minor_subsystem_version: u16,
        win32_version_value: u32,
        size_of_image: u32,
        size_of_headers: u32,
        check_sum: u32,
        subsystem: u16,
        dll_characteristics: u16,
        size_of_stack_reserve: u32,  // 4/8
        size_of_stack_commit: u32,  //4/8
        size_of_heap_reserve: u32, //4/8
        size_of_heap_commit: u32, //4/8
        loader_flags: u32, 
        /*Note that the number of directories is not fixed. 
        Before looking for a specific directory, check the NumberOfRvaAndSizes field in the optional header. */
        number_of_rva_and_sizes: u32, 
    }
    // File Headers 64 bit compiled
    pub struct PeImageOptionalFileHdr64{
        magic: u16,
        major_linker_version: u8,
        minor_linker_version : u8,
        size_of_code: u16,
        size_of_initialized_data: u16,
        size_of_uinitialized_data: u16,
        address_of_entry_point: u16,
        image_base: u16,
        section_alignment: u32,
        file_alignment: u32,
        major_operating_system_version: u8,
        minor_operating_system_version: u8,    
        major_image_version: u16,
        minor_image_version: u16,    
        major_subsystem_version: u8,
        minor_subsystem_version: u16,
        win32_version_value: u32,
        size_of_image: u32,
        size_of_headers: u32,
        check_sum: u32,
        subsystem: u16,
        dll_characteristics: u16,
        size_of_stack_reserve: u64,  // 4/8
        size_of_stack_commit: u64,  //4/8
        size_of_heap_reserve: u64, //4/8
        size_of_heap_commit: u64, //4/8
        loader_flags: u32, 
            /*Note that the number of directories is not fixed. 
        Before looking for a specific directory, check the NumberOfRvaAndSizes field in the optional header. */
        number_of_rva_and_sizes: u32, 
    }
    
    // Section Table (Section Headers)
    pub struct PeImageSectionTable{
        name: u32,
        virtual_size: u32,
        virtual_address: u32,
        size_of_raw_data: u32,
        pointer_to_raw: u32,
        pointer_to_relocation: u32,
        pointer_to_line_numbers: u32,
        number_of_relocations: u16,
        number_of_line_numbers: u16,
        charachteristics: u32,
    
    }
    
    /*
    Section Data
    */
    pub struct PeImageSectionHdr{
        virtual_address: u32,
        symbol_table_index: u32,
        type_indicator: u16,
        //TODO
        // Auxiliary Symbol Records
    }
    
    // The Attribute Certificate Table (Image Only)
    pub struct PeImageCertificateHdr{
        dw_length: u32,
        w_revision: u16,
        w_certificate_type: u16,
    }
    
    /// PE data directory header
    /*
    struct PeImageDataDir{
        virtual_address: u32,
        size: u32,
    }
    */
    pub fn hello(){
        println!("HELLO From module peparser");
    }
    
    #[test]
    pub fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }

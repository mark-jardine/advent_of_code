use std::io::Read;
use std::path::Path;
use std::fs::File;

pub fn load_input_file(filepath_string: &str) -> Option<String>{
    //print cwd
    println!("cwd: {}", std::env::current_dir().unwrap().display());      
    //load inpt file
    let filepath: &Path = Path::new(filepath_string);
    let display_filepath: std::path::Display = filepath.display();

    let mut file: File = match File::open(&filepath){
        Err(why) => panic!("Couldn't open input file {}: {}", display_filepath, why),
        Ok(file) => file,
    };

    let mut input: String = String::new();

    match file.read_to_string(&mut input){
        Err(_) => return None,
        Ok(_) => return Some(input.to_string()),
    };
}
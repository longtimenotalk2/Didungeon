use std::{io, fs::{File, self}};


pub fn clear_cmd() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn save_file(save_path : &str, data : String) -> Result<(), io::Error> {    
    File::create(save_path)?;
    fs::write(save_path, data)?;

    Ok(())
}

pub fn load_file(save_path : &str) -> Result<String, io::Error> {
    fs::read_to_string(save_path)
}
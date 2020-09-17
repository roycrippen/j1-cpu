use std::fs::File;
use std::io::{Read, Write};

pub fn read_forth_source(file_name: &String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.into_bytes())
}

pub fn read_binary(file_name: &String) -> std::io::Result<Vec<u8>> {
    let mut f = File::open(file_name)?;
    let xs = &mut Vec::new();
    f.read_to_end(xs)?;
    let _len = xs.len();
    Ok(xs.clone())
}

pub fn write_log_file(file_name: &String, s: &String) -> std::io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(s.as_ref())?;
    Ok(())
}

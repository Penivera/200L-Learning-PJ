use std::fs::{File};
use std::io::{Read, Write};
use std::path::{Path,PathBuf};
use std::collections::HashMap;


pub fn main(){
    let mut file_in: File = File::create(Path::new("src").join("check.csv")).unwrap();
    let mut file_out: File = File::open(Path::new("src").join("test.csv")).unwrap();
    let mut buffer: [u8; 469] = [0u8;469];
    loop{
        let nbyte = file_out.read(&mut buffer).unwrap();
        file_in.write_all(&buffer[..nbyte]).unwrap();
        if nbyte<buffer.len(){break;}
    }
    let file_path:PathBuf = Path::new("src").join("check.csv");
    let file: File = File::open(file_path).unwrap();
    let file: BufReader<File> = BufReader::new(file);
    let file_data: HashMap<String, u8> = count_lines(file).unwrap();
    println!("File data: {:?}",file_data.get("lines").unwrap());

}
fn count_lines(file:BufReader<File>)->Result<HashMap<String,u8>,FileError>{
    let mut lines:u8 = 0;
    let mut empty_lines:u8 = 0;
    for line in file.lines(){
        lines+=1;
        if line?.trim().len()==0{
            empty_lines+=1
        }
    }
    let mut result:HashMap<String,u8> = HashMap::new();
    result.insert("lines".to_string(), lines); result.insert(String::from("empy_lines"), empty_lines);
    return Ok(result);
}
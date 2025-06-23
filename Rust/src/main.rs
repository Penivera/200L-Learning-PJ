use std::path::Path;
#[allow(unused_imports)]
use std::{fs::File, io::{Read, Write},env::current_dir,path::PathBuf};
fn main(){
    let mut file: File = File::create(Path::new("src").join("test.csv")).unwrap();
    
    println!("{:?}",file.write_all("Title,Date".as_bytes()).unwrap());
    let mut new_file_sess = File::open("src/test.csv").unwrap();
    let mut file_con:String = String::new();
    
    new_file_sess.read_to_string(&mut file_con).unwrap();
    println!("File content {file_con}");

}
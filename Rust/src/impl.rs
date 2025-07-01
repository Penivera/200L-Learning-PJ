#[allow(unused_imports)]
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    io::{BufRead, BufReader, Read, Write, Error as FileError},
};

fn main(){
    struct Student {
        name:String,
        reg_no:String,
        level:u8
    }
    impl Student {
        //can pass self without typing for dot notation or reference type for function 
        fn name(self:Student)->String{
            return format!("{}\n{}\n{}",self.name,self.reg_no,self.level);
        }
    }
    let boojee = Student{
        name: "Unwana Bassey".to_string(),
        reg_no:"22/SC/CO/1193".to_string(),
        level:200,
    };
    //println!("The new student is {}",boojee.name());
    println!("Another Student is {}",Student::name(boojee));
    

    

}


use super::types::{Student, Surname, User};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
};
use std::{
    fmt::Display,
    io::{Error, Write, stdin},
};
impl User {
    pub fn new() -> User {
        User {
            id: 0,
            name: "".to_string(),
            email: "".to_string(),
        }
    }
}
#[allow(dead_code)]
fn test() {
    let mut user: User = User::new();
    println!("Hello {:?}", user);
    println!("Surname added to {:?}", {
        user.surname("Vera");
        user
    })
}

pub fn input<S: Into<String> + Display>(prompt: S) -> Result<String, Error> {
    let mut input: String = String::new();
    print!("{prompt}");
    std::io::stdout().flush().unwrap();
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(error) => Err(error),
    }
}

pub async fn do_smt() {
    let name: String = input("Enter your name: ").unwrap();
    let dpt: String = input("Enter Department: ").unwrap();
    let level: String = input("Enter level").unwrap();
    let new_student = Student {
        name: name,
        level: level,
        department: dpt,
        faculty: "Computing".to_string(),
        reg_no: input("Enter reg no: ").unwrap(),
    };
    println!("{:#?}", new_student);
    let mut some_vec: Vec<u32> = vec![
        2, 54, 34, 6, 66, 24, 2, 65767, 2, 134, 7623, 7, 8675, 3424, 825, 257, 876, 35,
    ];
    println!(
        "{:?}",
        some_vec.sort_by(|x: &u32, y: &u32| {
            if x < y {
                Greater
            } else if x > y {
                Less
            } else {
                Equal
            }
        })
    );
    let mut arr: Vec<(&str, u32)> = vec![("Peni", 34), ("Vera", 12)];
    let map: HashMap<String, u32> = arr.iter_mut().map(|v| (v.0.to_string(), v.1)).collect();
    println!("{:?}", map);
}

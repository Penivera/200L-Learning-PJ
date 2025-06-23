#[allow(unused_imports)]
use std::env::{Args,args,vars,set_var};
use std::io::{stdin, Error, stdout, Write};
fn input(prompt: &str) -> Result<String, Error> {
    print!("{prompt}");
    #[allow(unused_must_use)]
    stdout().flush();
    let mut data = String::new();
    stdin().read_line(&mut data)?;
    return Ok(data)
}

fn main() {
    // let command_line_args:Args = args();
    // println!("The command line arguments are: [{:?}]",command_line_args.collect::<Vec<String>>());
    // //for var in vars(){println!("{var:?}");}
    // //std::process::exit(105);

    // println!("First env lookup before setting[{:?}]", std::env::var("abcd"));
    // unsafe{
    //     set_var("abcd", "This is the value");
    // }
    
    // print!("Env data after set [{:?}]", std::env::var("abcd").unwrap());
    //SECTION Reading from the terminal

    // Example usage of input to avoid dead_code warning
    let name = input("What is your name ").unwrap();
    println!("{name}");
    std::io::stdout().write("Hello ".as_bytes()).unwrap();
}
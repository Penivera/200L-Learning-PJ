#[allow(unused_variables)]
fn main(){
    #[allow(dead_code)]
    enum Person {
        Legs(i32,bool),
        Hair,
    }  
    let part:Person = Person::Hair;
    match part {
        Person::Legs =>println!("Person's  Legs"),
        Person::Hair => println!("Person's Hair"),
    }
    let test = "ABIDOSHEKA";
    match test {
        "Noneses" => println!("well.."),
        _ => {},
    };
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Result {
    Success(u8),
    Failure(u16, char),
    Uncertainty,
    }
    let outcome = Result::Failure(20, 'X');
    //println!("{:?}",outcome);
    match outcome {
        Result::Success(0) => print!("Result: 0"),
        Result::Success(1) => print!("Result: 1"),
        Result::Success(_) => print!("Result: other"),
        Result::Failure(10, 'X') => println!("Error: 10 X"),
        Result::Failure(10, _) => print!("Error: 10"),
        Result::Failure(_, 'X') => println!("Error: X"),
        Result::Failure(_, _) => println!("Error: other"),
        _ => {}//Can write an entire function with custom logic in the braces also the underscore is to account for any type,
    }
    let name = "Peniel";

    match name {
        "Peniel" => println!("Hello Peniel"),
        _ => {}
    }
    let varfunc: () = {
        let x: i32 = 10;
        let y: i32 = 20;
        //x + y; // This is an expression, so it returns the value of x + y
        println!("Result of varfunc: {}", x + y);
    };
    let smt: [i32; 5] = [4,5,6,7,8];
    println!("{}",smt[2])
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person<T>{
    name:&'static str,
    age:T,
    level: &'static str,
    
}
fn divide2(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

#[allow(dead_code)]
fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divide {} by {}: {}",
            num, den, msg),
} }

fn return_person<T>(name:&'static str, age:T,level:&'static str)-> Person<T>{
    let new_person:Person<T> = Person{
        name:name,
        age: age,
        level: level,
    };
    return new_person;
}
#[allow(dead_code)]
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}


fn main(){
    let returned_person: Person<i8> = return_person::<i8>("Penivera", 4, "200 Level"); //Specific typing
    let infered_person_type = return_person("Nelson", 21, "300 Level"); //Everything is Inferred
    println!("New Person object With static types\n{:?}", returned_person);
    println!("Inferred Person type\n{:?}",infered_person_type);
    let mut v = vec![11, 22, 33];
    for _ in 0..5 {
        let item: Option<i32> = v.pop();
        
        match item {
            Some(number) => {
                println!("{}, ", number); //will aoutput the result of the pop and panic where nothing was available to be popped
                println!("Unwrapped Item: {:?}",item.unwrap());//convert the options type to the result and panic otherwise
            },
            None => println!("#, "),
        }
    }

    print!("{:?}, {:?}", divide2(8., 2.), divide2(8., 0.));

    // show_divide(8., 2.);
    // show_divide(8., 0.);
    // let r1 = divide(8., 2.);
    // let r2 = divide(8., 0.);
    // println!("{} {}", r1.is_ok(), r1.is_err());
    // println!("{} {}", r2.is_ok(), r2.is_err());
    // println!("{}", r1.unwrap());
    // println!("{}", r2.unwrap());


}

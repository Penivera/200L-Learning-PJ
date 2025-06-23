#[allow(unused_variables)]
pub fn main(){
    let sophia: (&'static str, &'static str, &'static str, i32, i32, bool) = ("sophia","You","thief",2,3,true); 
    /*Tuples are not iterable and NO you cannot dynamically enter it using an iterator counter */

    #[allow(dead_code)]
    #[derive(Debug)]
    struct SomePerson{
        name:&'static str,
        age:i8,
        level:&'static str,
        gender:char,
        courses:[&'static str;3],
        anything:(),
    } //Lexical order is important in structs, so you can access them by name
    #[allow(dead_code)]
    struct TuplePerson(
        &'static str, //name
        i8, //age
        &'static str, //level
        char,
    );
    //Tuple structs are not recommended for large data structures, but they can be useful for small ones
    
    let new_person:SomePerson = SomePerson{
        name:"Penivera",
        age:15,
        level:"200L",
        gender:'F',
        courses:["a","b","c"],
        anything:()

    };
    // for course in new_person.courses.iter() {
    //     println!("Course: {}", course);
    // }
    // println!("{:?}",new_person);

    // println!("{}",new_person.age);

    struct Test((&'static str,i8),);
    let test = Test(("Penivera", 15));
    println!("Test: {:?}", test.0.0);

}
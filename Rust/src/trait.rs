mod first_module {
    pub fn do_something()-> String{
        return "This is from some module".to_string();
    }//the pub makes it accessible outside 
    #[allow(dead_code)]
    fn another()->String{
        return "This is another".to_string();
    }//by default private and cannot be called outside the mod
}

fn f() {
    print!("f ");
    g();
    m::f();
    m::m::f();
}
fn g() { print!("g "); }
mod m {
    pub fn f() {
        print!("1.f ");
        g();
        m::f();
        super::g();
    }
    fn g() { print!("1.g "); }
    pub mod m {
        pub fn f() {
            print!("2.f ");
            g();
            super::g();
            super::super::g();
            crate::g();//from the top(global module)
}
        fn g() { print!("2.g "); }
    }
}
trait HasSquareRoot { // trait declaration
    fn sq_root(self) -> Self;
}
trait CanBeUpper{
    fn to_uppercase(self) -> String;
}
impl CanBeUpper for &str{
    fn to_uppercase(self) -> String { self.to_uppercase() }
}
impl CanBeUpper for String{
    fn to_uppercase(self) -> Self { self.as_str().to_uppercase() }
}
impl HasSquareRoot for f32 { // an implementation of the trait
    fn sq_root(self) -> Self { self.sqrt() }
}
impl HasSquareRoot for f64 { // another implementation
    fn sq_root(self) -> Self { self.sqrt() }
}
#[allow(dead_code)]
trait CanBeUpperToString: CanBeUpper+Copy{
    fn to_string(self)->String;
}
impl CanBeUpperToString for &str {
    fn to_string(self)->String {
        return String::from(self);
    }
}


// function that depends on the Number parameter,
// that must implement the HasSquareRoot trait
fn quartic_root<Number>(x: Number) -> Number
where Number: HasSquareRoot {
    x.sq_root().sq_root()
}

fn case_converter<Val>(val:Val)->String where Val :CanBeUpperToString{
    val.to_string();
    return val.to_uppercase();
}


fn main(){
    println!("Something from a module {}",first_module::do_something());
    f();
    type I32Number = i32;
    #[allow(unused_variables)]
    let check:I32Number =32;
    // Here that function if instantiated twice, in both cases
// using types that implement the HasSquareRoot trait
    println!("{}",quartic_root(100f64)); 
    println!("{}",case_converter("name"));


}
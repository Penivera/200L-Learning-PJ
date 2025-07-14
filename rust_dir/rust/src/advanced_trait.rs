use std::ops::Add;


struct Person{
    name:String,
    age:i8
}


//change what trait is implemented to use diff print e.g {}->display & {:?}->Debug
impl std::fmt::Display for Person{
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result{
        write!(
            f,
            "{} {}",
            self.name,
            format!("{} is {} old",self.name,self.age)

        )

    }
}

impl Drop for Person{
    fn drop(&mut self) {
        println!("Dropping Object of {} ",self)
    }
} //Destructor called when Person is destroyed

trait HasLnExp {
    fn ln(self) -> Self;
    fn exp(self) -> Self;
}
impl HasLnExp for f64 {
    fn ln(self) -> Self { self.ln() }
    fn exp(self) -> Self { self.exp() }
}
impl HasLnExp for f32 {
    fn ln(self) -> Self { self.ln() }
    fn exp(self) -> Self { self.exp() }
}
trait HasMultiply<Rhs> {
    fn multiply(self, rhs: Rhs) -> Self;
}
impl<Rhs> HasMultiply<Rhs> for f64 where Rhs: Into<Self> {
    fn multiply(self, rhs: Rhs) -> Self { self * rhs.into() }
}
impl<Rhs> HasMultiply<Rhs> for f32 where Rhs: Into<Self> {
    fn multiply(self, rhs: Rhs) -> Self { self * rhs.into() }
}
fn exponentiate<Base, Exponent>(
    base: Base, exponent: Exponent) -> Base
where Base: HasLnExp + HasMultiply<Exponent>
{
    (base.ln().multiply(exponent)).exp()
}
 struct S ( i32 );
impl Drop for S {
    fn drop(&mut self) {
        println!("Dropped {}", self.0);
    } }
#[derive(Debug)]
struct Complex {
re: f64,
im: f64,
}

impl Add for Complex{
    type Output = Self;
    fn add(self,rhs:Self)->Self::Output{
        return Self { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}


fn main(){
    // println!("{}", exponentiate(2.5f32, 3i16));
    // println!("{}", exponentiate(2.5f64, 3i16));
    // println!("{}", exponentiate(2.5f32, 3f32));
    // println!("{}", exponentiate(2.5f64, 3f32));
    // println!("{}", exponentiate(2.5f64, true));
    // {
    //     let new_person: Person = Person{name:String::from("Penivera"),age:3};
    //     println!("{}",new_person)
    // }
   
    //     let _a = S (1);
    //     let _b = S (2);
    //     let _c = S (3);
    //     {
    //         let _d = S (4);
    //         let _e = S (5);
    //         let _f = S (6);
    //         println!("INNER");
    //     }
    //     println!("OUTER");
    // let z1: Complex = Complex { re: 3.8, im: -2.1 };
    // let z2: Complex = Complex { re: -1.5, im: 8.6 };
    // let z3: Complex = z1 + z2;
    // println!("{:?}",z3);
    // if (8 >> 10) > 0 {
    //     println!("Greater")
    // } else if (0 >> 3) > 0 {
    //     println!("Check")
    // }
    let mut a = 0b1100;
    let b = 0b1010;

    println!("{:04b}", a & b);  // AND: 1000
    println!("{:04b}", a | b);  // OR: 1110
    println!("{:04b}", a ^ b);  // XOR: 0110

    println!("{:04b}", !a);     // NOT: depends on bits, flips all

    a &= b; println!("{:04b}", a);  // AND assignment: 1000

    let mut c = 0b0001;
    c <<= 2; println!("{:04b}", c);  // Left shift: 0100

}
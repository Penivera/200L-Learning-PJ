
fn print_double(mut x: f32)->() {
    x *= 2.;
    println!("Double printer: {}", x);
    
} //this takes ownership of the value, so it cannot change the original value
fn change_arg_value(x: &mut f32){
    *x *= 2.;
    println!("Changed value: {}", x);
}//this recieved a mutable reference to the value, so it can change the original value

fn main() {
    let mut  x: f32 = 2.0;
    print_double(x);
    println!("Original value: {}", x);
    change_arg_value( &mut x);
    println!("Changed value: {}", x);
    

}
//#[allow(dead_code)]
fn _some_fn(){
    /* f(); // Prints 2
        {
            f(); // Prints 3
            fn f() { print!("3\n"); }
        }
        f(); // Prints 2
        fn f() { print!("2"); }
        print_double(1.0); // Prints 2
        {
            fn f() { print!("3\n"); }
            f(); // Prints 3
        }
        */
}
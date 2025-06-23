
//use std::vec;
#[allow(unused_variables)]
fn main(){
    // let hexadecimal = 0x2b;//NOTE -  Number written in hexadecimal
    // println!("{hexadecimal}");
    // let octal = 0o345; //NOTE - Number written in Octal
    // print!("{octal}\n");
    // let binary: i32 = 0b010111; //NOTE -  Number written in binary
    // println!("{binary}");
    // let any_num = 1_000_000_000;//NOTE - underscores used for visual aid ignored by compiler
    // println!("{any_num}");
    // println!("{}", 23e10); //NOTE - e for exponetial
    // let a:i8 = 2;
    // let b:i32 = 30_000;
    // let c:i16 = 20_000;
    // let d:i64 = 40_000_0000_000_000;
    let e:i128 = 90_000_0000_00_000_000_000_000_000_000_000_0000;
    // println!("
    // {a}\n
    // {b}\n
    // {c}\n
    // {d}\n
    // {e}
    // ")
    let empty = ["";0];//couould be 0 for int or 0.00 for float ot true for bool,0]; //NOTE - Declared type of string
    let test = vec![2,4,5,6,7,8];
    for i in 0..test.len() {
        print!("{i}");
    }
    let try_empty: [_; 0]  = [0;0];
    let chakam:usize = 34;
    let chakam2:isize = 43;
    let chackam_pro:i8 = chakam as i8;
    println!("Chackam {}",chackam_pro);
    let floater:f32 = 0.989;
    let her:u32  = 32;
    print!("something {:?}",char::from_u32(her));


}
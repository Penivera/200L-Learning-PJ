#[allow(unused_imports)]
use std::{ops::RangeToInclusive, str::Chars, vec::IntoIter,slice::IterMut};
#[allow(unused_mut,unused_variables,unreachable_code,unused_assignments,dead_code)]
fn print_nth_char(s: &str, mut n: u32) {
    let mut iter:Chars = s.chars();
    loop {
        let item: Option<char> = iter.next();
        match item {
            Some(c) => /*if n == 0*/ { println!("{}", c); break; },
            None => { break; },
        }
        n -= 1; }
}

#[allow(unused_variables,unused_mut,dead_code)]
fn main(){
    let names:[&str;4] = ["Pen","Okon","adad","sfdf"];
    // for name in names  {println!("{}",name);}
    // let some_range: RangeToInclusive<i16> = ..=2;
    //print_nth_char("€èefshfjrwknbdfddfsf", 100); // It prints: €
    // let iters: Chars = names[0].chars();
    // for char in iters{println!("The character is {char}");}
    // let mut some_vec:IntoIter<&'static str>  = vec!["sdsds","sdsdsd","dffsd","sfsfdgdg"].into_iter();
    // println!("converting vectors into iter {:?}",some_vec.next());
    let mut v = vec![3, 4, 5];
    let iterator: IterMut<i32> = v.iter_mut();
    for mut_item_ref in iterator {
        *mut_item_ref += 1;
    }
    println!("{:?}", v);
    let arr: [char; 3] = ['a', 'b', 'c'];
    for (index, ch) in arr.into_iter().enumerate() {
        println!("{} {}, ", index, ch);
    }
    println!("{:?}", names.into_iter().collect::<String>());
    println!("{:?}", names.into_iter().collect::<Vec<&str>>()); //.collect creates a new collection from the iterator types can be specified via an turbofish exp or type inference 

    let smt =[66, -8, 43, 19, 0, -31]
        .into_iter()
        .filter(|x| { print!("F{} ", x); *x > 0 })
        .map(|x| { print!("M{} ", x); x * 2 });
    /*This program prints nothing, because it does nothing. Even the compiler reports the warning: unused `Map` that must be used, and then the note: iterators are lazy and do nothing unless consumed.*/
}

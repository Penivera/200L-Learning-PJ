
#[allow(unused_imports)]
use std::ops::Range;
use std::cmp::{Ordering, PartialOrd};
use std::fmt::Debug;
fn any_arr_sorter<T: PartialOrd+Copy+Debug>(arr:&mut [T])->(){
    for i in 0..arr.len(){
        for k in 0..arr.len(){
            if arr[i]<arr[k]{
                arr.swap(i, k);

            }
        }
                
    }
    
}

#[allow(unused_variables)]
fn main(){
    // let name:String = String::from("Peniel");
    // let last_name:String = String::from("Ben");
    // let fullname:String = format!("{name} {last_name} & {}",String::new());
    // println!("{fullname}");
    // let range: Range<i32> = 0..12;
    // //for i in range{println!("{i}");}
    // let first: &str = &name[0..2];
    // println!("The first letter of the name is {first}");
    // let arr:[i8;12]= [3;12];
    // for i in 0..12{println!("{}",arr[i]);}
    let mut arr: Vec<i32> = vec![1,0,4,6,78,9,45,3,42,3,45,4566];
    let mut arr2:Vec<i32> = arr.clone();
    arr2.sort_by(|a: &i32, b: &i32| {
        if a > b {
            Ordering::Greater
        } else if a < b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    let mut arr3: [i32; 12] = [1,0,4,6,78,9,45,3,42,3,45,4566];
    any_arr_sorter::<i32>(&mut arr);
    any_arr_sorter(&mut arr3);
    println!("More granular sorting logic{:?}",arr);
    println!("Rust closure sorting logic: {:?}",arr2);
    println!("SOrting normal array: {:?}",arr3);
    println!("slicing with ranges: {:?}", &arr[0..5]);
    
    
}
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_variables)]
fn main(){
    let name:String = String::from("Peniel");
    // let name2:&str = "Peniel";
    // let ara:[i32; 1] = [0];

    // println!("size of void: {}",std::mem::size_of::<i32>());
    // println!("Bit size of my name: {}",std::mem::size_of_val(&name));
    // println!("Bit size of str slice: {}",std::mem::size_of_val(name2));
    // println!("Bit size of array {}",std::mem::size_of_val(&ara));
    // println!("Bit size of smt: {}",size_of::<Vec<i32>>());
    // println!("Bit size of another smt: {}",size_of::<()>());
    // println!("size of: {}",size_of::<u128>());
    // println!("Memory location: {}",&name as *const String as usize);
    // println!("Pointer to a var: {:p}",&name,); //same addy as the one above just one in binary and the other in hex

    // let mut v: Vec<i32> = vec![0; 0];
    // println!("{} {}", v.len(), v.capacity());
    // v.push(11);
    // println!("{} {}", v.len(), v.capacity());
    // v.push(22);
    // println!("{} {}", v.len(), v.capacity());
    // v.push(33);
    // println!("{} {}", v.len(), v.capacity());
    // v.push(44);
    // println!("{} {}", v.len(), v.capacity());
    // v.push(55);
    // println!("{} {}", v.len(), v.capacity());
    // let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    // arr.sort();
    // println!("Sorted array: {:?}", arr);
    let mut arr = [4, 8, 1, 10, 0,1, 45, 12, 7];
    use std::cmp::Ordering;
    fn desc(a: &i32, b: &i32) -> Ordering {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else {Ordering::Equal}
    }
    arr.sort_by(desc);
    print!("Sorted wit closure function\n{:?}", arr);
    
}
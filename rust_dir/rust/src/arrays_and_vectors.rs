//#[allow(unconditional_panic)] excape error in compilation for index out of range
#[allow(unused_variables)] //NOTE -you can allow, deny or warn
fn main(){
    let names = ["Peniel","Sophia"];
    /*for name in names {print!("{name} \n");}
    print!("{}",names[0]);*/
    let new_array:Vec<[i32]> = [3;20]; //NOTE - Generate new array of the first niumber for second number amount of index
    // let test_array = [
    //     [
    //         [
    //             2;3
    //         ];
    //         4
    //     ];
    //     5
    // ];
    // println!("{:?}\n {:?}",test_array[0][1],test_array.len());
    let dynamic_array: Vec<&'static str> = vec!["Hello","There"];
    println!("{:?}",dynamic_array);

    let mut check1 =[-1; 10] ;
    #[allow(unused_mut)]
    let mut check2 = [2;10];
    println!("{:?} {:?}",check1, check2);
    check1 = check2 ; //Will work for arrays 
    print!("{:?} {:?}", check1,check2);
    check1 = check2;
    print!("{:?} {:?}",check1,check2);
    #[allow(unused_mut)]
    let mut check_v2 = vec![7;7];
    let mut check_v1 = vec![5;7];
    check_v1 = check_v2; 
    //print!("{:?} {:?}",check_v1, check_v2) //v2 does not exist as it has been compied and destroyed 

}
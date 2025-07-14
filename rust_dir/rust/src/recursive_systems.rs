const SIZE: usize = 100_000;
const N_ARRAY: usize = 1_000_000;
#[allow(dead_code)]
fn create_array() -> [u8; SIZE] { [0u8; SIZE] }
#[allow(dead_code)]
fn recursive_stack_func(n: usize) {
    let a = create_array();
    println!("N Array {}\t A[0]:{}", N_ARRAY - n + 1, a[0]);
    //println!("{:?}",a);
    println!("{n}");
    if n > 1 { recursive_stack_func(n - 1) }
}

fn create_heap_array() -> Box<[u8; SIZE]> { Box::new([0u8; SIZE]) }
fn recursive_heap_func(n: usize) {
    let a: Box<[u8; 100000]> = create_heap_array();
    println!("{} {}", N_ARRAY - n + 1, a[0]);
    if n > 1 { recursive_heap_func(n - 1) }
}


fn main() {
    //recursive_stack_func(N_ARRAY); //this will crash the thread as it is executing on the stack
    // You can now use items from my_module here
    recursive_heap_func(N_ARRAY);
}
pub mod types;
pub mod utils;
pub mod ran;
pub mod cus_macro;

#[allow(unused_imports)]
use types::{Student, Surname, User};
use tokio::{join,select};
use std::time::Duration;

async fn anything(thing:&str,duration:u64)->&str{
    tokio::time::sleep(Duration::from_millis(duration)).await;
    return thing;
}
#[tokio::main]
async fn main(){
    let something: (&str, &str) = join!(
        anything("blah", 4),
        anything("thing", 4)
    );
    println!("{:?}",something);
    let something2 = select!{
        val =anything("blah", 4)=> {
            println!("future one completed val:{}",val);
            val
        },
        val  = anything("thing", 4) => {
            println!("Future 2 completed val {}",val);
            val
        }
    };
    println!("{:?}",something2);
    println!("THe sum of smt is {}",sum![1,2,3,4,5,6,7]);

}

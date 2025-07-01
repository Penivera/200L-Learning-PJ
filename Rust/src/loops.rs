
fn main(){
    let mut counter:i32 = 1;
    while counter <= 10 {
        if counter%2 !=0{
            println!("{}",counter);
        }
        counter +=1;
    }
    counter = 1;
    /*loop{
        println!("Round {counter}");
        counter +=1;
    }*/
    for i in 1..11{
        print!("{i}");
    }

    for i in 1..=5{
        print!("count: {i}");
    }

}
use std::io::{Read, Write};
struct BoxedText {
    text:String,
    prefix:char
}

impl BoxedText {
    fn from(text:&str,prefix:char)->BoxedText{
        BoxedText { text:text.to_string(), prefix: prefix }

    }
}
#[allow(dead_code)]
struct Text{
    text:String
}



trait Draw {
    fn draw(&self);
}

impl Draw for Text{
    fn draw(&self){
        println!("{}",self.text)
    }
}

impl Draw for BoxedText {
    fn draw(&self){
        println!("{}, {}",self.text,self.prefix)
    }
}
fn draw_text(txt:&dyn Draw){ //&dyn tells the compiler that txt is of a dynamic type of Draw trait
    txt.draw()
}


fn main(){
    let start_time = std::time::Instant::now();
    let mut smt = BoxedText::from("Penivera",'A');
    let mut input: String = String::new();
    let is_bool = input.trim() == "b";
    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdout().flush().unwrap();
    if is_bool{
        draw_text(&mut smt);
    }
    println!("Time elapsed: {:?}", start_time.elapsed());

}
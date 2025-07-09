abstract class Animal{
    public void sound(){
        System.out.println("Mooooh");
    }
}

class Cow extends Animal{
    
}

public class Abstract {
    public static void main(String[] args){
        Cow cow1 = new Cow();
        cow1.sound();
    }
    
}

 

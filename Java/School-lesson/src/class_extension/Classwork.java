//Declare an abstract class and call it Car with three methods and tehn create 2 classes to extend that abstract class the program should carry 3 instance variables these instance variables will be used in the 2 classes 

abstract class Car{
    public int speed(String road){
        if ("Express".equals(road)){
            System.out.println("Floor it");
            return 1000;
        }
        else if ("one way".equals(road)){
            System.out.println("Slow down");
            return 20;
        }
        else{
            return 0;
        }
        
    }
    public void honk(String type){
        if (type.equals("BMW")){
            System.out.println("Paaaam");
        }
        else if (type.equals("Toyota")){
            System.out.println("POooom");
        }
        else{
            System.out.println("Piiimmm");
        }
    }
    public void fuel_level(){
        System.out.println("Empty");
    }
}

class Toyota extends Car{
    String type = "Toyota";
    public void sound(){
        honk(type);
    }
    
}

class BMW extends Car{
    String type = "BMW";
    String road = "Express";
    public void details(){
        System.out.println("Speed "+ speed(road));
        honk(type);
    }
}

public class Classwork {
    public static void main(String[] args) {
        BMW bmw1 = new BMW();
        bmw1.details();
        Toyota toyota1 = new Toyota();
        toyota1.sound();
    }
    

    
}

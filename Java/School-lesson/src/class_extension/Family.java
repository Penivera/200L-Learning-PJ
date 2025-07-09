class Parent{
    String name ="Udokong";
    public void message(){
        System.out.println("This is a parent method");
    }
    

}

class Child extends Parent{
    @SuppressWarnings("override")
    public void message(){
        super.message();
        System.out.println("I am a child");
        System.out.println(name);
    }
}
public class Family{
    public static void main(String[] args){
        Child some_child = new Child();
        some_child.message();
        
    }
}
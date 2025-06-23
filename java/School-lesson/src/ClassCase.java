import java.util.Scanner;

public class ClassCase {
    public static void println(Object message){
        System.out.println(message);
    }
    public static void print(Object message){
        System.out.print(message);
    }
    static Scanner input = new Scanner(System.in);
    public static void main(String[] args){
        println("Enter a value to chose an option");
        print("1: Afang Soup\n2: Rice and Stew\n3:Melon Soup\n4:Egg,Bread and Tea\nEnter Your Choice: ");
        int option = input.nextInt();
        switch (option){
            case 1:
                println("Afang Soup is Ready");
                break;
            case 2:
                println("Rice & Stew is Ready");
                break;
            case 3:
                println("Melon soup is Ready");
                break;
            case 4:
                println("Afang soup is ready");
                break;
            default:
                println("Invalid choice entered");
        } //You can replace the colon with an arrow(->) to avoid the multiple use of break
        do{
            println("Rubbish");
        }while (true);
    }
}

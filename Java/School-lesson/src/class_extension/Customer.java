 import java.util.Scanner;

public class Customer {
    String name;
    String phone;
    String Address;

    public void get_details(){
        Scanner input = new Scanner(System.in);
        System.out.print("Enter Customer Name: ");
        name = input.nextLine().strip();
        System.out.print("Enter Customer Phone number: ");
        phone = input.nextLine().strip();
        System.out.print("Enter Customer's Address: ");
        Address =  input.nextLine().strip();

    }
    public void display_details(){
        System.out.printf("""
                Customer name: %s
                Customer Phone Number: %s
                Customer Address: %s""",name,phone,Address);
    }

}

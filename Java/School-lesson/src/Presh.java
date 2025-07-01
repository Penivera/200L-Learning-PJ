import java.util.Scanner;

class Presh{
    static String name="Presh";
    static String pwd= "I am Presh";
    public static void main(String[] args){
        Scanner sc=new Scanner(System.in);
        System.out.println("Enter name");
        String in_name=sc.nextLine().strip();
        System.out.println("Enter password");
        String in_pass=sc.nextLine().strip();
        if(name.equals(in_name)&& pwd.equals(in_pass)) {
            System.out.println("logins details correct");
        }
        else{
            System.out.println("Incorrect password");
        }




    }
}
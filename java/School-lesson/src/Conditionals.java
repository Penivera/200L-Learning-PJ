import java.util.Scanner;
public class Conditionals {
    static Scanner input = new Scanner(System.in);
    public static void main(String[] args){
        System.out.println("Enter your Blood Pressure: ");
        double bp = input.nextDouble();
        if (bp < 60){
            System.out.println("Low Blood Pressure");
        } else if (bp >=60 && bp <=130) {
            System.out.println("Normal Blood Pressure");
        } else if (bp>130) {
            System.out.println("High Blood Pressure");

        }
    }
}

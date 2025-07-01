import java.util.Objects;
import java.util.Scanner;
public class Sorter {
    static Scanner input = new Scanner(System.in);
    public static void println(Object message){
        System.out.println(message);
    }
    public static void main(String[] args){
        int[] number = new int[5];
        for (int k=0; k<5; k++){
            System.out.printf("Enter element No. %d: ",k+1);
            number[k] = input.nextInt();
        }
        String name = "Peniel";
        String another_name = "Peniel";
        if (Objects.equals(name,another_name)){
            System.out.println("Hello");

        }

    }
}

import java.util.Scanner;
public class Random {
    static Scanner input = new Scanner(System.in);
    public static void main(String[] args){
        int[] random_numbers = new int[10];
        for (int i=0; i<10;i++){
            System.out.printf("Enter random number %d: ",i+1);
            random_numbers[i] = input.nextInt();
        }
        for (int number:random_numbers){
            if (number%2 == 0){
                System.out.printf("Even numbers %d,",number);
            }
        }
    }

}

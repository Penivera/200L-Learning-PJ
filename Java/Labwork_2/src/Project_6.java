import java.util.Scanner;

public class Project_6 {
    public static void main(String[] args){

        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter numbers of apples: ");
        int number = scanner.nextInt();

        for (int i = 1; i < number; i+=2) {
            if (i%5 == 0){
                continue;
            }else {
                System.out.print(i + " ");
            }
        }
        System.out.println();
        for (int i = 2; i < number; i+=2) {
            if (i%5==0){
                continue;
            }else {
                System.out.print(i + " ");
            }
        }
    }
}

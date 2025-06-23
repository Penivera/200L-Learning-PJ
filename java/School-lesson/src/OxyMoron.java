import java.util.Scanner;

public class OxyMoron {
    public static void main(String [] args){
        Scanner value = new Scanner(System.in);
        int[][] student = new int[4][3];
        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 3; j++) {
                System.out.print("Enter value:");
                student[i][j] = value.nextInt();
                System.out.println(student[i][j] + "\t");
                System.out.println();
            }

        }

        /*for (int[] students: student ) {
            for (int stud: students){
                System.out.print(stud+ " ");
            }
            System.out.println();

         */




    }
}

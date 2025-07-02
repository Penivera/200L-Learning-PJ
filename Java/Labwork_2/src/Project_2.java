import java.util.Scanner;

public class Project_2 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int studNo;
        int NoOfQualify = 0;

        System.out.println("********************************************************************");
        System.out.println("\t\tWELCOME TO DE-GIBSON UNIVERSITY MANAGEMENT APPLICATION");
        System.out.println("********************************************************************");

        System.out.print("Enter number of students: ");
        studNo = scanner.nextInt();

        int[] Score = new int[studNo];

        for (int i = 0; i < studNo; i++) {
            System.out.printf("Enter Score for Student %d:", i+1);
            Score[i] = scanner.nextInt();

            if (Score[i]>=275){
                System.out.println("Admitted!");
                NoOfQualify++;
            }else {
                System.out.println("Not Admitted");
            }

        }

        System.out.println("Number of qualified Students: " +NoOfQualify);
        int percentage = (int) (((double) NoOfQualify/studNo)*(100));
        System.out.printf("Percentage of admitted students: %d%%\n",percentage);
        if (percentage>=60){
            System.out.println("Increase School Fees");
        }else {
            System.out.println("ADVICE!: \nDo Not Increase School Fees");
        }

    }
}
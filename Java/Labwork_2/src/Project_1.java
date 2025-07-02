import java.util.Scanner;
public class Project_1 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.print("Enter number of student: ");
        int numOfStud = scanner.nextInt();
        scanner.nextLine();

        int[] total = new int[numOfStud];
        String[] regNo = new  String[numOfStud];
        char[] grade = new char[numOfStud];
        int[] test = new int[numOfStud];
        int[] exam = new int[numOfStud];

        for (int i = 0; i < numOfStud; i++) {
            System.out.print("Enter Reg Number: ");
            regNo[i] = scanner.nextLine();

            System.out.print("Enter Test Score: ");
            test[i] = scanner.nextInt();

            System.out.print("Enter Exam Score: ");
            exam[i] = scanner.nextInt();
            scanner.nextLine();

            total[i] = exam[i] + test[i];
            if (total[i] >= 70 && total[i] <= 100){
                grade[i] = 'A';
            }else if(total[i] >=60 && total[i] <=69){
                grade[i] = 'B';
            } else if (total[i]>=50 && total[i]<=59) {
                grade[i] = 'C';
            } else if (total[i] >= 45 && total[i]<=49) {
                grade[i] = 'D';
            } else if (total[i] >= 40 && total[i]<=44) {
                grade[i] = 'E';
            }else {
                grade[i] = 'F';
            }
        }
        for (int i = 0; i < numOfStud; i++) {
            System.out.println("Reg_Num\tTest\tExam\tTotal\tGrade");
            System.out.println(regNo[i]+"\t"+test[i]+"\t\t"+exam[i]+"\t\t"+total[i]+"\t\t"+grade[i]);
        }


    }
}

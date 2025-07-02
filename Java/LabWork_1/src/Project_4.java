import java.util.Scanner;

public class Project_4 {
    public static void main(String[] args){
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter Student's Reg No.: ");
        String regNo = scanner.nextLine();

        System.out.print("Enter Student's Name: ");
        String name = scanner.nextLine();

        System.out.print("Enter Student's Year of Birth: ");
        int yearOfBirth = scanner.nextInt();
        scanner.nextLine()

        System.out.print("Enter Student's Department: ");
        String dept = scanner.nextLine();

        System.out.print("Enter Student's Test Score: ");
        int testScore = scanner.nextInt();

        System.out.print("Enter Student's Exam Score: ");
        int examScore = scanner.nextInt();

        int total = testScore + examScore;
        int age = 2025 - yearOfBirth;

        System.out.println("Reg no\tName\tage\tdepartment\ttotal");
        System.out.println(regNo+"\t"+name+"\t"+age+"\t"+dept+"\t"+total);
    }
}

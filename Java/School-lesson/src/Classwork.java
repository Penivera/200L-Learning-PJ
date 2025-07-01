import java.util.Scanner;
public class Classwork {
    static Scanner input = new Scanner(System.in);

    static int accept_input(String prompt){
        System.out.print(prompt);
        return input.nextInt();
    }
    static int calculate_total(int test, int exam){
        return test+exam;
    }
    static void display(int test,int exam,int total){
        System.out.println("Exams\t Test\t Total\t");
        System.out.printf("%d\t\t  %d\t\t%d",exam,test,total);
    }
    public static void main(String[] args){
        int test_score = accept_input("Enter Test Score: ");
        int exam_score = accept_input("Enter Exam Score: ");
        int total = calculate_total(test_score,exam_score);
        display(test_score,exam_score,total);

    }
}

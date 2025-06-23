import java.util.Scanner;
public class ControlStructures {
    static Scanner input = new Scanner(System.in);
     public static void main(String[] args){
         double SumInt = 0;
         /*
         for(int i=10 ;i > 0;i--){
             //System.out.println(i);
             SumInt = SumInt + i;
         }
         System.out.println(SumInt);*/
         for (int i = 0 ;i < 4 ;i++){
             System.out.printf("Enter Your Score for Course %d: ",i+1);
             int score = input.nextInt();
             SumInt = score + SumInt;
         }
         double student_average =  SumInt/4;
         if (student_average >=200){
             System.out.println("You have been Accepted\nYour average is "+SumInt);
         }else{
             System.out.println("You have been rejected\nYour Average is "+SumInt);
         }



     }
}

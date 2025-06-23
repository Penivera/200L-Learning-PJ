package practice;

import java.util.Arrays;
import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
        int i ;
        Scanner input = new Scanner(System.in);
        System.out.println("Welcome to student test Result Sheet");
        System.out.println("Enter Total number of Students");
        int numStudent = input.nextInt();
        for (i=0; i < numStudent; i++ ){
            System.out.printf("Enter Name of Student %d:\n" , i+1);
            String name = input.nextLine();
            input.nextLine();



        }



    }
}
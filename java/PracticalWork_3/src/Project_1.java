/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */

import java.util.Scanner;

/**
 *
 * @author peniel
 */
public class Project_1 {
    public static void real_solutions_to_quadratic(int a,int b,int c){
        int numerator = b*b - 4*a*c;
        if (numerator<0){
            System.out.println("Real solutions cannot be found");
        }else{
            double root_using_plus = ((-1*b) + Math.sqrt(numerator))/(2*a);
            double root_using_minus = ((-1*b) - Math.sqrt(numerator))/(2*a);
            System.out.printf("X = %f\nor\n",root_using_plus);
            System.out.printf("X = %f\n",root_using_minus);

        }
    }
    public static void main(String[] args){
        Scanner input = new Scanner(System.in);
        System.out.print("Enter a: "); int a = input.nextInt();
        System.out.print("Enter b: "); int b =input.nextInt();
        System.out.print("Enter c: ");int c= input.nextInt();
        real_solutions_to_quadratic(a,b,c);
    }
}

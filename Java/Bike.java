import java.lang.reflect.Array;
import java.util.Arrays;

public class Bike {

    static int sum(int... a) {
        int SumA = 0;
        for (int i : a) {
            SumA = SumA + i;
        }
        return SumA;
    }


    static int mul(int... a) {
        int MulA = 1;
        for (int j : a) {
            MulA = MulA * j;
        }
        return MulA;
    }
    static int sub(int... a){
        int SubA = 0;
        for(int k:a){
            SubA = k-SubA;
        }
        return SubA;
    }
    static double div(double... args){
        double result = args[0];
        for(int idx =1;idx<args.length;idx++){
            result/= args[idx];

        }
        return result;
    }
    static double recur_div(double... args) {
        if (args.length < 2) {
            return args[0];
        }


        // Base case: if only two numbers, divide them
        if (args.length == 2) {
            return args[0] / args[1];
        }

        // Recursively divide
        double first = args[0];
        double second = args[1];
        double result = first / second;

        // Create new args with result followed by the rest
        double[] newArgs = new double[args.length - 1];
        newArgs[0] = result;
        System.arraycopy(args, 2, newArgs, 1, args.length - 2);

        return recur_div(newArgs);
    }



    public static void main(String[] args) {
        System.out.println(sum(20,20,20,20,20,20));
        System.out.println(mul(50,20,40,30));
        System.out.println(sub(100-70-20-5));
        System.out.println(div(2,3));
        System.out.println(recur_div(1,2,3,4,5,6,2,3,4,89,9656,344656,6434,34));

    }
}
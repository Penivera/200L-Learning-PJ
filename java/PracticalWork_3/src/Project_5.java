/**
 *
 * @author penivera
 */
import java.util.Random;
public class Project_5 {
    public double sum_array(double[] arr){
        double sum = 0;
        for (double num : arr) {
            sum += num;
        }
        return sum;
    }

    public double max_array_value(double[] arr){
        double max = arr[0];
        for (double num : arr) {
            if (num > max) {
                max = num;
            }
        }
        return max;
    }

    public double min_array_value(double[] arr){
        double min = arr[0];
        for (double num:arr){
            if(num < min){
                min = num;
            }
        }
        return min;
    }
    public static void main(String[] args){
        //method to generate random numbers
        int arraySize = 10; // Desired size of the array
        int min = 1;        // Minimum value for random numbers (inclusive)

        double[] arr = new double[arraySize];
        Random random = new Random();

        for (int i = 0; i < arraySize; i++) {
            // Generate a random integer between min and max (inclusive)
            arr[i] = random.nextDouble(100) + min;
        }
        Project_5 project = new Project_5();
        System.out.println(project.sum_array(arr));
        System.out.println(project.max_array_value(arr));
        System.out.println(project.min_array_value(arr));
    }


}

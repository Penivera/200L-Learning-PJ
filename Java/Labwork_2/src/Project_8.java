import java.util.Arrays;

public class Project_8 {
    public static void main(String[] args){
        double sigma = 0;
        double sum = 0;
        int[] x = {26,18,54,37,45,34,23,65,29,63,56,22,41,62,50,30};
        int n = x.length;
        double Xmean = Arrays.stream(x).sum()/n;

        for (int i = 0; i < n; i++) {
            sum = sum + Math.pow((x[i]-Xmean),2);
            sigma = Math.sqrt(sum/n-1);
        }
        System.out.println("sigma = "+sigma);
    }

}
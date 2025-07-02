public class Project_7i {
    public static void main(String[] args){
        int A1 =1;
        double lamda = 0.5;
        double SumA = 0;
        for (int i = 1; i <= 30; i++) {
            double Ak = A1 * Math.pow(lamda,i-1);
            SumA = SumA + Ak;
        }

        System.out.printf("%.2f",SumA);
    }
}

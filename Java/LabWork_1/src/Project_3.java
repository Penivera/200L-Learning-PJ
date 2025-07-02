public class Project_3 {
    static double side_a = 20;
    static double side_b = 27.45;
    static int side_c = 18;
    public static void main(String[] args) {
        double s = (side_a + side_b + side_c)/2;
        double ScaleneArea = Math.sqrt(s*(s-side_a)*(s-side_b)*(s-side_c));
        System.out.printf("%.3f is the area of the scalene triangle\n",ScaleneArea);
    }
}

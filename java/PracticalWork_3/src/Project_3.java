

/**
 *
 * @author peniel
 */
public class Project_3 {
    public static int recursive_fibonacci(int n){
        if (n<=1){
            return n;
        }
        return recursive_fibonacci(n-1) + recursive_fibonacci(n-2);

    }
    public static void main(String[] args) {
        for (int i = 0;i<=15;i++){
            System.out.println(recursive_fibonacci(i)+ " ");
        }
    }

}

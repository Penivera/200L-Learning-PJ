import java.util.Scanner;

public class Project_3 {
    public static void main(String[] args){
        Scanner scanner = new Scanner(System.in);
        String user_in;
        boolean isVowel = false;
        boolean isLetter = false;
        char[] vowels = {'A','E','I','O','U'};
        char[] alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".toCharArray();

        System.out.print("Enter You name: ");
        user_in = scanner.nextLine();
        char[] user_name = user_in.to_CharArray();
        for (int i=0;i<alphabets.length;i++){
        }
        for(int i= 0;i<user_name.length;i++){
            for(int k = 0;k<vowels.length;k++){
                if (user_name[i]==vowels[i]){
                    System.out.printf("%s is a vowel\n",user_name[i]);
                }else{
                    System.out.printf("%s is a consonant\n",user_name[i]);
                }
            }
        }
    }
}

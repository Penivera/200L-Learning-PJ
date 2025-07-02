import java.util.Scanner;

public class Project_3 {
    public static void main(String[] args){
        Scanner scanner = new Scanner(System.in);
        char user_in;
        boolean isVowel = false;
        boolean isLetter = false;
        char[] vowels = {'A','E','I','O','U'};
        String alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        char[] letters = alphabets.toCharArray();

        System.out.print("Enter any character to detect vowel and consonant: ");
        user_in = scanner.next().toUpperCase().charAt(0);

        for (int i = 0; i < letters.length; i++) {
            if (user_in == letters[i]) {
                isLetter = true;
                for (int j = 0; j < vowels.length; j++) {
                    if(user_in == vowels[j]){
                        isVowel = true;
                        break;
                    }else {
                        isVowel = false;
                    }
                }
                break;
            }else {
                isLetter = false;
            }
        }

        if (isLetter){
            if (isVowel){
                System.out.println(user_in + " is a vowel");
            }else {
                System.out.println(user_in+" is a consonant");
            }
        }else {
            System.out.println(user_in+" is not an alphabet");
        }
    }
}

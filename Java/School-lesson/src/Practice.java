import java.util.Scanner;

class Data{
    static Scanner sc = new Scanner(System.in);
    private String name;
    private  int age;
    private int level;

    public void collect_inst_val(){
        System.out.println("Enter your name: ");
        name = sc.nextLine();
        System.out.println("Enter your age: ");
        age = sc.nextInt();
        System.out.println("Enter your level: ");
        level = sc.nextInt();
    }
    public void output_info(){
        //System.out.printf("Name: %s, age: %d, Leve: %d",name,age,level);
        System.out.printf("Name\tage\tlevel\n%s\t%d\t%d",name,age,level);
    }

}

class Example extends Data{
    public void format_print(){
        collect_inst_val();
        output_info();


    }
}
public class Practice{
    public static void main(String[] args){
        Example smt = new Example();
        smt.format_print();

    }
}



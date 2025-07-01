class Student{
    String Name;
    char Gender;

    public Student(String Name, char Gender) {
        this.Name = Name;
        this.Gender = Gender;
    }
}

public class Main {

    public static void main(String[] args){
        Student student1 = new Student("Peniel",'M');
        System.out.println(student1.Name);

    }
}
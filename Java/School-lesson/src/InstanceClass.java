class Information{
    String Name ;
    int Balance;
    Information(String Name, int Balance){
        this.Name = Name;
        this.Balance = Balance;
    }
    Information(){
        Name = "Peter";
        Balance = 23;
    }

    void details(int dept){
        System.out.println("Name: "+ Name );
        System.out.println("Balance: " +Balance+dept);
    }

}
public class InstanceClass{

    public static void main(String[] args){
        Information info = new Information();
        info.details(2);

    }
}
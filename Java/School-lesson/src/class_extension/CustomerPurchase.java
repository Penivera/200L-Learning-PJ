import java.util.Scanner;

public class CustomerPurchase extends Customer {
    double total = 0;
    int itemNum;
    String[] items;
    double[] itemPrice;

    public void input_information(){
        Scanner input = new Scanner(System.in);
        get_details();
        System.out.print("Enter number of items purchased: ");
        itemNum = input.nextInt();
        input.nextLine();
        items = new String[itemNum];
        itemPrice = new double[itemNum];
        for (int i = 0;i<itemNum;i++){
            System.out.printf("Enter item %d: ",i+1);
            items[i] = input.nextLine();
            System.out.printf("Enter price for item %d: ",i+1);
            itemPrice[i] = input.nextDouble();
            input.nextLine();
            System.out.println();
            total+= itemPrice[i];
        }
    }
    public void display_information(){
        display_details();
        for(int i =0;i<itemNum;i++){
            System.out.printf("%s: %2f",items[i],itemPrice[i]);
        }
        System.out.printf("Total: %2f",total);
    }
}

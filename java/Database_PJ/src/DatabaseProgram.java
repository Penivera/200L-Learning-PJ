import java.sql.*;

public class DatabaseProgram {
    public static void main(String[] args){
        try{
            Connection c = DriverManager.getConnection("jdbc:postgresql://localhost:5432/STUDENT_DB","postgres","admin");
            System.out.println("Opened database successfully");
            Statement stmt = c.createStatement();
            //Create table
            String sql = "CREATE TABLE StudentInformation "+ "(\"RegistrationNo\"character(12),"+" "+"\"FIRSTNAME\" character(255), "+" "+"\"MIDDLENAME\"  character(255), "+"\"Department\" character(255) )";
            stmt.executeUpdate(sql);
            System.out.println("Table created successfully...");
            System.out.println(sql);
            stmt.close();
            try{
                c.close();
            }catch (SQLException se){
                se.printStackTrace();
            }
        }catch(Exception ee){
            System.out.println(ee);
        }
    }
}

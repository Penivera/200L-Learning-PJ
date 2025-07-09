interface Student{
    public void admission();
    public void result();
    public void clearance();
}

class Undergraduate implements Student{
    @Override
    public void admission(){
        System.out.println("Admitted");
    }
    @Override
    public void result(){
        System.out.println("Result");

    }
    @Override
    public void clearance(){
        System.out.println("Clearance");

    }
}

class PostGraduate implements Student{
    @Override
    public void admission(){
        System.out.println("Admitted");

    }
    @Override
    public void result(){
        System.out.println("Result");

    }
    @Override
    public void clearance(){
        System.out.println("Clearance");

    }
}

class PartTime implements Student{
    @Override
    public void admission(){
        System.out.println("Admitted");

    }
    @Override
    public void result(){
        System.out.println("Result");

    }
    
    @Override
    public void clearance(){
        System.out.println("Clearance");

    }
}

public class Interface {
    public static void main(String[] args){

    }
    
}
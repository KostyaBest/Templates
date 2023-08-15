
package test1;


public class Test1{
    
    static {
        System.loadLibrary("test1_Test1");
    }
   

    public Test1(){

    }
    public native double sum(double a, double b);
    public native double minus(double a,double b);
    public native double division(double a,double b);
    public native double multiply(double a,double b);
    public static void main(String[] args)
    {
    }
}



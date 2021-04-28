import java.util.Scanner;

class ProblemD {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);

        int el = scan.nextInt() / 10;
        int sc = scan.nextInt() / 4;


        if(el<=sc)
        System.out.println( el * 10);

        if(sc<el)
        System.out.println( sc * 10);
    }
}
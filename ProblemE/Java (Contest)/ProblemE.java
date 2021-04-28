import java.util.Scanner;

class ProblemE {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        int num = scan.nextInt();

        for (int i = 1; i < num; i++) {
            if(i % 3 == 0)
            if(i % 5 != 0)
            System.out.print("Fizz");

            if (i % 5 == 0)
                if(i % 3 != 0)
                System.out.print("Buzz");

            if(i % 3 != 0 && i % 5 != 0)
            System.out.print(i);

            if(i % 3 == 0 && i % 5 == 0)
            System.out.print("Fizz Buzz");

            if(i % 10 == 0) {
            System.out.print(",");
            System.out.print("\n");
            } else {
                System.out.print(", ");
            }

            }

        int i = num;
        if(i % 3 == 0)
            if(i % 5 != 0)
            System.out.print("Fizz.");

            if (i % 5 == 0)
                if(i % 3 != 0)
                System.out.print("Buzz.");

            if(i % 3 != 0 && i % 5 != 0)
            System.out.print(i + ".");

            if(i % 3 == 0 && i % 5 == 0)
            System.out.print("Fizz Buzz.");
        
        System.out.println();
    }
}
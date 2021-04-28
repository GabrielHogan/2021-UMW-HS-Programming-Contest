import java.util.Scanner;

public class ProblemC {
  public static void main(String[] args){
    Scanner scanner = new Scanner(System.in);
    String input = scanner.nextLine();
    String inputb = scanner.nextLine();
    if (input.length() > inputb.length() ) {
      System.out.println(input);
    } else if (input.length() < inputb.length() ) {
      System.out.println(inputb);
    } else if (input.length() == inputb.length() ) {
      System.out.println("TIE");
    }
  }
}

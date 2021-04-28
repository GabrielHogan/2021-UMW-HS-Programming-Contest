import java.util.Scanner;

public class ProblemF {
  public static void main(String[] args){
    Scanner scanner = new Scanner(System.in);

    int goldTotal = scanner.nextInt();
    int goldTotalInt = goldTotal;

    int[] cost = {0, 0, 0, 0, 0};
    for (int x = 0; x < 5; x++){
      int costInput = scanner.nextInt();
      cost[x] = (costInput);
    }
    int[] onboard = {0, 0, 0, 0, 0};
    for (int x = 0; x < 5; x++){
      int onboardInput = scanner.nextInt();
      onboard[x] = (onboardInput);
    }
    int[] needed = {0, 0, 0, 0, 0};
    for (int x = 0; x < 5; x++){
      int neededInput = scanner.nextInt();
      needed[x] = (neededInput);
    }
    int[] totalNeeded = {0, 0, 0, 0, 0};
    for (int x = 0; x < 5; x++){
      totalNeeded[x] = needed[x] - onboard[x];
    }
    int[] individualCost = {0, 0, 0, 0, 0};
    for (int x = 0; x < 5; x++){
      individualCost[x] = totalNeeded[x] * cost[x];
    }
    int sum = 0;
    for (int x = 0; x < 5; x++){
      sum = sum + individualCost[x];
    }

    //int final = (goldTotalInt - sum);
    if (goldTotalInt >= sum){
      System.out.println(goldTotalInt - sum);
    }else if (goldTotalInt < sum){
      System.out.println("Captain, I need more gold pieces.");
    }
  }
}

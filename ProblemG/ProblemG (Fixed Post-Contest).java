// This was fixed by Mr. Arnold Post Contest.

import java.util.Scanner;

class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int num = scan.nextInt();
    int[][] arr = new int[num][num];
    int stop = num * num;

    for (int row = 0; row < arr.length; row++)
    {
      for (int column = 0; column < arr[row].length; column++)
      {
        arr[row][column] = scan.nextInt();
      }
    }


    int toplevel = 0;
    int rightlevel = num - 1;
    int botlevel = num - 1;
    int leftlevel = 0;

    while(stop > 0) 
    {
      
      if(stop > 0){
        for (int k = leftlevel; k <= rightlevel; k++) {
          System.out.print(arr[toplevel][k] + " ");
          stop--;
        }
        toplevel++;
      }
     
      if(stop > 0){
        for (int b = toplevel; b <= botlevel; b++) {
          System.out.print(arr[b][rightlevel] + " ");
          stop--;
        }
        rightlevel--;
      }

      if(stop > 0){ 
        for (int t = rightlevel; t >= leftlevel; t--) {
          System.out.print(arr[botlevel][t] + " ");
          stop--;
        }
        botlevel--;
      }
      
      if(stop > 0){
        for (int m = botlevel; m >= toplevel; m--) {
          System.out.print(arr[m][leftlevel] + " ");
          stop--;
        }
        leftlevel++;
      }
     
    }
    System.out.println();
  }
}

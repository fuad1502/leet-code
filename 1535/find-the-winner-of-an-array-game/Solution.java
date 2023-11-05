/*
 * Notice that when arr[i] is first evaluated, it is evaluated with the maximum
 * of arr[0:i-1] If it is smaller, then it arr[i] will get 0 points If it is
 * larger, it will get (j-i) points, where j is the first element larger than
 * arr[i] If that point is >= k, then arr[i] is the winner, if not, we start
 * evaluating arr[j] If we get to the end of arr, yet, arr.length - 1 - i is not
 * >= k, we still returns arr[i]
 */
class Solution {
  public int getWinner(int[] arr, int k) {
    // Initialize the first evaluation index
    // Give a bonus point of one if we evaluate from arr[1]
    int evaluateIdx;
    int notFirstBonus;
    if (arr[0] > arr[1]) {
      evaluateIdx = 0;
      notFirstBonus = 0;
    } else {
      evaluateIdx = 1;
      notFirstBonus = 1;
    }
    int checkIdx = evaluateIdx + 1;
    // Evaluation loop
    while (!((checkIdx - evaluateIdx - 1 + notFirstBonus) >= k) &&
           (checkIdx < arr.length)) {
      if (arr[checkIdx] < arr[evaluateIdx]) {
        checkIdx++;
      } else {
        evaluateIdx = checkIdx;
        notFirstBonus = 1;
        checkIdx = evaluateIdx + 1;
      }
    }
    return arr[evaluateIdx];
  }

  public static void main(String[] args) {
    Solution s = new Solution();
    // int[] arr = {2, 1, 3, 5, 4, 6, 7};
    // int k = 2;
    int[] arr = {3, 2, 1};
    int k = 10;
    System.out.println(s.getWinner(arr, k));
  }
}

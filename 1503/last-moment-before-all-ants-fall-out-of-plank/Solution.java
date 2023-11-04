class Solution {
  public int getLastMoment(int n, int[] left, int[] right) {
    int maximum_left = 0;
    if (left.length != 0) {
      for (int position : left) {
        if (position > maximum_left) {
          maximum_left = position;
        }
      }
    }
    int minimum_right = n;
    if (right.length != 0) {
      for (int ant_position : right) {
        if (ant_position < minimum_right) {
          minimum_right = ant_position;
        }
      }
    }
    return Math.max(n - minimum_right, maximum_left);
  }

  public static void main(String[] args) {
    Solution s = new Solution();
    // int n = 4;
    // int[] left = {4, 3};
    // int[] right = {0, 1};
    int n = 7;
    int[] left = {};
    int[] right = {0, 1, 2, 3, 4, 5, 6, 7};
    System.out.println(s.getLastMoment(n, left, right));
  }
}

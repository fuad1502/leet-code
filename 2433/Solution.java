class Solution {
  public int[] findArray(int[] pref) {
    int[] res = new int[pref.length];
    int cumulativeXor = 0;
    int i = 0;
    for (int p : pref) {
      res[i] = p ^ cumulativeXor;
      cumulativeXor ^= res[i];
      i += 1;
    }
    return res;
  }

  public static void main(String[] args) {
    Solution s = new Solution();
    int[] pref = {5, 2, 0, 3, 1};
    int[] res = s.findArray(pref);
    for (int v : res) {
      System.out.println(v);
    }
  }
}

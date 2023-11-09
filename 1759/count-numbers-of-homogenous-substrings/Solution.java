class Solution {
  static final int MOD = 1000000007;

  public int countHomogenous(String s) {
    int count = 0;
    int startIdx = 0;
    int endIdx = 1;
    while (endIdx < s.length()) {
      if (s.charAt(startIdx) == s.charAt(endIdx)) {
        endIdx += 1;
      } else {
        int currentLength = endIdx - startIdx;
        int currentCount = calculateCount(currentLength);
        count = (count + (currentCount % MOD)) % MOD;
        startIdx = endIdx;
        endIdx = startIdx + 1;
      }
    }
    // Add last homogenous substring
    int currentLength = endIdx - startIdx;
    int currentCount = calculateCount(currentLength);
    count = (count + (currentCount % MOD)) % MOD;

    return count;
  }

  public int calculateCount(int length) {
    long t1 = length;
    long t2 = length + 1;
    long t3 = (t1 * t2) / 2;
    int count = (int)(t3 % (long)MOD);
    return count;
  }

  public static void main(String[] args) {
    Solution solution = new Solution();
    String s = "abbcccaa";
    assert solution.countHomogenous(s) == 13;
    assert solution.calculateCount(100000) == 49965;
  }
}

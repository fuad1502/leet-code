import java.util.HashMap;

public class Solution {
  public static void main(String[] args) {
    Solution s = new Solution();
    // int[] nums = {1, 2, 1, 2, 1};
    // int[] nums = {1, 2, 3};
    // int[] nums = {-1, -1, 1};
    // int[] nums = {-92, -63, 75, -86, -58, 22, 31, -16, -66, -67, 420};
    int[] nums = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    int k = 0;
    System.out.println(s.subarraySum(nums, k));
  }

  public int subarraySum(int[] nums, int k) {
    int result = 0;
    HashMap<Integer, Integer> prefixSums = new HashMap<Integer, Integer>();
    int prefixSum = 0;
    for (int num : nums) {
      prefixSum += num;
      // Check the prefix sum itself
      if (prefixSum == k) {
        result += 1;
      }
      // Find corresponding pair
      if (prefixSums.containsKey(prefixSum - k)) {
        result += prefixSums.get(prefixSum - k);
      }
      // Add prefix sum to Hash Map
      if (prefixSums.containsKey(prefixSum)) {
        prefixSums.put(prefixSum, prefixSums.get(prefixSum) + 1);
      } else {
        prefixSums.put(prefixSum, 1);
      }
    }
    return result;
  }
}

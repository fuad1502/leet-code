#include <algorithm>
#include <cmath>
#include <iostream>
#include <iterator>
#include <vector>

using namespace std;

class Solution {
private:
  constexpr static int mod_val = 1000000007;

public:
  int numFactoredBinaryTrees(vector<int> &arr) {
    // Sort array, the algorithm only works with the array sorted
    sort(arr.begin(), arr.end());

    // Stores the number of trees that can be constructed with arr[i] as root
    // Initialize with 1 (a tree that constitues the root itself)
    vector<int> n_trees(arr.size(), 1);

    for (int i = 0; i < arr.size(); i++) {
      // To optimize finding the factor
      auto max_factor = sqrt(arr[i]);
      auto max_factor_idx = std::distance(
          arr.begin(), upper_bound(arr.begin(), arr.end(), max_factor));

      // Find the factors
      for (int j = 0; j < max_factor_idx; j++) {
        // Check if divisible
        if (arr[i] % arr[j] == 0) {
          auto it = lower_bound(arr.begin() + max_factor_idx - 1, arr.end(),
                                arr[i] / arr[j]);
          // Check if dividend is in arr
          if (*it == arr[i] / arr[j]) {
            auto k = 1;
            // Scale by two if the divisor != dividend
            if (arr[i] / arr[j] != arr[j])
              k = 2;

            // The number of trees that can be formed with this root is equal
            // to the product of the number of trees that can be formed by
            // the factors as root
            n_trees[i] +=
                (k * n_trees[j] * n_trees[std::distance(arr.begin(), it)]) %
                mod_val;
          }
        }
      }
    }

    long result = 0;
    for (auto n_tree : n_trees) {
      result = (result + n_tree) % mod_val;
    }

    return result;
  }
};

int main(int argc, char *argv[]) {
  vector<int> arr{2, 4, 5, 10};
  Solution solution;
  cout << solution.numFactoredBinaryTrees(arr) << endl;
  return 0;
}

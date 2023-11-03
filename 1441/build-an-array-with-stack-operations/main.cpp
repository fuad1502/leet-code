#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
  vector<string> buildArray(vector<int> &target, int n) {
    vector<string> result;
    int target_idx = 0;
    for (int i = 1; i <= n; i++) {
      if (target[target_idx] == i) {
        result.push_back("Push");
        target_idx++;
        if (target_idx == target.size()) {
          break;
        }
      } else {
        result.push_back("Push");
        result.push_back("Pop");
      }
    }
    return result;
  }
};

int main(int argc, char *argv[]) {
  Solution s;
  vector<int> target{1, 3};
  int n = 3;
  auto result = s.buildArray(target, n);
  vector<string> expected{"Push", "Push", "Pop", "Push"};
  if(result != expected) {
    cout << "Wrong result" << endl;
    return 1;
  }
  target = {1, 2, 3};
  n = 3;
  result = s.buildArray(target, n);
  expected = {"Push", "Push", "Push"};
  if(result != expected) {
    cout << "Wrong result" << endl;
    return 1;
  }
  target = {1, 2};
  n = 4;
  result = s.buildArray(target, n);
  expected = {"Push", "Push"};
  if(result != expected) {
    cout << "Wrong result" << endl;
    return 1;
  }
  return 0;
}

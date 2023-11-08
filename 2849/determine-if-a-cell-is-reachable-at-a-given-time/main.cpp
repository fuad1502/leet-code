/*
 * The solution to this problem is relatively simple, we just need to know the
 * minimum time required to reach the end point. The reason is that there is no
 * restriction on moving to a previously traversed cell. Therefore, once we
 * arrive at an adjacent cell to the end point, we can just delay our reach time
 * by going around the destination point.
 *
 * Now, to determine the minimum time required to reach the end point, notice
 * that each move would only move us in x and / or y direction by 1. Therefore,
 * the fastest path is to move in diagonal until we are at the same x or y
 * position as the end point, and then we simply move in the direction of the
 * end point.
 *
 * Interestingly, this simply results to the maximum of the width and length.
 *
 * The only special case is when we're already at the end point. We cannot reach
 * the end point in exactly 1 step. Other then that, including 0, it's possible.
 */
#include <algorithm>
#include <cassert>
#include <cstdlib>

class Solution {
public:
  bool isReachableAtTime(int sx, int sy, int fx, int fy, int t) {
    auto width = abs(fy - sy);
    auto length = abs(fx - sx);
    auto minimum_t = std::max(width, length);
    if (minimum_t == 0) {
      return t != 1;
    } else {
      return t >= minimum_t;
    }
  }
};

int main(int argc, char *argv[]) {
  Solution s;
  assert((s.isReachableAtTime(2, 4, 7, 7, 6) == true));
  assert((s.isReachableAtTime(3, 1, 7, 3, 3) == false));
  assert((s.isReachableAtTime(3, 1, 3, 1, 1) == false));
  return 0;
}

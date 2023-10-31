#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
  static const long hashSize = 0x8000000000000000;

  long modpow2(long exp, long modulo) {
    if (exp > 63) {
      return (modpow2(exp / 2, modulo) * modpow2(exp - exp / 2, modulo)) %
             modulo;
    } else {
      return (1L << exp) % modulo;
    }
  }

  long strHash(const string &s) {
    long result = 0;
    for (int i = 0; i < s.size(); i++) {
      result =
          (result + (modpow2(i, hashSize) * s[s.size() - i - 1]) % hashSize) %
          hashSize;
    }
    return result;
  }

  long calculateNewSubstringHash(long previousHash, char droppedChar,
                                 char appendedChar, long stringLength) {
    return (((previousHash -
              (droppedChar * modpow2(stringLength - 1, hashSize)) % hashSize) %
             hashSize) *
            2) %
               hashSize +
           appendedChar;
  }

  bool compareSubstring(const string &needle, const string &haystack,
                        int compareIdx) {
    for (int i = 0; i < needle.size(); i++) {
      if (needle[i] != haystack[compareIdx + i]) {
        return false;
      }
    }
    return true;
  }

public:
  int strStr(string haystack, string needle) {
    if (needle.size() > haystack.size()) {
      return -1;
    }
    long needleHash = strHash(needle);
    long substringHash = strHash(haystack.substr(0, needle.size()));
    if (substringHash == needleHash && compareSubstring(needle, haystack, 0)) {
      return 0;
    }
    for (int i = 1; i < haystack.size() - needle.size() + 1; i++) {
      substringHash = calculateNewSubstringHash(substringHash, haystack[i - 1],
                                                haystack[i + needle.size() - 1],
                                                needle.size());
      if (substringHash == needleHash &&
          compareSubstring(needle, haystack, i)) {
        return i;
      }
    }
    return -1;
  }
};

int main(int argc, char *argv[]) {
  Solution s;

  cout << s.strStr("sadbutsad", "sad") << endl;
  cout << s.strStr("sadbutsad", "but") << endl;
  cout << s.strStr("sadbutsad", "sbut") << endl;
  cout << s.strStr("abb", "abaaaa") << endl;
  cout << s.strStr("fourscoreandsevenyearsagoourfathersbroughtforthuponthiscont"
                   "inentanewnation",
                   "ourfathersbroughtforthuponthiscontinentanewnation")
       << endl;

  return 0;
}

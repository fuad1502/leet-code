import java.util.ArrayList;
import java.util.HashMap;

/*
To solve this problem, we first need to form the 2-letter tree for each vowel as
the root, e.g. a->(e) e->(a,i) i->(a,e,o,u) o->(i,u) u->(a) We also store the
two letter word combination beginning with each vowel: a->1 e->2 i->4 o->2 u->1
If we were only asked for two letter words, our answer is simply the sum of
these, which is 10. To form three letter word, we simply sum the two letter word
combination of the root child: a->(a,i)->2 e->(e,a,e,o,u)->5 i->(e,a,i,i,u,a)->6
As an after thought, I think it's better if we divide the problem by two each
step -> n = 5 -> 1, 4 -> 2, 2
*/

class Solution {
  private static final long MOD_VAL = 1000000007;

  private class VowelPermutation {
    enum Vowel { A, E, I, O, U }

    private class VowelWord {
      HashMap<Vowel, Long> endings = new HashMap<Vowel, Long>();

      VowelWord() {}

      VowelWord(Vowel vowel) {
        switch (vowel) {
        case A:
          endings.put(Vowel.E, 1l);
          break;
        case E:
          endings.put(Vowel.A, 1l);
          endings.put(Vowel.I, 1l);
          break;
        case I:
          endings.put(Vowel.A, 1l);
          endings.put(Vowel.E, 1l);
          endings.put(Vowel.O, 1l);
          endings.put(Vowel.U, 1l);
          break;
        case O:
          endings.put(Vowel.I, 1l);
          endings.put(Vowel.U, 1l);
          break;
        case U:
          endings.put(Vowel.A, 1l);
          break;
        }
      }

      public VowelWord copy() {
        VowelWord vowelWord = new VowelWord();
        for(Vowel v : Vowel.values()) {
          if(this.endings.containsKey(v)) {
            vowelWord.endings.put(v, this.endings.get(v));
          }
        }
        return vowelWord;
      }

      void setEndings(HashMap<Vowel, Long> endings) { this.endings = endings; }

      long getEndingValue(Vowel vowel) {
        if (endings.containsKey(vowel)) {
          return endings.get(vowel);
        } else {
          return 0;
        }
      }

      long getPermutation() {
        long permutation = 0;
        for (Vowel vowel : Vowel.values()) {
          permutation = (permutation + getEndingValue(vowel)) % MOD_VAL;
        }
        return permutation;
      }
    };

    private ArrayList<VowelWord> rootWords = new ArrayList<VowelWord>(5);

    VowelPermutation() {
      rootWords.add(new VowelWord(Vowel.A));
      rootWords.add(new VowelWord(Vowel.E));
      rootWords.add(new VowelWord(Vowel.I));
      rootWords.add(new VowelWord(Vowel.O));
      rootWords.add(new VowelWord(Vowel.U));
    }

    public VowelPermutation copy() {
      VowelPermutation vowelPermutation = new VowelPermutation();
      for (int i = 0; i < 5; i++) {
        vowelPermutation.rootWords.set(
            i, this.rootWords.get(i).copy());
      }
      return vowelPermutation;
    }

    void combine(VowelPermutation secondPermutation) {
      for (VowelWord rootWord : rootWords) {
        HashMap<Vowel, Long> newEndings = new HashMap<Vowel, Long>();
        for (Vowel secondWordEndVowel : Vowel.values()) {
          long newValue = 0;
          for (Vowel rootWordEndVowel : Vowel.values()) {
            newValue = (newValue + (rootWord.getEndingValue(rootWordEndVowel) *
                                    secondPermutation.getRoot(rootWordEndVowel)
                                        .getEndingValue(secondWordEndVowel)) %
                                       MOD_VAL) %
                       MOD_VAL;
          }
          newEndings.put(secondWordEndVowel, newValue);
        }
        rootWord.setEndings(newEndings);
      }
    }

    VowelWord getRoot(Vowel vowel) {
      switch (vowel) {
      case A:
        return rootWords.get(0);
      case E:
        return rootWords.get(1);
      case I:
        return rootWords.get(2);
      case O:
        return rootWords.get(3);
      case U:
        return rootWords.get(4);
      default:
        return rootWords.get(0);
      }
    }

    long getAllPermutation() {
      long totalPermutation = 0;
      for (VowelWord rootWord : rootWords) {
        totalPermutation =
            (totalPermutation + rootWord.getPermutation()) % MOD_VAL;
      }
      return totalPermutation;
    }
  }

  private HashMap<Integer, VowelPermutation> existingPermutation =
      new HashMap<Integer, VowelPermutation>();

  public int countVowelPermutation(int n) {
    if (n == 0) {
      return 0;
    }
    if (n == 1) {
      return 5;
    }
    try {
      return (int)createVowelPermutations(n).getAllPermutation();
    } catch (Exception e) {
      return 0;
    }
  }

  private VowelPermutation createVowelPermutations(int n)
      throws CloneNotSupportedException {
    if (existingPermutation.containsKey(n)) {
      VowelPermutation vowelPermutation =
          (Solution.VowelPermutation)existingPermutation.get(n).copy();
      return vowelPermutation;
    } else if (n == 2) {
      VowelPermutation vowelPermutation = new VowelPermutation();
      existingPermutation.put(n, vowelPermutation.copy());
      return vowelPermutation;
    } else {
      VowelPermutation firstPermutation = createVowelPermutations(n / 2 + 1);
      VowelPermutation secondPermutation;
      if (n % 2 == 0) {
        secondPermutation = createVowelPermutations(n / 2);
      } else {
        secondPermutation = createVowelPermutations(n / 2 + 1);
      }
      firstPermutation.combine(secondPermutation);
      existingPermutation.put(n, firstPermutation.copy());
      return firstPermutation;
    }
  }

  public static void main(String[] args) {
    Solution solution = new Solution();
    System.out.println(solution.countVowelPermutation(0));
    System.out.println(solution.countVowelPermutation(1));
    System.out.println(solution.countVowelPermutation(2));
    System.out.println(solution.countVowelPermutation(5));
    System.out.println(solution.countVowelPermutation(144));
  }
}

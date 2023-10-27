struct Solution {}

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowels = ['a', 'i', 'u', 'e', 'o'];
        let words = sentence.split(" ");
        let mut result = String::new();
        let mut additional_a = String::from("a");
        for word in words {
            let first_letter = word
                .chars()
                .next()
                .expect("The words should have at least one letter");
            let a_vowel = vowels.iter().find(|&&c| c == first_letter.to_ascii_lowercase());
            let transformed_word = match a_vowel {
                Some(_) => word.to_string() + "ma",
                None => word[1..].to_string() + &first_letter.to_string() + "ma",
            };
            result = result + &transformed_word + &additional_a + " ";
            additional_a += "a";
        }
        result.pop();
        result
    }
}

fn main() {
    let sentence = "I speak Goat Latin";
    println!("{}", Solution::to_goat_latin(sentence.to_string()));
}

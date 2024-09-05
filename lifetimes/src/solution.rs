use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Difference<'a, 'b> {
    first_only: Vec<&'a str>,
    second_only: Vec<&'b str>,
}

pub fn words(sentence: &str) -> HashSet<&str> {
    sentence
        .split(' ')
        .filter(|word| !word.is_empty()) // this makes sure we don't have empty words
        .collect()
}

pub fn find_difference<'a, 'b>(sentence1: &'a str, sentence2: &'b str) -> Difference<'a, 'b> {
    // lifetime annotations here are not required, added for clarity
    let sentence_1_words: HashSet<&'a str> = words(sentence1);
    let sentence_2_words: HashSet<&'b str> = words(sentence2);

    let mut diff = Difference::default();

    for word in &sentence_1_words {
        if !sentence_2_words.contains(word) {
            diff.first_only.push(word)
        }
    }

    for word in &sentence_2_words {
        if !sentence_1_words.contains(word) {
            diff.second_only.push(word)
        }
    }

    // sorting simplifies the equality assertions below
    diff.first_only.sort();
    diff.second_only.sort();

    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completely_disjoint_sentences() {
        let sentence1 = "apple banana cherry";
        let sentence2 = "dog elephant fox";
        let result = find_difference(sentence1, sentence2);

        assert_eq!(result.first_only, vec!["apple", "banana", "cherry"]);
        assert_eq!(result.second_only, vec!["dog", "elephant", "fox"]);
    }

    #[test]
    fn test_identical_sentences() {
        let sentence1 = "apple banana cherry";
        let sentence2 = "apple banana cherry";
        let result = find_difference(sentence1, sentence2);

        assert!(result.first_only.is_empty());
        assert!(result.second_only.is_empty());
    }

    #[test]
    fn test_some_common_words() {
        let sentence1 = "apple banana cherry";
        let sentence2 = "banana cherry dog";
        let result = find_difference(sentence1, sentence2);

        assert_eq!(result.first_only, vec!["apple"]);
        assert_eq!(result.second_only, vec!["dog"]);
    }

    #[test]
    fn test_empty_sentences() {
        let sentence1 = "";
        let sentence2 = "";
        let result = find_difference(sentence1, sentence2);

        assert!(result.first_only.is_empty());
        assert!(result.second_only.is_empty());
    }

    #[test]
    fn test_one_empty_sentence() {
        let sentence1 = "apple banana cherry";
        let sentence2 = "";
        let result = find_difference(sentence1, sentence2);

        assert_eq!(result.first_only, vec!["apple", "banana", "cherry"]);
        assert!(result.second_only.is_empty());
    }

    #[test]
    fn test_drop_first_sentence_before_second() {
        let sentence1 = String::from("A long sentence that takes up a lot of memory. We want to drop it as soon as possible.");
        let sentence2 =
            String::from("A short sentence, we are ok with keeping around for a while.");
        let Difference {
            first_only,
            second_only,
        } = find_difference(&sentence1, &sentence2);

        #[rustfmt::skip]
        assert_eq!(
            first_only,
            vec! ["We", "as", "drop", "it", "long", "lot", "memory.", "of", "possible.", "sentence", "soon", "takes", "that", "to", "up", "want"],
        );
        drop(sentence1);

        #[rustfmt::skip]
        assert_eq!(
            second_only,
            vec! ["are", "around", "for", "keeping", "ok", "sentence,", "short", "we", "while.", "with"],
        );
    }
}

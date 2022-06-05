use std::collections::HashMap;

pub fn find_word_concatenation(string: String, words: Vec<String>) -> Vec<u32> {
    let mut indices: Vec<u32> = Vec::new();

    let mut frequency_map: HashMap<String, i32> = HashMap::new();

    let word_length = words[0].len();
    let mut end: u32 = 0;
    let mut start: u32 = 0;
    let mut matched = 0;
    let num_words = words.len();

    for word in words {
        frequency_map.insert(word.clone(), *frequency_map.get(&word).unwrap_or(&0) + 1);
    }
    print!("{:?}", frequency_map);

    while (end as usize) < string.len() {
        let mut move_start = false;
        let next_string = string
            .get((end as usize)..(end as usize) + word_length)
            .unwrap();
        if frequency_map.contains_key(next_string) {
            let frequency = *frequency_map.get(next_string).unwrap();
            if frequency == 0 {
                move_start = true;
            }
            frequency_map.insert(next_string.to_string(), frequency - 1);
            if frequency - 1 == 0 {
                matched += 1;
            }
        }
        print!("{:?}", frequency_map);

        if matched == num_words || move_start {
            if matched == num_words {
                indices.push(start);
            }
            let start_string = string
                .get((start as usize)..(start as usize) + word_length)
                .unwrap();
            print!("{}", start_string);
            if frequency_map.contains_key(start_string) {
                let frequency = *frequency_map.get(start_string).unwrap();
                frequency_map.insert(start_string.to_string(), frequency + 1);
                if frequency == 0 {
                    matched -= 1;
                }
            }
            start += word_length as u32;
        }
        end += word_length as u32;
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        // Input: String="catfoxcat", Words=["cat", "fox"]
        // Output: [0, 3]
        // Explanation: The two substring containing both the words are "catfox" & "foxcat".
        let answer = Vec::from([0, 3]);
        let test = find_word_concatenation(
            String::from("catfoxcat"),
            Vec::from(["cat".to_string(), "fox".to_string()]),
        );
        assert_eq!(test, answer);
    }

    #[test]
    fn test_two() {
        // Input: String="catcatfoxfox", Words=["cat", "fox"]
        // Output: [3]
        // Explanation: The only substring containing both the words is "catfox".
        let answer = Vec::from([3]);
        let test = find_word_concatenation(
            String::from("catcatfoxfox"),
            Vec::from(["cat".to_string(), "fox".to_string()]),
        );
        assert_eq!(test, answer);
    }
}

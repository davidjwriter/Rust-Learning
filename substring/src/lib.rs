use std::collections::HashMap;
use std::str;

pub fn find_substring(string: String, pattern: String) -> String {
    let mut frequency_map: HashMap<u8, i32> = HashMap::new();
    for c in pattern.as_bytes() {
        frequency_map.insert(*c, *frequency_map.get(c).unwrap_or(&0) + 1);
    }
    print!("{:?}", frequency_map);

    let mut matched = 0;
    let mut substring_start = 0;
    let mut min_length = string.len() + 1;
    let pattern_length = pattern.len();
    let mut start = 0;
    for end in 0..string.len() {
        let end_char = string.as_bytes()[end];

        if frequency_map.contains_key(&end_char) {
            // Key is in map, reduce and if we're still greater than 0, increment matched
            let frequency = *frequency_map.get(&end_char).unwrap();
            if frequency > 0 {
                matched += 1;
            }
            frequency_map.insert(end_char, frequency - 1);
        }

        // Move Window Until No Match
        while pattern_length == matched {
            if min_length > (end - start + 1) {
                min_length = end - start + 1;
                substring_start = start;
            }

            let start_char = string.as_bytes()[start];

            if frequency_map.contains_key(&start_char) {
                let frequency = *frequency_map.get(&start_char).unwrap();
                if frequency == 0 {
                    matched -= 1;
                }
                frequency_map.insert(start_char, frequency + 1);
            }
            start += 1;
        }
    }

    let mut answer = "".to_string();
    if string.len() >= min_length {
        let bytes = string.as_bytes();
        for b in substring_start..substring_start + min_length {
            answer += str::from_utf8(&[bytes[b]]).unwrap();
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        // Input: String="aabdec", Pattern="abc"
        // Output: "abdec"
        // Explanation: The smallest substring having all characters of the pattern is "abdec"
        let answer: String = String::from("abdec");
        let test: String = find_substring("aabdec".to_string(), "abc".to_string());
        assert!(answer.eq(&test));
    }

    #[test]
    fn test_two() {
        // Input: String="aabdec", Pattern="abac"
        // Output: "aabdec"
        // Explanation: The smallest substring having all character occurrences of the pattern is "aabdec"
        let answer: String = String::from("aabdec");
        let test = find_substring("aabdec".to_string(), "abac".to_string());
        assert!(answer.eq(&test));
    }

    #[test]
    fn test_three() {
        // Input: String="abdbca", Pattern="abc"
        // Output: "bca"
        // Explanation: The smallest substring having all characters of the pattern is "bca".
        let answer: String = String::from("bca");
        let test = find_substring("abdbca".to_string(), "abc".to_string());
        assert!(answer.eq(&test));
    }

    #[test]
    fn test_four() {
        // Input: String="adcad", Pattern="abc"
        // Output: ""
        // Explanation: No substring in the given string has all characters of the pattern.
        let answer: String = String::from("");
        let test = find_substring("adcad".to_string(), "abc".to_string());
        assert!(answer.eq(&test));
    }
}

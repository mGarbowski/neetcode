//! https://neetcode.io/problems/string-encode-and-decode

const SEPARATOR: &str = "#";

pub fn encode(strs: Vec<String>) -> String {
    let with_length: Vec<String> = strs.iter()
        .map(|str| format!("{}{SEPARATOR}{str}", str.len()))
        .collect();
    with_length.concat()
}

pub fn decode(s: String) -> Vec<String> {
    let mut strings = Vec::new();
    let mut start_idx = 0;

    while start_idx < s.len() {
        let unprocessed = &s[start_idx..];
        let separator_idx = unprocessed.find(SEPARATOR).unwrap();
        let length_str = &unprocessed[0..separator_idx];
        let length = length_str.parse::<usize>().unwrap();
        let end_idx = separator_idx + 1 + length;
        let extracted_str = unprocessed[separator_idx + 1..end_idx].to_string();
        strings.push(extracted_str);
        start_idx += end_idx;
    }

    strings
}


#[cfg(test)]
mod test {
    use crate::arrays_and_hashing::encode_decode_strings::*;

    #[test]
    fn encode_and_decode() {
        let strs = vec![
            "neet".to_string(),
            "code".to_string(),
            "love".to_string(),
            "you".to_string(),
        ];
        assert_eq!(strs, decode(encode(strs.clone())));
    }

    #[test]
    fn encode_and_decode_contains_separator() {
        let strs = vec![
            "Hello # world".to_string(),
            "#two".to_string(),
            "three#".to_string(),
        ];
        assert_eq!(strs, decode(encode(strs.clone())));
    }

    #[test]
    fn encode_and_decode_double_diigt_length() {
        let strs = vec![
            "neet".to_string(),
            "codecodecodecodecodecode".to_string(),
            "love".to_string(),
            "youyouyouyouyouyou".to_string(),
        ];
        assert_eq!(strs, decode(encode(strs.clone())));
    }
}
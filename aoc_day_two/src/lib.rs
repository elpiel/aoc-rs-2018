use std::collections::HashMap;

pub fn calculate_checksum(boxes_input: &Vec<String>) -> i32 {
    let (twice_count, trice_count) = boxes_input.iter().fold((0, 0), |(mut twice, mut trice), box_id| {
        let letters = box_id.chars().into_iter()
            .fold(HashMap::new(), |mut letters_map, box_id_char| {
                *letters_map.entry(box_id_char).or_insert(0) += 1;

                letters_map
            });

        twice += letters.values().find_map(|&count| match count {
            2 => Some(1),
            _ => None,
        }).unwrap_or(0);

        trice += letters.values().find_map(|&count| match count {
            3 => Some(1),
            _ => None,
        }).unwrap_or(0);

        (twice, trice)
    });

    twice_count * trice_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_checksum() {
        assert_eq!(calculate_checksum(&vec!["abcdef".to_owned()]), 0);
        assert_eq!(calculate_checksum(&vec!["aabbbc".to_owned()]), 1);
        assert_eq!(calculate_checksum(&vec!["aabbbc".to_owned(), "aabbbc".to_owned()]), 4);
        assert_eq!(calculate_checksum(&vec!["aaabbb".to_owned(), "aabbbc".to_owned()]), 2);
        let example_input = vec![
            "abcdef".to_owned(), "bababc".to_owned(), "abbcde".to_owned(), "abcccd".to_owned(),
            "aabcdd".to_owned(), "abcdee".to_owned(), "ababab".to_owned()];
        assert_eq!(calculate_checksum(&example_input), 12);
    }
}



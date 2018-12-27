use std::collections::HashMap;
use std::string::String;

pub fn calculate_checksum(boxes_input: &Vec<String>) -> i32 {
    let (twice_count, trice_count) = boxes_input.iter().fold((0, 0), |(mut twice, mut trice), box_id| {
        let letters = box_id.chars().into_iter()
            .fold(HashMap::new(), |mut letters_map, box_id_char| {
                *letters_map.entry(box_id_char).or_insert(0) += 1;

                letters_map
            });

        twice += letters.values().any(|&count| count == 2) as i32;
        trice += letters.values().any(|&count| count == 3) as i32;

        (twice, trice)
    });

    twice_count * trice_count
}

pub fn find_common_correct_box_id_part(boxes_input: &Vec<String>) -> String {

    for (index, box_id) in boxes_input.iter().enumerate() {
        let found = boxes_input.iter().skip(index+1).find_map(|searched_box_id| {

            let similar_chars = box_id.chars().enumerate().filter_map(|(index, box_char)| {
                if searched_box_id.chars().nth(index).unwrap() == box_char {
                    return Some(box_char);
                }
                None
            }).collect::<String>();

            match &similar_chars.chars().count() + 1 == box_id.len() {
                true => Some(similar_chars),
                false => None,
            }
        });

        if found.is_some() {

        }

        match found {
            Some(found_similarity_string) => return found_similarity_string,
            _ => {},
        }
    }

    "".to_owned()
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

    #[test]
    fn finds_common_sing_char_difference_box_id_part() {
        let example_input = vec!["abcde".to_owned(), "fghij".to_owned(), "klmno".to_owned(),
                             "pqrst".to_owned(), "fguij".to_owned(), "axcye".to_owned(),
                             "wvxyz".to_owned()];
        assert_eq!(find_common_correct_box_id_part(&example_input), "fgij".to_owned());
    }
}



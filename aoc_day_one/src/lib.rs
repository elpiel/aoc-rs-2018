use std::collections::HashSet;

pub fn calculate_frequency(frequency_changes: &Vec<i32>) -> i32 {
    frequency_changes.iter().sum()
}

pub fn calculate_first_frequency_twice(frequency_changes: &Vec<i32>) -> i32 {
    let mut starting_freq = 0;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(starting_freq);

    loop {
        for freq_change in frequency_changes {
            starting_freq += freq_change;
            if seen_frequencies.contains(&starting_freq) {
                return starting_freq;
            }
            seen_frequencies.insert(starting_freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_frequency() {
        assert_eq!(calculate_frequency(&vec![1, -2, 3, 1]), 3);
        assert_eq!(calculate_frequency(&vec![1, 1, 1]), 3);
        assert_eq!(calculate_frequency(&vec![1, 1, -2]), 0);
        assert_eq!(calculate_frequency(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn it_calculates_first_frequency_twice() {
        assert_eq!(calculate_first_frequency_twice(&vec![1, -2, 3, 1, 1, -2]), 2);
        assert_eq!(calculate_first_frequency_twice(&vec![1, -1]), 0);
        assert_eq!(calculate_first_frequency_twice(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(calculate_first_frequency_twice(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(calculate_first_frequency_twice(&vec![7, 7, -2, -7, -4]), 14);
    }
}



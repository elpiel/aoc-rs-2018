pub fn calculate_frequency(frequency_changes: &Vec<i32>) -> i32 {
    frequency_changes.iter().sum()
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
}



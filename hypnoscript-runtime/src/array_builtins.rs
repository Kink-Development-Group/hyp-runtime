/// Array/Vector builtin functions
pub struct ArrayBuiltins;

impl ArrayBuiltins {
    /// Get array length
    pub fn length<T>(arr: &[T]) -> usize {
        arr.len()
    }

    /// Check if array is empty
    pub fn is_empty<T>(arr: &[T]) -> bool {
        arr.is_empty()
    }

    /// Get element at index
    pub fn get<T: Clone>(arr: &[T], index: usize) -> Option<T> {
        arr.get(index).cloned()
    }

    /// Find index of element
    pub fn index_of<T: PartialEq>(arr: &[T], element: &T) -> i64 {
        arr.iter()
            .position(|x| x == element)
            .map(|i| i as i64)
            .unwrap_or(-1)
    }

    /// Check if array contains element
    pub fn contains<T: PartialEq>(arr: &[T], element: &T) -> bool {
        arr.contains(element)
    }

    /// Reverse array
    pub fn reverse<T: Clone>(arr: &[T]) -> Vec<T> {
        arr.iter().rev().cloned().collect()
    }

    /// Get sum of numeric array
    pub fn sum(arr: &[f64]) -> f64 {
        arr.iter().sum()
    }

    /// Get average of numeric array
    pub fn average(arr: &[f64]) -> f64 {
        if arr.is_empty() {
            0.0
        } else {
            Self::sum(arr) / arr.len() as f64
        }
    }

    /// Get minimum value
    pub fn min(arr: &[f64]) -> f64 {
        arr.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    /// Get maximum value
    pub fn max(arr: &[f64]) -> f64 {
        arr.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    /// Sort array (ascending)
    pub fn sort(arr: &[f64]) -> Vec<f64> {
        let mut sorted = arr.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
    }

    /// Get first element
    pub fn first<T: Clone>(arr: &[T]) -> Option<T> {
        arr.first().cloned()
    }

    /// Get last element
    pub fn last<T: Clone>(arr: &[T]) -> Option<T> {
        arr.last().cloned()
    }

    /// Take first n elements
    pub fn take<T: Clone>(arr: &[T], n: usize) -> Vec<T> {
        arr.iter().take(n).cloned().collect()
    }

    /// Skip first n elements
    pub fn skip<T: Clone>(arr: &[T], n: usize) -> Vec<T> {
        arr.iter().skip(n).cloned().collect()
    }

    /// Slice array
    pub fn slice<T: Clone>(arr: &[T], start: usize, end: usize) -> Vec<T> {
        let start = start.min(arr.len());
        let end = end.min(arr.len());
        if start >= end {
            Vec::new()
        } else {
            arr[start..end].to_vec()
        }
    }

    /// Join array elements into string
    pub fn join<T: std::fmt::Display>(arr: &[T], separator: &str) -> String {
        arr.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// Count occurrences of element
    pub fn count<T: PartialEq>(arr: &[T], element: &T) -> usize {
        arr.iter().filter(|&x| x == element).count()
    }

    /// Remove duplicates
    pub fn distinct<T: Clone + PartialEq>(arr: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        for item in arr {
            if !result.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(ArrayBuiltins::length(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(ArrayBuiltins::length(&[] as &[i32]), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(ArrayBuiltins::sum(&[1.0, 2.0, 3.0, 4.0, 5.0]), 15.0);
    }

    #[test]
    fn test_average() {
        assert_eq!(ArrayBuiltins::average(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(ArrayBuiltins::reverse(&[1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_distinct() {
        assert_eq!(ArrayBuiltins::distinct(&[1, 2, 2, 3, 3, 3]), vec![1, 2, 3]);
    }
}

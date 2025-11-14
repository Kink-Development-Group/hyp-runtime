use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;

/// Array/Vector builtin functions
///
/// Provides comprehensive array operations including functional programming
/// patterns (map, filter, reduce), aggregations, and transformations.
pub struct ArrayBuiltins;

impl BuiltinModule for ArrayBuiltins {
    fn module_name() -> &'static str {
        "Array"
    }

    fn description() -> &'static str {
        "Array manipulation and functional programming operations"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("Array manipulation and functional programming operations")
            .with_translation("de", "Array-Manipulation und funktionale Programmieroperationen")
            .with_translation("fr", "Manipulation de tableaux et opérations de programmation fonctionnelle")
            .with_translation("es", "Manipulación de arrays y operaciones de programación funcional");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "Length", "IsEmpty", "Get", "IndexOf", "Contains", "Reverse",
            "Sum", "Average", "Min", "Max", "Sort",
            "First", "Last", "Take", "Skip", "Slice",
            "Join", "Count", "Distinct",
            "Map", "Filter", "Reduce", "Find", "FindIndex",
            "Every", "Some", "Flatten", "Zip",
            "Partition", "GroupBy", "Chunk", "Windows",
        ]
    }
}

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

    // --- Functional Programming Operations ---

    /// Map: Apply a function to each element
    /// Note: Due to HypnoScript's current limitations, this is a placeholder.
    /// In practice, the interpreter would need to handle closures.
    pub fn map<T, U, F>(arr: &[T], f: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        arr.iter().map(f).collect()
    }

    /// Filter: Keep only elements that satisfy a predicate
    pub fn filter<T: Clone, F>(arr: &[T], predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        arr.iter().filter(|x| predicate(x)).cloned().collect()
    }

    /// Reduce: Reduce array to single value using accumulator function
    pub fn reduce<T, F>(arr: &[T], initial: T, f: F) -> T
    where
        T: Clone,
        F: Fn(T, &T) -> T,
    {
        arr.iter().fold(initial, f)
    }

    /// Find: Return first element matching predicate
    pub fn find<T: Clone, F>(arr: &[T], predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        arr.iter().find(|x| predicate(x)).cloned()
    }

    /// Find index: Return index of first element matching predicate
    pub fn find_index<T, F>(arr: &[T], predicate: F) -> i64
    where
        F: Fn(&T) -> bool,
    {
        arr.iter()
            .position(|x| predicate(x))
            .map(|i| i as i64)
            .unwrap_or(-1)
    }

    /// Every: Check if all elements satisfy predicate
    pub fn every<T, F>(arr: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        arr.iter().all(predicate)
    }

    /// Some: Check if any element satisfies predicate
    pub fn some<T, F>(arr: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        arr.iter().any(predicate)
    }

    /// Flatten: Flatten nested arrays one level
    pub fn flatten<T: Clone>(arr: &[Vec<T>]) -> Vec<T> {
        arr.iter().flat_map(|v| v.iter().cloned()).collect()
    }

    /// Zip: Combine two arrays into pairs
    pub fn zip<T: Clone, U: Clone>(arr1: &[T], arr2: &[U]) -> Vec<(T, U)> {
        arr1.iter()
            .zip(arr2.iter())
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }

    /// Chunk: Split array into chunks of given size
    pub fn chunk<T: Clone>(arr: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 {
            return Vec::new();
        }
        arr.chunks(size).map(|chunk| chunk.to_vec()).collect()
    }

    /// Shuffle: Randomly shuffle array elements
    /// Note: Uses a simple Fisher-Yates shuffle with a basic RNG
    pub fn shuffle<T: Clone>(arr: &[T], seed: u64) -> Vec<T> {
        let mut result = arr.to_vec();
        let mut rng_state = seed;

        for i in (1..result.len()).rev() {
            // Simple LCG for pseudo-random numbers
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let j = (rng_state as usize) % (i + 1);
            result.swap(i, j);
        }

        result
    }

    /// Partition: Split array into two based on predicate
    pub fn partition<T: Clone, F>(arr: &[T], predicate: F) -> (Vec<T>, Vec<T>)
    where
        F: Fn(&T) -> bool,
    {
        let mut true_vec = Vec::new();
        let mut false_vec = Vec::new();

        for item in arr {
            if predicate(item) {
                true_vec.push(item.clone());
            } else {
                false_vec.push(item.clone());
            }
        }

        (true_vec, false_vec)
    }

    /// Unique by key: Remove duplicates based on a key function
    pub fn unique_by<T: Clone, K: Eq + std::hash::Hash, F>(arr: &[T], key_fn: F) -> Vec<T>
    where
        F: Fn(&T) -> K,
    {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        let mut result = Vec::new();

        for item in arr {
            let key = key_fn(item);
            if seen.insert(key) {
                result.push(item.clone());
            }
        }

        result
    }

    /// Group by: Group elements by a key function
    pub fn group_by<T: Clone, K: Eq + std::hash::Hash, F>(
        arr: &[T],
        key_fn: F,
    ) -> std::collections::HashMap<K, Vec<T>>
    where
        F: Fn(&T) -> K,
    {
        use std::collections::HashMap;
        let mut groups: HashMap<K, Vec<T>> = HashMap::new();

        for item in arr {
            let key = key_fn(item);
            groups.entry(key).or_insert_with(Vec::new).push(item.clone());
        }

        groups
    }

    /// Rotate left: Move elements n positions to the left
    pub fn rotate_left<T: Clone>(arr: &[T], n: usize) -> Vec<T> {
        if arr.is_empty() {
            return Vec::new();
        }
        let n = n % arr.len();
        let mut result = arr.to_vec();
        result.rotate_left(n);
        result
    }

    /// Rotate right: Move elements n positions to the right
    pub fn rotate_right<T: Clone>(arr: &[T], n: usize) -> Vec<T> {
        if arr.is_empty() {
            return Vec::new();
        }
        let n = n % arr.len();
        let mut result = arr.to_vec();
        result.rotate_right(n);
        result
    }

    /// Interleave: Merge two arrays by alternating elements
    pub fn interleave<T: Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        let max_len = arr1.len().max(arr2.len());

        for i in 0..max_len {
            if i < arr1.len() {
                result.push(arr1[i].clone());
            }
            if i < arr2.len() {
                result.push(arr2[i].clone());
            }
        }

        result
    }

    /// Windows: Create sliding windows of size n
    ///
    /// # Arguments
    /// * `arr` - The source array
    /// * `size` - Window size
    ///
    /// # Returns
    /// Vector of windows (each window is a Vec)
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::ArrayBuiltins;
    /// let arr = [1, 2, 3, 4, 5];
    /// let windows = ArrayBuiltins::windows(&arr, 3);
    /// // Returns: [[1, 2, 3], [2, 3, 4], [3, 4, 5]]
    /// ```
    pub fn windows<T: Clone>(arr: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 || size > arr.len() {
            return Vec::new();
        }
        arr.windows(size).map(|w| w.to_vec()).collect()
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

    #[test]
    fn test_map() {
        let arr = [1, 2, 3, 4];
        let doubled = ArrayBuiltins::map(&arr, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter() {
        let arr = [1, 2, 3, 4, 5, 6];
        let evens = ArrayBuiltins::filter(&arr, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
    }

    #[test]
    fn test_reduce() {
        let arr = [1, 2, 3, 4];
        let sum = ArrayBuiltins::reduce(&arr, 0, |acc, x| acc + x);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_find() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(ArrayBuiltins::find(&arr, |x| *x > 3), Some(4));
        assert_eq!(ArrayBuiltins::find(&arr, |x| *x > 10), None);
    }

    #[test]
    fn test_every_some() {
        let arr = [2, 4, 6, 8];
        assert!(ArrayBuiltins::every(&arr, |x| x % 2 == 0));
        assert!(!ArrayBuiltins::every(&arr, |x| *x > 5));

        assert!(ArrayBuiltins::some(&arr, |x| *x > 5));
        assert!(!ArrayBuiltins::some(&arr, |x| *x > 10));
    }

    #[test]
    fn test_flatten() {
        let arr = vec![vec![1, 2], vec![3, 4], vec![5]];
        assert_eq!(ArrayBuiltins::flatten(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_zip() {
        let arr1 = [1, 2, 3];
        let arr2 = ['a', 'b', 'c'];
        let zipped = ArrayBuiltins::zip(&arr1, &arr2);
        assert_eq!(zipped, vec![(1, 'a'), (2, 'b'), (3, 'c')]);
    }

    #[test]
    fn test_chunk() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let chunks = ArrayBuiltins::chunk(&arr, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }

    #[test]
    fn test_shuffle() {
        let arr = [1, 2, 3, 4, 5];
        let shuffled = ArrayBuiltins::shuffle(&arr, 42);
        // Should have same elements, different order
        assert_eq!(shuffled.len(), arr.len());
        assert!(shuffled.iter().all(|x| arr.contains(x)));
    }

    #[test]
    fn test_partition() {
        let arr = [1, 2, 3, 4, 5, 6];
        let (evens, odds) = ArrayBuiltins::partition(&arr, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_rotate() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(ArrayBuiltins::rotate_left(&arr, 2), vec![3, 4, 5, 1, 2]);
        assert_eq!(ArrayBuiltins::rotate_right(&arr, 2), vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_interleave() {
        let arr1 = [1, 2, 3];
        let arr2 = [10, 20, 30];
        assert_eq!(
            ArrayBuiltins::interleave(&arr1, &arr2),
            vec![1, 10, 2, 20, 3, 30]
        );
    }

    #[test]
    fn test_windows() {
        let arr = [1, 2, 3, 4, 5];
        let windows = ArrayBuiltins::windows(&arr, 3);
        assert_eq!(
            windows,
            vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]
        );
    }

    #[test]
    fn test_module_metadata() {
        assert_eq!(ArrayBuiltins::module_name(), "Array");
        assert!(!ArrayBuiltins::function_names().is_empty());
        assert!(ArrayBuiltins::function_names().contains(&"Partition"));
        assert!(ArrayBuiltins::function_names().contains(&"GroupBy"));
    }
}

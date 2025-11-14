//! Collection builtin functions for HypnoScript.
//!
//! This module provides Set-like operations and advanced collection utilities
//! that complement the Array builtins. Includes set operations (union, intersection,
//! difference), frequency counting, and other collection-oriented functions.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;

/// Collection operations and Set-like functions.
///
/// Provides Set operations (union, intersection, difference), frequency analysis,
/// and other advanced collection utilities.
pub struct CollectionBuiltins;

impl BuiltinModule for CollectionBuiltins {
    fn module_name() -> &'static str {
        "Collection"
    }

    fn description() -> &'static str {
        "Set operations and advanced collection utilities"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("Set operations and advanced collection utilities")
            .with_translation("de", "Set-Operationen und erweiterte Collection-Utilities")
            .with_translation("fr", "Opérations d'ensemble et utilitaires de collection avancés")
            .with_translation("es", "Operaciones de conjunto y utilidades de colección avanzadas");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "Union", "Intersection", "Difference", "SymmetricDifference",
            "IsSubset", "IsSuperset", "IsDisjoint",
            "Frequency", "MostCommon", "LeastCommon",
            "ToSet", "SetSize", "CartesianProduct",
        ]
    }
}

impl CollectionBuiltins {
    /// Union: Combine two collections, removing duplicates
    ///
    /// # Arguments
    /// * `arr1` - First collection
    /// * `arr2` - Second collection
    ///
    /// # Returns
    /// Vector containing all unique elements from both collections
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::CollectionBuiltins;
    /// let a = vec![1, 2, 3];
    /// let b = vec![3, 4, 5];
    /// let union = CollectionBuiltins::union(&a, &b);
    /// // Returns: [1, 2, 3, 4, 5]
    /// ```
    pub fn union<T: Clone + Eq + Hash>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let mut set: HashSet<T> = arr1.iter().cloned().collect();
        set.extend(arr2.iter().cloned());
        set.into_iter().collect()
    }

    /// Intersection: Find common elements in two collections
    ///
    /// # Arguments
    /// * `arr1` - First collection
    /// * `arr2` - Second collection
    ///
    /// # Returns
    /// Vector containing elements present in both collections
    pub fn intersection<T: Clone + Eq + Hash>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let set1: HashSet<T> = arr1.iter().cloned().collect();
        let set2: HashSet<T> = arr2.iter().cloned().collect();
        set1.intersection(&set2).cloned().collect()
    }

    /// Difference: Find elements in first collection but not in second
    ///
    /// # Arguments
    /// * `arr1` - First collection
    /// * `arr2` - Second collection
    ///
    /// # Returns
    /// Vector containing elements in arr1 that are not in arr2
    pub fn difference<T: Clone + Eq + Hash>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let set1: HashSet<T> = arr1.iter().cloned().collect();
        let set2: HashSet<T> = arr2.iter().cloned().collect();
        set1.difference(&set2).cloned().collect()
    }

    /// Symmetric Difference: Elements in either collection but not both
    ///
    /// # Arguments
    /// * `arr1` - First collection
    /// * `arr2` - Second collection
    ///
    /// # Returns
    /// Vector containing elements that are in exactly one of the collections
    pub fn symmetric_difference<T: Clone + Eq + Hash>(arr1: &[T], arr2: &[T]) -> Vec<T> {
        let set1: HashSet<T> = arr1.iter().cloned().collect();
        let set2: HashSet<T> = arr2.iter().cloned().collect();
        set1.symmetric_difference(&set2).cloned().collect()
    }

    /// Check if first collection is a subset of second
    ///
    /// # Arguments
    /// * `arr1` - Potential subset
    /// * `arr2` - Potential superset
    ///
    /// # Returns
    /// True if all elements in arr1 are also in arr2
    pub fn is_subset<T: Eq + Hash>(arr1: &[T], arr2: &[T]) -> bool {
        let set1: HashSet<&T> = arr1.iter().collect();
        let set2: HashSet<&T> = arr2.iter().collect();
        set1.is_subset(&set2)
    }

    /// Check if first collection is a superset of second
    ///
    /// # Arguments
    /// * `arr1` - Potential superset
    /// * `arr2` - Potential subset
    ///
    /// # Returns
    /// True if arr1 contains all elements from arr2
    pub fn is_superset<T: Eq + Hash>(arr1: &[T], arr2: &[T]) -> bool {
        let set1: HashSet<&T> = arr1.iter().collect();
        let set2: HashSet<&T> = arr2.iter().collect();
        set1.is_superset(&set2)
    }

    /// Check if two collections have no common elements
    ///
    /// # Arguments
    /// * `arr1` - First collection
    /// * `arr2` - Second collection
    ///
    /// # Returns
    /// True if the collections have no elements in common
    pub fn is_disjoint<T: Eq + Hash>(arr1: &[T], arr2: &[T]) -> bool {
        let set1: HashSet<&T> = arr1.iter().collect();
        let set2: HashSet<&T> = arr2.iter().collect();
        set1.is_disjoint(&set2)
    }

    /// Count frequency of each element
    ///
    /// # Arguments
    /// * `arr` - Input collection
    ///
    /// # Returns
    /// HashMap mapping each unique element to its frequency count
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::CollectionBuiltins;
    /// let arr = vec![1, 2, 2, 3, 3, 3];
    /// let freq = CollectionBuiltins::frequency(&arr);
    /// // Returns: {1: 1, 2: 2, 3: 3}
    /// ```
    pub fn frequency<T: Clone + Eq + Hash>(arr: &[T]) -> HashMap<T, usize> {
        let mut freq_map = HashMap::new();
        for item in arr {
            *freq_map.entry(item.clone()).or_insert(0) += 1;
        }
        freq_map
    }

    /// Find the n most common elements
    ///
    /// # Arguments
    /// * `arr` - Input collection
    /// * `n` - Number of top elements to return
    ///
    /// # Returns
    /// Vector of (element, count) tuples, sorted by count descending
    pub fn most_common<T: Clone + Eq + Hash>(arr: &[T], n: usize) -> Vec<(T, usize)> {
        let freq = Self::frequency(arr);
        let mut freq_vec: Vec<_> = freq.into_iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        freq_vec.into_iter().take(n).collect()
    }

    /// Find the n least common elements
    ///
    /// # Arguments
    /// * `arr` - Input collection
    /// * `n` - Number of bottom elements to return
    ///
    /// # Returns
    /// Vector of (element, count) tuples, sorted by count ascending
    pub fn least_common<T: Clone + Eq + Hash>(arr: &[T], n: usize) -> Vec<(T, usize)> {
        let freq = Self::frequency(arr);
        let mut freq_vec: Vec<_> = freq.into_iter().collect();
        freq_vec.sort_by(|a, b| a.1.cmp(&b.1));
        freq_vec.into_iter().take(n).collect()
    }

    /// Convert collection to set (remove duplicates, no guaranteed order)
    ///
    /// # Arguments
    /// * `arr` - Input collection
    ///
    /// # Returns
    /// Vector with duplicates removed
    pub fn to_set<T: Clone + Eq + Hash>(arr: &[T]) -> Vec<T> {
        let set: HashSet<T> = arr.iter().cloned().collect();
        set.into_iter().collect()
    }

    /// Get the number of unique elements (cardinality)
    ///
    /// # Arguments
    /// * `arr` - Input collection
    ///
    /// # Returns
    /// Count of unique elements
    pub fn set_size<T: Eq + Hash>(arr: &[T]) -> usize {
        let set: HashSet<&T> = arr.iter().collect();
        set.len()
    }

    /// Cartesian Product: All possible pairs from two collections
    ///
    /// # Arguments
    /// * `arr1` - First collection
    /// * `arr2` - Second collection
    ///
    /// # Returns
    /// Vector of all possible (a, b) pairs where a ∈ arr1 and b ∈ arr2
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::CollectionBuiltins;
    /// let a = vec![1, 2];
    /// let b = vec!['x', 'y'];
    /// let product = CollectionBuiltins::cartesian_product(&a, &b);
    /// // Returns: [(1, 'x'), (1, 'y'), (2, 'x'), (2, 'y')]
    /// ```
    pub fn cartesian_product<T: Clone, U: Clone>(arr1: &[T], arr2: &[U]) -> Vec<(T, U)> {
        let mut result = Vec::new();
        for a in arr1 {
            for b in arr2 {
                result.push((a.clone(), b.clone()));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        let mut result = CollectionBuiltins::union(&a, &b);
        result.sort();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_intersection() {
        let a = vec![1, 2, 3, 4];
        let b = vec![3, 4, 5, 6];
        let mut result = CollectionBuiltins::intersection(&a, &b);
        result.sort();
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_difference() {
        let a = vec![1, 2, 3, 4];
        let b = vec![3, 4, 5];
        let mut result = CollectionBuiltins::difference(&a, &b);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_symmetric_difference() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 4];
        let mut result = CollectionBuiltins::symmetric_difference(&a, &b);
        result.sort();
        assert_eq!(result, vec![1, 4]);
    }

    #[test]
    fn test_is_subset() {
        let a = vec![1, 2];
        let b = vec![1, 2, 3, 4];
        assert!(CollectionBuiltins::is_subset(&a, &b));
        assert!(!CollectionBuiltins::is_subset(&b, &a));
    }

    #[test]
    fn test_is_superset() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2];
        assert!(CollectionBuiltins::is_superset(&a, &b));
        assert!(!CollectionBuiltins::is_superset(&b, &a));
    }

    #[test]
    fn test_is_disjoint() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![3, 4, 5];
        assert!(CollectionBuiltins::is_disjoint(&a, &b));
        assert!(!CollectionBuiltins::is_disjoint(&a, &c));
    }

    #[test]
    fn test_frequency() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        let freq = CollectionBuiltins::frequency(&arr);
        assert_eq!(freq.get(&1), Some(&1));
        assert_eq!(freq.get(&2), Some(&2));
        assert_eq!(freq.get(&3), Some(&3));
    }

    #[test]
    fn test_most_common() {
        let arr = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
        let common = CollectionBuiltins::most_common(&arr, 2);
        assert_eq!(common.len(), 2);
        assert_eq!(common[0], (4, 4));
        assert_eq!(common[1], (3, 3));
    }

    #[test]
    fn test_least_common() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        let common = CollectionBuiltins::least_common(&arr, 2);
        assert_eq!(common.len(), 2);
        assert_eq!(common[0], (1, 1));
        assert_eq!(common[1], (2, 2));
    }

    #[test]
    fn test_set_size() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(CollectionBuiltins::set_size(&arr), 3);
    }

    #[test]
    fn test_cartesian_product() {
        let a = vec![1, 2];
        let b = vec!['x', 'y'];
        let product = CollectionBuiltins::cartesian_product(&a, &b);
        assert_eq!(product.len(), 4);
        assert!(product.contains(&(1, 'x')));
        assert!(product.contains(&(1, 'y')));
        assert!(product.contains(&(2, 'x')));
        assert!(product.contains(&(2, 'y')));
    }

    #[test]
    fn test_module_metadata() {
        assert_eq!(CollectionBuiltins::module_name(), "Collection");
        assert!(!CollectionBuiltins::function_names().is_empty());
        assert!(CollectionBuiltins::function_names().contains(&"Union"));
        assert!(CollectionBuiltins::function_names().contains(&"Intersection"));
    }
}

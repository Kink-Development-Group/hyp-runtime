/// Statistics builtin functions
pub struct StatisticsBuiltins;

impl StatisticsBuiltins {
    /// Calculate mean (average) of numbers
    pub fn calculate_mean(numbers: &[f64]) -> f64 {
        if numbers.is_empty() {
            return 0.0;
        }
        numbers.iter().sum::<f64>() / numbers.len() as f64
    }

    /// Calculate median of numbers
    pub fn calculate_median(numbers: &[f64]) -> f64 {
        if numbers.is_empty() {
            return 0.0;
        }
        let mut sorted = numbers.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = sorted.len();
        if len % 2 == 0 {
            (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
        } else {
            sorted[len / 2]
        }
    }

    /// Calculate mode (most frequent value)
    pub fn calculate_mode(numbers: &[f64]) -> f64 {
        if numbers.is_empty() {
            return 0.0;
        }
        
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        for &n in numbers {
            *counts.entry(n.to_bits()).or_insert(0) += 1;
        }
        
        counts.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(bits, _)| f64::from_bits(*bits))
            .unwrap_or(0.0)
    }

    /// Calculate standard deviation
    pub fn calculate_standard_deviation(numbers: &[f64]) -> f64 {
        if numbers.len() < 2 {
            return 0.0;
        }
        let mean = Self::calculate_mean(numbers);
        let variance = numbers.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (numbers.len() - 1) as f64;
        variance.sqrt()
    }

    /// Calculate variance
    pub fn calculate_variance(numbers: &[f64]) -> f64 {
        if numbers.len() < 2 {
            return 0.0;
        }
        let mean = Self::calculate_mean(numbers);
        numbers.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (numbers.len() - 1) as f64
    }

    /// Calculate range (max - min)
    pub fn calculate_range(numbers: &[f64]) -> f64 {
        if numbers.is_empty() {
            return 0.0;
        }
        let min = numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        max - min
    }

    /// Calculate percentile
    pub fn calculate_percentile(numbers: &[f64], percentile: f64) -> f64 {
        if numbers.is_empty() || percentile < 0.0 || percentile > 100.0 {
            return 0.0;
        }
        let mut sorted = numbers.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let index = (percentile / 100.0 * (sorted.len() - 1) as f64).round() as usize;
        sorted[index]
    }

    /// Calculate correlation coefficient between two arrays
    pub fn calculate_correlation(x: &[f64], y: &[f64]) -> f64 {
        if x.len() != y.len() || x.is_empty() {
            return 0.0;
        }
        
        let mean_x = Self::calculate_mean(x);
        let mean_y = Self::calculate_mean(y);
        
        let numerator: f64 = x.iter().zip(y.iter())
            .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
            .sum();
        
        let denom_x: f64 = x.iter().map(|&xi| (xi - mean_x).powi(2)).sum();
        let denom_y: f64 = y.iter().map(|&yi| (yi - mean_y).powi(2)).sum();
        
        if denom_x == 0.0 || denom_y == 0.0 {
            return 0.0;
        }
        
        numerator / (denom_x * denom_y).sqrt()
    }

    /// Simple linear regression (returns slope and intercept)
    pub fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
        if x.len() != y.len() || x.is_empty() {
            return (0.0, 0.0);
        }
        
        let mean_x = Self::calculate_mean(x);
        let mean_y = Self::calculate_mean(y);
        
        let numerator: f64 = x.iter().zip(y.iter())
            .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
            .sum();
        
        let denominator: f64 = x.iter()
            .map(|&xi| (xi - mean_x).powi(2))
            .sum();
        
        if denominator == 0.0 {
            return (0.0, mean_y);
        }
        
        let slope = numerator / denominator;
        let intercept = mean_y - slope * mean_x;
        
        (slope, intercept)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mean() {
        assert_eq!(StatisticsBuiltins::calculate_mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(StatisticsBuiltins::calculate_mean(&[10.0, 20.0, 30.0]), 20.0);
    }

    #[test]
    fn test_calculate_median() {
        assert_eq!(StatisticsBuiltins::calculate_median(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(StatisticsBuiltins::calculate_median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }

    #[test]
    fn test_calculate_standard_deviation() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let sd = StatisticsBuiltins::calculate_standard_deviation(&data);
        assert!((sd - 2.138).abs() < 0.01); // Approximately 2.138
    }

    #[test]
    fn test_calculate_range() {
        assert_eq!(StatisticsBuiltins::calculate_range(&[1.0, 2.0, 3.0, 4.0, 5.0]), 4.0);
        assert_eq!(StatisticsBuiltins::calculate_range(&[10.0, 100.0]), 90.0);
    }
}

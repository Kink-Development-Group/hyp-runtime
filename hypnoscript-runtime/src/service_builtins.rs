use serde::{Deserialize, Serialize};

/// Suggested retry delays for a given configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrySchedule {
    pub attempts: u32,
    pub delays_ms: Vec<u64>,
}

impl RetrySchedule {
    pub fn as_slice(&self) -> &[u64] {
        &self.delays_ms
    }
}

/// Basic service health metrics useful for API/service environments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceHealthReport {
    pub uptime_percentage: f64,
    pub average_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub slo_breached: bool,
}

impl ServiceHealthReport {
    fn new(
        uptime_percentage: f64,
        average_latency_ms: f64,
        p95_latency_ms: f64,
        slo_breached: bool,
    ) -> Self {
        Self {
            uptime_percentage,
            average_latency_ms,
            p95_latency_ms,
            slo_breached,
        }
    }
}

/// Builtins focused on long-running service workloads.
pub struct ServiceBuiltins;

impl ServiceBuiltins {
    /// Generates an exponential backoff schedule with optional jitter.
    pub fn retry_schedule(
        attempts: u32,
        base_delay_ms: u64,
        multiplier: f64,
        jitter_ms: u64,
        max_delay_ms: Option<u64>,
    ) -> RetrySchedule {
        let mut delays = Vec::new();
        let mut current = base_delay_ms as f64;
        for _ in 0..attempts {
            let mut delay = current as u64;
            if let Some(max_delay) = max_delay_ms {
                delay = delay.min(max_delay);
            }
            if jitter_ms > 0 {
                let jitter = rand_jitter(jitter_ms);
                delay += jitter;
            }
            delays.push(delay);
            current *= multiplier.max(1.0);
        }
        RetrySchedule {
            attempts,
            delays_ms: delays,
        }
    }

    /// Computes a health report combining latency samples + uptime data.
    pub fn health_report(
        latencies_ms: &[u64],
        successful_requests: u64,
        total_requests: u64,
        slo_latency_ms: u64,
    ) -> ServiceHealthReport {
        let uptime = if total_requests == 0 {
            100.0
        } else {
            (successful_requests as f64 / total_requests as f64) * 100.0
        };
        let avg_latency = if latencies_ms.is_empty() {
            0.0
        } else {
            let sum: u128 = latencies_ms.iter().map(|&v| v as u128).sum();
            (sum as f64) / (latencies_ms.len() as f64)
        };
        let p95 = percentile(latencies_ms, 0.95);
        let slo_breached = p95 > slo_latency_ms;

        ServiceHealthReport::new(uptime, avg_latency, p95 as f64, slo_breached)
    }

    /// Applies a rolling error window to decide whether to open a circuit.
    pub fn should_open_circuit(failures: &[bool], threshold: f64) -> bool {
        if failures.is_empty() {
            return false;
        }
        let failure_ratio = failures.iter().filter(|&&f| f).count() as f64 / failures.len() as f64;
        failure_ratio >= threshold
    }
}

fn rand_jitter(max_jitter_ms: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    if max_jitter_ms == 0 {
        0
    } else {
        nanos % (max_jitter_ms + 1)
    }
}

fn percentile(samples: &[u64], quantile: f64) -> u64 {
    if samples.is_empty() {
        return 0;
    }
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let position = ((sorted.len() as f64 - 1.0) * quantile).round() as usize;
    sorted[position]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_schedule_grows_exponentially() {
        let schedule = ServiceBuiltins::retry_schedule(3, 100, 2.0, 0, None);
        assert_eq!(schedule.as_slice(), &[100, 200, 400]);
    }

    #[test]
    fn health_report_detects_slo_violation() {
        let report = ServiceBuiltins::health_report(&[10, 20, 120], 95, 100, 100);
        assert!(report.slo_breached);
        assert_eq!(report.uptime_percentage, 95.0);
    }

    #[test]
    fn circuit_breaker_threshold() {
        let should_open = ServiceBuiltins::should_open_circuit(&[true, true, false, true], 0.6);
        assert!(should_open);
    }
}

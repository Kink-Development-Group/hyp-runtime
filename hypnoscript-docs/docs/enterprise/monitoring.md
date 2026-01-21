# Runtime Monitoring & Observability

HypnoScript provides comprehensive monitoring and observability features for runtime environments, including metrics, logging, distributed tracing, and proactive alerting systems.

## Monitoring Architecture

### Overview

```hyp
// Monitoring stack configuration
monitoring {
    // Data collection
    collection: {
        metrics: "prometheus"
        logs: "fluentd"
        traces: "jaeger"
        events: "kafka"
    }

    // Storage
    storage: {
        metrics: "influxdb"
        logs: "elasticsearch"
        traces: "jaeger"
        events: "kafka"
    }

    // Visualization
    visualization: {
        dashboards: "grafana"
        alerting: "alertmanager"
        reporting: "kibana"
    }
}
```

## Metrics

### System Metrics

```hyp
// System monitoring
system_metrics {
    // CPU metrics
    cpu: {
        usage_percent: true
        load_average: true
        context_switches: true
        interrupts: true
    }

    // Memory metrics
    memory: {
        usage_bytes: true
        available_bytes: true
        swap_usage: true
        page_faults: true
    }

    // Disk metrics
    disk: {
        usage_percent: true
        io_operations: true
        io_bytes: true
        latency: true
    }

    // Network metrics
    network: {
        bytes_sent: true
        bytes_received: true
        packets_sent: true
        packets_received: true
        errors: true
        drops: true
    }
}
```

### Application Metrics

```hyp
// Application monitoring
application_metrics {
    // Performance metrics
    performance: {
        response_time: {
            p50: true
            p95: true
            p99: true
            p999: true
        }
        throughput: {
            requests_per_second: true
            transactions_per_second: true
        }
        error_rate: true
        availability: true
    }

    // Business metrics
    business: {
        active_users: true
        script_executions: true
        data_processed: true
        revenue_impact: true
    }

    // Custom metrics
    custom: {
        script_complexity: true
        execution_duration: true
        memory_usage: true
        cache_hit_rate: true
    }
}
```

### Metrics Configuration

```hyp
// Metrics collection
metrics_collection {
    // Prometheus configuration
    prometheus: {
        scrape_interval: "15s"
        evaluation_interval: "15s"
        retention_days: 30

        // Service discovery
        service_discovery: {
            kubernetes: true
            consul: true
            static_configs: true
        }

        // Relabeling
        relabel_configs: [
            {
                source_labels: ["__meta_kubernetes_pod_label_app"]
                target_label: "app"
            },
            {
                source_labels: ["__meta_kubernetes_namespace"]
                target_label: "namespace"
            }
        ]
    }

    // Custom metrics
    custom_metrics: {
        script_execution_time: {
            type: "histogram"
            buckets: [0.1, 0.5, 1, 2, 5, 10, 30, 60]
            labels: ["script_name", "environment", "user"]
        }

        script_memory_usage: {
            type: "gauge"
            labels: ["script_name", "environment"]
        }

        script_error_count: {
            type: "counter"
            labels: ["script_name", "error_type", "environment"]
        }
    }
}
```

## Logging

### Structured Logging

```hyp
// Logging configuration
logging {
    // Log levels
    levels: {
        development: "debug"
        staging: "info"
        production: "warn"
    }

    // Log format
    format: {
        type: "json"
        timestamp: "iso8601"
        include_metadata: true

        // Standard fields
        standard_fields: [
            "timestamp",
            "level",
            "message",
            "service",
            "version",
            "environment",
            "trace_id",
            "span_id"
        ]
    }

    // Log rotation
    rotation: {
        max_size: "100MB"
        max_files: 10
        max_age: "30d"
        compress: true
    }
}
```

### Log Aggregation

```hyp
// Log aggregation
log_aggregation {
    // Fluentd configuration
    fluentd: {
        input: {
            type: "tail"
            path: "/var/log/hypnoscript/*.log"
            pos_file: "/var/log/fluentd/hypnoscript.pos"
            tag: "hypnoscript.*"
            format: "json"
        }

        filter: [
            {
                type: "record_transformer"
                enable_ruby: true
                record: {
                    service: "hypnoscript"
                    environment: env.ENVIRONMENT
                    version: env.VERSION
                }
            },
            {
                type: "grep"
                regexp1: "level error"
                tag: "hypnoscript.error"
            }
        ]

        output: [
            {
                type: "elasticsearch"
                host: "elasticsearch.example.com"
                port: 9200
                logstash_format: true
                logstash_prefix: "hypnoscript"
            },
            {
                type: "s3"
                aws_key_id: env.AWS_ACCESS_KEY_ID
                aws_sec_key: env.AWS_SECRET_ACCESS_KEY
                s3_bucket: "hypnoscript-logs"
                s3_region: "eu-west-1"
                path: "logs/%Y/%m/%d/"
            }
        ]
    }
}
```

## Distributed Tracing

### Tracing Configuration

```hyp
// Distributed Tracing
tracing {
    // Jaeger configuration
    jaeger: {
        endpoint: "http://jaeger.example.com:14268/api/traces"
        service_name: "hypnoscript"
        environment: env.ENVIRONMENT

        // Sampling
        sampling: {
            type: "probabilistic"
            param: 0.1  // 10% of traces
        }

        // Tags
        tags: {
            version: env.VERSION
            environment: env.ENVIRONMENT
            region: env.AWS_REGION
        }
    }

    // Trace configuration
    trace_config: {
        // Automatic instrumentation
        auto_instrumentation: {
            http: true
            database: true
            cache: true
            messaging: true
        }

        // Custom spans
        custom_spans: {
            script_execution: true
            data_processing: true
            external_api_call: true
        }

        // Trace propagation
        propagation: {
            headers: ["x-trace-id", "x-span-id"]
            baggage: true
        }
    }
}
```

### Trace Analysis

```hyp
// Trace analysis
trace_analysis {
    // Performance analysis
    performance: {
        slow_query_detection: {
            threshold: "1s"
            alert: true
        }

        bottleneck_identification: true
        dependency_mapping: true
    }

    // Error analysis
    error_analysis: {
        error_tracking: true
        error_grouping: true
        error_trends: true
    }

    // Business traces
    business_traces: {
        user_journey_tracking: true
        conversion_funnel: true
        feature_usage: true
    }
}
```

## Alerting

### Alert Configuration

```hyp
// Alerting system
alerting {
    // Alertmanager configuration
    alertmanager: {
        global: {
            smtp_smarthost: "smtp.example.com:587"
            smtp_from: "alerts@example.com"
            smtp_auth_username: env.SMTP_USERNAME
            smtp_auth_password: env.SMTP_PASSWORD
        }

        route: {
            group_by: ["alertname", "service", "environment"]
            group_wait: "30s"
            group_interval: "5m"
            repeat_interval: "4h"

            receiver: "team-hypnoscript"

            routes: [
                {
                    match: {
                        severity: "critical"
                    }
                    receiver: "team-hypnoscript-critical"
                    repeat_interval: "1h"
                },
                {
                    match: {
                        service: "hypnoscript-api"
                    }
                    receiver: "team-api"
                }
            ]
        }

        receivers: [
            {
                name: "team-hypnoscript"
                email_configs: [
                    {
                        to: "hypnoscript-team@example.com"
                    }
                ]
                slack_configs: [
                    {
                        api_url: env.SLACK_WEBHOOK_URL
                        channel: "#hypnoscript-alerts"
                    }
                ]
            },
            {
                name: "team-hypnoscript-critical"
                email_configs: [
                    {
                        to: "hypnoscript-critical@example.com"
                    }
                ]
                pagerduty_configs: [
                    {
                        service_key: env.PAGERDUTY_SERVICE_KEY
                    }
                ]
            }
        ]
    }
}
```

### Alert Rules

```hyp
// Prometheus Alert Rules
alert_rules {
    // System alerts
    system_alerts: {
        high_cpu_usage: {
            expr: '100 - (avg by(instance) (irate(node_cpu_seconds_total{mode="idle"}[5m])) * 100) > 80'
            for: "5m"
            labels: {
                severity: "warning"
                service: "system"
            }
            annotations: {
                summary: "High CPU usage on {{ $labels.instance }}"
                description: "CPU usage is above 80% for 5 minutes"
            }
        }

        high_memory_usage: {
            expr: '(node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100 > 85'
            for: "5m"
            labels: {
                severity: "warning"
                service: "system"
            }
            annotations: {
                summary: "High memory usage on {{ $labels.instance }}"
                description: "Memory usage is above 85% for 5 minutes"
            }
        }

        disk_space_low: {
            expr: '(node_filesystem_avail_bytes{mountpoint="/"} / node_filesystem_size_bytes{mountpoint="/"}) * 100 < 10'
            for: "5m"
            labels: {
                severity: "critical"
                service: "system"
            }
            annotations: {
                summary: "Low disk space on {{ $labels.instance }}"
                description: "Disk space is below 10%"
            }
        }
    }

    // Application alerts
    application_alerts: {
        high_error_rate: {
            expr: 'rate(hypnoscript_errors_total[5m]) / rate(hypnoscript_requests_total[5m]) * 100 > 5'
            for: "2m"
            labels: {
                severity: "critical"
                service: "hypnoscript"
            }
            annotations: {
                summary: "High error rate in HypnoScript"
                description: "Error rate is above 5% for 2 minutes"
            }
        }

        high_response_time: {
            expr: 'histogram_quantile(0.95, rate(hypnoscript_request_duration_seconds_bucket[5m])) > 2'
            for: "5m"
            labels: {
                severity: "warning"
                service: "hypnoscript"
            }
            annotations: {
                summary: "High response time in HypnoScript"
                description: "95th percentile response time is above 2 seconds"
            }
        }

        service_down: {
            expr: 'up{service="hypnoscript"} == 0'
            for: "1m"
            labels: {
                severity: "critical"
                service: "hypnoscript"
            }
            annotations: {
                summary: "HypnoScript service is down"
                description: "Service has been down for more than 1 minute"
            }
        }
    }
}
```

## Dashboards

### Grafana Dashboards

```hyp
// Dashboard configuration
dashboards {
    // System dashboard
    system_dashboard: {
        title: "HypnoScript System Overview"
        refresh: "30s"

        panels: [
            {
                title: "CPU Usage"
                type: "graph"
                query: '100 - (avg by(instance) (irate(node_cpu_seconds_total{mode="idle"}[5m])) * 100)'
                y_axis: {
                    min: 0
                    max: 100
                    unit: "percent"
                }
            },
            {
                title: "Memory Usage"
                type: "graph"
                query: '(node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100'
                y_axis: {
                    min: 0
                    max: 100
                    unit: "percent"
                }
            },
            {
                title: "Disk Usage"
                type: "graph"
                query: '(node_filesystem_size_bytes{mountpoint="/"} - node_filesystem_avail_bytes{mountpoint="/"}) / node_filesystem_size_bytes{mountpoint="/"} * 100'
                y_axis: {
                    min: 0
                    max: 100
                    unit: "percent"
                }
            },
            {
                title: "Network Traffic"
                type: "graph"
                query: 'rate(node_network_receive_bytes_total[5m])'
                y_axis: {
                    unit: "bytes"
                }
            }
        ]
    }

    // Application dashboard
    application_dashboard: {
        title: "HypnoScript Application Metrics"
        refresh: "15s"

        panels: [
            {
                title: "Request Rate"
                type: "graph"
                query: 'rate(hypnoscript_requests_total[5m])'
                y_axis: {
                    unit: "reqps"
                }
            },
            {
                title: "Response Time (95th percentile)"
                type: "graph"
                query: 'histogram_quantile(0.95, rate(hypnoscript_request_duration_seconds_bucket[5m]))'
                y_axis: {
                    unit: "s"
                }
            },
            {
                title: "Error Rate"
                type: "graph"
                query: 'rate(hypnoscript_errors_total[5m]) / rate(hypnoscript_requests_total[5m]) * 100'
                y_axis: {
                    min: 0
                    max: 100
                    unit: "percent"
                }
            },
            {
                title: "Active Scripts"
                type: "stat"
                query: 'hypnoscript_active_scripts'
            },
            {
                title: "Script Execution Time"
                type: "heatmap"
                query: 'rate(hypnoscript_execution_duration_seconds_bucket[5m])'
            }
        ]
    }

    // Business dashboard
    business_dashboard: {
        title: "HypnoScript Business Metrics"
        refresh: "1m"

        panels: [
            {
                title: "Active Users"
                type: "stat"
                query: 'hypnoscript_active_users'
            },
            {
                title: "Script Executions"
                type: "graph"
                query: 'rate(hypnoscript_executions_total[5m])'
                y_axis: {
                    unit: "executions/s"
                }
            },
            {
                title: "Data Processed"
                type: "graph"
                query: 'rate(hypnoscript_data_processed_bytes[5m])'
                y_axis: {
                    unit: "bytes"
                }
            },
            {
                title: "Revenue Impact"
                type: "stat"
                query: 'hypnoscript_revenue_impact'
                y_axis: {
                    unit: "currency"
                }
            }
        ]
    }
}
```

## Performance Monitoring

### APM (Application Performance Monitoring)

```hyp
// APM configuration
apm {
    // Performance tracking
    performance_tracking: {
        // Method-level tracking
        method_tracking: {
            enabled: true
            threshold: "100ms"
            include_arguments: false
        }

        // Database tracking
        database_tracking: {
            enabled: true
            slow_query_threshold: "1s"
            include_sql: false
        }

        // External call tracking
        external_call_tracking: {
            enabled: true
            timeout_threshold: "5s"
            include_headers: false
        }
    }

    // Resource monitoring
    resource_monitoring: {
        memory_leak_detection: true
        gc_monitoring: true
        thread_monitoring: true
        connection_pool_monitoring: true
    }

    // Business transaction monitoring
    business_transaction_monitoring: {
        user_journey_tracking: true
        conversion_funnel_monitoring: true
        feature_usage_tracking: true
    }
}
```

## Best Practices

### Monitoring Best Practices

1. **Golden Signals**
   - Latency (Response Time)
   - Traffic (Request Rate)
   - Errors (Error Rate)
   - Saturation (Resource Usage)

2. **Alerting Strategies**
   - Few but meaningful alerts
   - Define different severity levels
   - Set up automatic escalation

3. **Dashboard Design**
   - Place key metrics prominently
   - Use consistent color schemes
   - Add contextual information

4. **Logging Strategies**
   - Use structured logging
   - Mask sensitive data
   - Configure log rotation

5. **Tracing Strategies**
   - Implement distributed tracing
   - Use sampling for performance
   - Add business context

### Monitoring Checklist

- [ ] System metrics configured
- [ ] Application metrics implemented
- [ ] Logging system set up
- [ ] Distributed tracing enabled
- [ ] Alerting rules defined
- [ ] Dashboards created
- [ ] Performance-Monitoring configured
- [ ] Business metrics defined
- [ ] Monitoring documentation created
- [ ] Team training completed

These monitoring and observability features ensure that HypnoScript in runtime environments is fully monitored and can respond proactively to issues.

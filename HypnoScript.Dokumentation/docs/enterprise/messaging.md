# Enterprise Messaging & Queuing

HypnoScript bietet umfassende Messaging- und Queuing-Funktionen für Enterprise-Umgebungen, einschließlich Message Brokers, Event-Driven Architecture, Message Patterns und zuverlässige Nachrichtenverarbeitung.

## Message Broker Integration

### Broker-Konfiguration

```hyp
// Message Broker-Konfiguration
messaging {
    // Apache Kafka
    kafka: {
        bootstrap_servers: [
            "kafka-1.example.com:9092",
            "kafka-2.example.com:9092",
            "kafka-3.example.com:9092"
        ]

        // Producer-Konfiguration
        producer: {
            acks: "all"
            retries: 3
            batch_size: 16384
            linger_ms: 5
            buffer_memory: 33554432
            compression_type: "snappy"

            // Sicherheit
            security: {
                sasl_mechanism: "PLAIN"
                sasl_username: env.KAFKA_USERNAME
                sasl_password: env.KAFKA_PASSWORD
                ssl_enabled: true
            }
        }

        // Consumer-Konfiguration
        consumer: {
            group_id: "hypnoscript-consumer-group"
            auto_offset_reset: "earliest"
            enable_auto_commit: false
            session_timeout_ms: 30000
            heartbeat_interval_ms: 3000
            max_poll_records: 500
            max_poll_interval_ms: 300000

            // Sicherheit
            security: {
                sasl_mechanism: "PLAIN"
                sasl_username: env.KAFKA_USERNAME
                sasl_password: env.KAFKA_PASSWORD
                ssl_enabled: true
            }
        }
    }

    // RabbitMQ
    rabbitmq: {
        host: "rabbitmq.example.com"
        port: 5672
        virtual_host: "/hypnoscript"
        username: env.RABBITMQ_USERNAME
        password: env.RABBITMQ_PASSWORD

        // Verbindungseinstellungen
        connection: {
            heartbeat: 60
            connection_timeout: 60000
            channel_rpc_timeout: 10000
            automatic_recovery: true
            network_recovery_interval: 5000
        }

        // Channel-Pooling
        channel_pool: {
            max_channels: 100
            channel_timeout: 30000
        }

        // SSL/TLS
        ssl: {
            enabled: true
            verify_peer: true
            fail_if_no_peer_cert: false
        }
    }

    // Apache ActiveMQ
    activemq: {
        broker_url: "tcp://activemq.example.com:61616"
        username: env.ACTIVEMQ_USERNAME
        password: env.ACTIVEMQ_PASSWORD

        // Verbindungseinstellungen
        connection: {
            max_connections: 50
            connection_timeout: 30000
            idle_timeout: 300000
            keep_alive: true
        }

        // Session-Pooling
        session_pool: {
            max_sessions: 200
            session_timeout: 60000
        }
    }

    // AWS SQS/SNS
    aws_messaging: {
        region: "eu-west-1"
        access_key_id: env.AWS_ACCESS_KEY_ID
        secret_access_key: env.AWS_SECRET_ACCESS_KEY

        // SQS-Konfiguration
        sqs: {
            max_messages: 10
            visibility_timeout: 30
            wait_time_seconds: 20
            message_retention_period: 1209600  // 14 Tage
            receive_message_wait_time_seconds: 20
        }

        // SNS-Konfiguration
        sns: {
            message_structure: "json"
            message_attributes: true
        }
    }
}
```

## Event-Driven Architecture

### Event-Definitionen

```hyp
// Event-Schema-Definitionen
events {
    // Script-Events
    ScriptEvents: {
        // Script erstellt
        ScriptCreated: {
            event_type: "script.created"
            version: "1.0"

            payload: {
                script_id: "uuid"
                name: "string"
                created_by: "uuid"
                created_at: "timestamp"
                metadata: "object"
            }

            metadata: {
                source: "hypnoscript-api"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }

        // Script aktualisiert
        ScriptUpdated: {
            event_type: "script.updated"
            version: "1.0"

            payload: {
                script_id: "uuid"
                name: "string"
                version: "integer"
                updated_by: "uuid"
                updated_at: "timestamp"
                changes: "object"
            }

            metadata: {
                source: "hypnoscript-api"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }

        // Script gelöscht
        ScriptDeleted: {
            event_type: "script.deleted"
            version: "1.0"

            payload: {
                script_id: "uuid"
                deleted_by: "uuid"
                deleted_at: "timestamp"
                reason: "string"
            }

            metadata: {
                source: "hypnoscript-api"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }

        // Script ausgeführt
        ScriptExecuted: {
            event_type: "script.executed"
            version: "1.0"

            payload: {
                execution_id: "uuid"
                script_id: "uuid"
                user_id: "uuid"
                started_at: "timestamp"
                completed_at: "timestamp"
                duration_ms: "integer"
                status: "string"
                result: "object"
                error_message: "string"
                environment: "string"
            }

            metadata: {
                source: "hypnoscript-executor"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }
    }

    // User-Events
    UserEvents: {
        // Benutzer registriert
        UserRegistered: {
            event_type: "user.registered"
            version: "1.0"

            payload: {
                user_id: "uuid"
                email: "string"
                username: "string"
                registered_at: "timestamp"
            }

            metadata: {
                source: "hypnoscript-auth"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }

        // Benutzer angemeldet
        UserLoggedIn: {
            event_type: "user.logged_in"
            version: "1.0"

            payload: {
                user_id: "uuid"
                login_at: "timestamp"
                ip_address: "string"
                user_agent: "string"
            }

            metadata: {
                source: "hypnoscript-auth"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }
    }

    // System-Events
    SystemEvents: {
        // System-Start
        SystemStarted: {
            event_type: "system.started"
            version: "1.0"

            payload: {
                service_name: "string"
                version: "string"
                started_at: "timestamp"
                environment: "string"
                instance_id: "string"
            }

            metadata: {
                source: "hypnoscript-system"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }

        // System-Fehler
        SystemError: {
            event_type: "system.error"
            version: "1.0"

            payload: {
                error_code: "string"
                error_message: "string"
                stack_trace: "string"
                occurred_at: "timestamp"
                service_name: "string"
                severity: "string"
            }

            metadata: {
                source: "hypnoscript-system"
                correlation_id: "uuid"
                causation_id: "uuid"
                timestamp: "timestamp"
            }
        }
    }
}
```

### Event-Producer

```hyp
// Event-Producer-Konfiguration
event_producers {
    // Script-Event-Producer
    ScriptEventProducer: {
        broker: "kafka"
        topic_prefix: "hypnoscript.events"

        // Event-Mapping
        events: {
            "script.created": {
                topic: "script-events"
                partition_key: "script_id"
                retry_policy: {
                    max_retries: 3
                    backoff_strategy: "exponential"
                    initial_delay: 1000
                }
            }

            "script.updated": {
                topic: "script-events"
                partition_key: "script_id"
                retry_policy: {
                    max_retries: 3
                    backoff_strategy: "exponential"
                    initial_delay: 1000
                }
            }

            "script.deleted": {
                topic: "script-events"
                partition_key: "script_id"
                retry_policy: {
                    max_retries: 3
                    backoff_strategy: "exponential"
                    initial_delay: 1000
                }
            }

            "script.executed": {
                topic: "execution-events"
                partition_key: "script_id"
                retry_policy: {
                    max_retries: 5
                    backoff_strategy: "exponential"
                    initial_delay: 2000
                }
            }
        }

        // Event-Serialisierung
        serialization: {
            format: "json"
            compression: "snappy"
            schema_registry: {
                url: "http://schema-registry.example.com"
                auto_register: true
            }
        }

        // Event-Validierung
        validation: {
            schema_validation: true
            required_fields: ["event_type", "payload", "metadata"]
            payload_size_limit: 1048576  // 1MB
        }
    }

    // User-Event-Producer
    UserEventProducer: {
        broker: "kafka"
        topic_prefix: "hypnoscript.user"

        events: {
            "user.registered": {
                topic: "user-events"
                partition_key: "user_id"
            }

            "user.logged_in": {
                topic: "user-events"
                partition_key: "user_id"
            }
        }

        serialization: {
            format: "json"
            compression: "snappy"
        }
    }
}
```

### Event-Consumer

```hyp
// Event-Consumer-Konfiguration
event_consumers {
    // Script-Event-Consumer
    ScriptEventConsumer: {
        broker: "kafka"
        group_id: "script-event-processor"

        // Topic-Subscription
        topics: [
            {
                name: "script-events"
                partitions: [0, 1, 2, 3]
                auto_offset_reset: "earliest"
            },
            {
                name: "execution-events"
                partitions: [0, 1, 2, 3]
                auto_offset_reset: "earliest"
            }
        ]

        // Event-Handler
        handlers: {
            "script.created": {
                handler: "ScriptCreatedHandler"
                concurrency: 5
                timeout: 30000
                retry_policy: {
                    max_retries: 3
                    backoff_strategy: "exponential"
                }
            }

            "script.updated": {
                handler: "ScriptUpdatedHandler"
                concurrency: 5
                timeout: 30000
                retry_policy: {
                    max_retries: 3
                    backoff_strategy: "exponential"
                }
            }

            "script.executed": {
                handler: "ScriptExecutedHandler"
                concurrency: 10
                timeout: 60000
                retry_policy: {
                    max_retries: 5
                    backoff_strategy: "exponential"
                }
            }
        }

        // Consumer-Einstellungen
        settings: {
            max_poll_records: 100
            max_poll_interval_ms: 300000
            session_timeout_ms: 30000
            heartbeat_interval_ms: 3000
            enable_auto_commit: false
        }
    }

    // Analytics-Event-Consumer
    AnalyticsEventConsumer: {
        broker: "kafka"
        group_id: "analytics-processor"

        topics: [
            {
                name: "script-events"
                partitions: [0, 1, 2, 3]
            },
            {
                name: "execution-events"
                partitions: [0, 1, 2, 3]
            },
            {
                name: "user-events"
                partitions: [0, 1, 2, 3]
            }
        ]

        handlers: {
            "*": {
                handler: "AnalyticsEventHandler"
                concurrency: 20
                timeout: 60000
                batch_size: 100
                batch_timeout: 5000
            }
        }

        settings: {
            max_poll_records: 500
            enable_auto_commit: true
            auto_commit_interval_ms: 5000
        }
    }
}
```

## Message Patterns

### Request-Reply Pattern

```hyp
// Request-Reply Pattern
request_reply {
    // Script-Validierung
    script_validation: {
        request_topic: "script.validation.request"
        reply_topic: "script.validation.reply"
        correlation_id_header: "correlation_id"

        // Request-Schema
        request_schema: {
            script_id: "uuid"
            content: "string"
            validation_rules: "array"
            timeout: "integer"
        }

        // Reply-Schema
        reply_schema: {
            script_id: "uuid"
            valid: "boolean"
            errors: "array"
            warnings: "array"
            validation_time_ms: "integer"
        }

        // Timeout-Konfiguration
        timeout: 30000  // 30 Sekunden
        retry_policy: {
            max_retries: 3
            backoff_strategy: "exponential"
            initial_delay: 1000
        }
    }

    // Script-Ausführung
    script_execution: {
        request_topic: "script.execution.request"
        reply_topic: "script.execution.reply"
        correlation_id_header: "correlation_id"

        request_schema: {
            script_id: "uuid"
            parameters: "object"
            timeout: "integer"
            environment: "string"
        }

        reply_schema: {
            execution_id: "uuid"
            script_id: "uuid"
            status: "string"
            result: "object"
            error_message: "string"
            execution_time_ms: "integer"
        }

        timeout: 300000  // 5 Minuten
        retry_policy: {
            max_retries: 2
            backoff_strategy: "exponential"
            initial_delay: 5000
        }
    }
}
```

### Publish-Subscribe Pattern

```hyp
// Publish-Subscribe Pattern
pub_sub {
    // Script-Änderungen
    script_changes: {
        topic: "script.changes"

        // Publisher
        publisher: {
            name: "ScriptChangePublisher"
            partition_strategy: "hash"
            partition_key: "script_id"

            // Message-Format
            message_format: {
                type: "json"
                compression: "snappy"
                schema_version: "1.0"
            }
        }

        // Subscribers
        subscribers: [
            {
                name: "AuditLogger"
                group_id: "audit-logger"
                handler: "AuditLogHandler"
                concurrency: 3
            },
            {
                name: "CacheInvalidator"
                group_id: "cache-invalidator"
                handler: "CacheInvalidationHandler"
                concurrency: 5
            },
            {
                name: "NotificationService"
                group_id: "notification-service"
                handler: "NotificationHandler"
                concurrency: 2
            },
            {
                name: "AnalyticsProcessor"
                group_id: "analytics-processor"
                handler: "AnalyticsHandler"
                concurrency: 10
            }
        ]
    }

    // System-Events
    system_events: {
        topic: "system.events"

        publisher: {
            name: "SystemEventPublisher"
            partition_strategy: "round_robin"
        }

        subscribers: [
            {
                name: "MonitoringService"
                group_id: "monitoring-service"
                handler: "MonitoringHandler"
                concurrency: 5
            },
            {
                name: "AlertingService"
                group_id: "alerting-service"
                handler: "AlertingHandler"
                concurrency: 3
            },
            {
                name: "LogAggregator"
                group_id: "log-aggregator"
                handler: "LogAggregationHandler"
                concurrency: 8
            }
        ]
    }
}
```

### Dead Letter Queue Pattern

```hyp
// Dead Letter Queue Pattern
dead_letter_queue {
    // DLQ-Konfiguration
    dlq_config: {
        // Haupt-Queue
        main_queue: {
            name: "script-execution-queue"
            max_retries: 3
            retry_delay: 5000
            dlq_name: "script-execution-dlq"
        }

        // DLQ-Queue
        dlq_queue: {
            name: "script-execution-dlq"
            message_retention: 2592000  // 30 Tage
            max_redelivery: 1
        }
    }

    // DLQ-Handler
    dlq_handlers: {
        // Fehleranalyse
        error_analysis: {
            handler: "DLQErrorAnalysisHandler"
            concurrency: 2
            timeout: 60000

            // Fehler-Kategorisierung
            error_categories: {
                validation_error: {
                    action: "log_and_alert"
                    severity: "warning"
                },
                timeout_error: {
                    action: "retry_with_backoff"
                    max_retries: 2
                },
                system_error: {
                    action: "escalate"
                    severity: "critical"
                }
            }
        }

        // Manuelle Verarbeitung
        manual_processing: {
            handler: "DLQManualProcessingHandler"
            concurrency: 1
            timeout: 300000

            // Benutzer-Interface
            ui: {
                enabled: true
                endpoint: "/api/dlq/manual-processing"
                authentication: "required"
                authorization: "admin_only"
            }
        }
    }
}
```

## Message Reliability

### Message-Garantien

```hyp
// Message-Garantien
message_guarantees {
    // At-Least-Once Delivery
    at_least_once: {
        enabled: true

        // Producer-Garantien
        producer: {
            acks: "all"
            retries: 3
            idempotence: true
            transactional: true
        }

        // Consumer-Garantien
        consumer: {
            manual_commit: true
            commit_sync: true
            offset_commit_interval: 1000
        }
    }

    // Exactly-Once Processing
    exactly_once: {
        enabled: true

        // Idempotenz
        idempotence: {
            enabled: true
            key_strategy: "message_id"
            storage: "redis"
            ttl: 86400  // 24 Stunden
        }

        // Transaktionale Verarbeitung
        transactional: {
            enabled: true
            isolation_level: "read_committed"
            timeout: 30000
        }
    }

    // Message-Ordering
    message_ordering: {
        enabled: true

        // Partition-Key-Strategie
        partition_key: {
            strategy: "hash"
            fields: ["script_id", "user_id"]
        }

        // Consumer-Gruppen
        consumer_groups: {
            single_partition_consumers: true
            max_concurrent_partitions: 1
        }
    }
}
```

### Message-Monitoring

```hyp
// Message-Monitoring
message_monitoring {
    // Metriken
    metrics: {
        // Producer-Metriken
        producer: {
            message_count: true
            message_size: true
            send_latency: true
            error_rate: true
            retry_count: true
        }

        // Consumer-Metriken
        consumer: {
            message_count: true
            processing_latency: true
            error_rate: true
            lag: true
            commit_latency: true
        }

        // Queue-Metriken
        queue: {
            queue_size: true
            queue_depth: true
            message_age: true
            consumer_count: true
        }
    }

    // Alerting
    alerting: {
        // Consumer-Lag
        consumer_lag: {
            threshold: 1000
            alert_level: "warning"
            escalation_time: 300  // 5 Minuten
        }

        // Error-Rate
        error_rate: {
            threshold: 0.05  // 5%
            alert_level: "critical"
            window_size: 300  // 5 Minuten
        }

        // Processing-Latency
        processing_latency: {
            threshold: 30000  // 30 Sekunden
            alert_level: "warning"
            percentile: 95
        }
    }

    // Tracing
    tracing: {
        enabled: true

        // Trace-Propagation
        trace_propagation: {
            headers: ["x-trace-id", "x-span-id", "x-correlation-id"]
            baggage: true
        }

        // Span-Creation
        span_creation: {
            producer_send: true
            consumer_receive: true
            message_processing: true
        }
    }
}
```

## Best Practices

### Messaging-Best-Practices

1. **Message-Design**

   - Immutable Events verwenden
   - Schema-Versionierung implementieren
   - Backward Compatibility gewährleisten

2. **Reliability**

   - Idempotente Consumer implementieren
   - Dead Letter Queues konfigurieren
   - Retry-Policies definieren

3. **Performance**

   - Batch-Processing verwenden
   - Partitioning-Strategien optimieren
   - Consumer-Gruppen richtig konfigurieren

4. **Monitoring**

   - Consumer-Lag überwachen
   - Error-Rates tracken
   - Message-Age monitoren

5. **Security**
   - Message-Verschlüsselung aktivieren
   - Authentication/Authorization implementieren
   - Audit-Logging aktivieren

### Messaging-Checkliste

- [ ] Message Broker konfiguriert
- [ ] Event-Schemas definiert
- [ ] Producer/Consumer implementiert
- [ ] Message-Patterns ausgewählt
- [ ] Dead Letter Queues eingerichtet
- [ ] Monitoring konfiguriert
- [ ] Security implementiert
- [ ] Performance optimiert
- [ ] Error-Handling definiert
- [ ] Dokumentation erstellt

Diese Messaging- und Queuing-Funktionen stellen sicher, dass HypnoScript in Enterprise-Umgebungen skalierbare, zuverlässige und event-driven Architekturen unterstützt.

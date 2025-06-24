# Runtime Database Integration

HypnoScript bietet umfassende Datenbankintegrationsfunktionen für Runtime-Umgebungen, einschließlich Multi-Database-Support, Connection Pooling, Transaktionsmanagement und automatische Migrationen.

## Datenbankverbindungen

### Verbindungskonfiguration

```hyp
// Datenbankverbindungen
database {
    // PostgreSQL-Konfiguration
    postgresql: {
        primary: {
            host: "db-primary.example.com"
            port: 5432
            database: "hypnoscript_prod"
            username: env.DB_USERNAME
            password: env.DB_PASSWORD
            ssl_mode: "require"
            max_connections: 100
            connection_timeout: 30
        }

        replica: {
            host: "db-replica.example.com"
            port: 5432
            database: "hypnoscript_prod"
            username: env.DB_USERNAME
            password: env.DB_PASSWORD
            ssl_mode: "require"
            max_connections: 50
            read_only: true
        }
    }

    // MySQL-Konfiguration
    mysql: {
        primary: {
            host: "mysql-primary.example.com"
            port: 3306
            database: "hypnoscript"
            username: env.MYSQL_USERNAME
            password: env.MYSQL_PASSWORD
            ssl_mode: "required"
            max_connections: 80
        }
    }

    // SQL Server-Konfiguration
    sqlserver: {
        primary: {
            host: "sqlserver.example.com"
            port: 1433
            database: "HypnoScript"
            username: env.SQLSERVER_USERNAME
            password: env.SQLSERVER_PASSWORD
            encrypt: true
            trust_server_certificate: false
            max_connections: 60
        }
    }

    // Oracle-Konfiguration
    oracle: {
        primary: {
            host: "oracle.example.com"
            port: 1521
            service_name: "hypnoscript.example.com"
            username: env.ORACLE_USERNAME
            password: env.ORACLE_PASSWORD
            max_connections: 40
        }
    }
}
```

### Connection Pooling

```hyp
// Connection Pooling
connection_pooling {
    // Allgemeine Pool-Einstellungen
    general: {
        min_connections: 5
        max_connections: 100
        connection_lifetime: 3600  // 1 Stunde
        connection_idle_timeout: 300  // 5 Minuten
        connection_validation_timeout: 30
    }

    // Pool-Monitoring
    monitoring: {
        pool_usage_metrics: true
        connection_wait_time: true
        connection_creation_time: true
        connection_validation_failures: true
    }

    // Pool-Optimierung
    optimization: {
        // Load Balancing
        load_balancing: {
            strategy: "round_robin"
            health_check_interval: 30
            failover_enabled: true
        }

        // Connection Leasing
        leasing: {
            max_lease_time: 300  // 5 Minuten
            auto_return: true
            deadlock_detection: true
        }
    }
}
```

## ORM (Object-Relational Mapping)

### Entity-Definitionen

```hyp
// Entity-Modelle
entities {
    // Script-Entity
    Script: {
        table: "scripts"
        primary_key: "id"

        fields: {
            id: {
                type: "uuid"
                auto_generate: true
                primary_key: true
            }

            name: {
                type: "varchar"
                length: 255
                nullable: false
                unique: true
            }

            content: {
                type: "text"
                nullable: false
            }

            version: {
                type: "integer"
                default: 1
            }

            created_at: {
                type: "timestamp"
                default: "now()"
            }

            updated_at: {
                type: "timestamp"
                default: "now()"
                on_update: "now()"
            }

            created_by: {
                type: "uuid"
                foreign_key: "users.id"
                nullable: false
            }

            status: {
                type: "enum"
                values: ["draft", "active", "archived"]
                default: "draft"
            }

            metadata: {
                type: "jsonb"
                nullable: true
            }
        }

        indexes: [
            {
                name: "idx_scripts_name"
                columns: ["name"]
                unique: true
            },
            {
                name: "idx_scripts_created_by"
                columns: ["created_by"]
            },
            {
                name: "idx_scripts_status"
                columns: ["status"]
            },
            {
                name: "idx_scripts_created_at"
                columns: ["created_at"]
            }
        ]
    }

    // Execution-Entity
    Execution: {
        table: "script_executions"
        primary_key: "id"

        fields: {
            id: {
                type: "uuid"
                auto_generate: true
                primary_key: true
            }

            script_id: {
                type: "uuid"
                foreign_key: "scripts.id"
                nullable: false
            }

            user_id: {
                type: "uuid"
                foreign_key: "users.id"
                nullable: false
            }

            started_at: {
                type: "timestamp"
                default: "now()"
            }

            completed_at: {
                type: "timestamp"
                nullable: true
            }

            duration_ms: {
                type: "bigint"
                nullable: true
            }

            status: {
                type: "enum"
                values: ["running", "completed", "failed", "cancelled"]
                default: "running"
            }

            result: {
                type: "jsonb"
                nullable: true
            }

            error_message: {
                type: "text"
                nullable: true
            }

            environment: {
                type: "varchar"
                length: 50
                default: "production"
            }

            metadata: {
                type: "jsonb"
                nullable: true
            }
        }

        indexes: [
            {
                name: "idx_executions_script_id"
                columns: ["script_id"]
            },
            {
                name: "idx_executions_user_id"
                columns: ["user_id"]
            },
            {
                name: "idx_executions_started_at"
                columns: ["started_at"]
            },
            {
                name: "idx_executions_status"
                columns: ["status"]
            }
        ]
    }

    // User-Entity
    User: {
        table: "users"
        primary_key: "id"

        fields: {
            id: {
                type: "uuid"
                auto_generate: true
                primary_key: true
            }

            email: {
                type: "varchar"
                length: 255
                nullable: false
                unique: true
            }

            username: {
                type: "varchar"
                length: 100
                nullable: false
                unique: true
            }

            password_hash: {
                type: "varchar"
                length: 255
                nullable: false
            }

            first_name: {
                type: "varchar"
                length: 100
                nullable: true
            }

            last_name: {
                type: "varchar"
                length: 100
                nullable: true
            }

            is_active: {
                type: "boolean"
                default: true
            }

            last_login: {
                type: "timestamp"
                nullable: true
            }

            created_at: {
                type: "timestamp"
                default: "now()"
            }

            updated_at: {
                type: "timestamp"
                default: "now()"
                on_update: "now()"
            }
        }

        indexes: [
            {
                name: "idx_users_email"
                columns: ["email"]
                unique: true
            },
            {
                name: "idx_users_username"
                columns: ["username"]
                unique: true
            },
            {
                name: "idx_users_is_active"
                columns: ["is_active"]
            }
        ]
    }
}
```

### Repository-Pattern

```hyp
// Repository-Implementierungen
repositories {
    // Script-Repository
    ScriptRepository: {
        entity: "Script"

        methods: {
            // Standard-CRUD-Operationen
            findById: {
                sql: "SELECT * FROM scripts WHERE id = ?"
                parameters: ["id"]
                return_type: "Script"
            }

            findByName: {
                sql: "SELECT * FROM scripts WHERE name = ?"
                parameters: ["name"]
                return_type: "Script"
            }

            findByStatus: {
                sql: "SELECT * FROM scripts WHERE status = ? ORDER BY created_at DESC"
                parameters: ["status"]
                return_type: "Script[]"
            }

            findByCreator: {
                sql: "SELECT * FROM scripts WHERE created_by = ? ORDER BY created_at DESC"
                parameters: ["user_id"]
                return_type: "Script[]"
            }

            search: {
                sql: "SELECT * FROM scripts WHERE name ILIKE ? OR content ILIKE ? ORDER BY created_at DESC"
                parameters: ["%search_term%", "%search_term%"]
                return_type: "Script[]"
            }

            create: {
                sql: "INSERT INTO scripts (id, name, content, version, created_by, status, metadata) VALUES (?, ?, ?, ?, ?, ?, ?)"
                parameters: ["id", "name", "content", "version", "created_by", "status", "metadata"]
                return_type: "Script"
            }

            update: {
                sql: "UPDATE scripts SET name = ?, content = ?, version = ?, status = ?, metadata = ?, updated_at = now() WHERE id = ?"
                parameters: ["name", "content", "version", "status", "metadata", "id"]
                return_type: "boolean"
            }

            delete: {
                sql: "DELETE FROM scripts WHERE id = ?"
                parameters: ["id"]
                return_type: "boolean"
            }

            // Spezielle Abfragen
            getExecutionStats: {
                sql: """
                    SELECT
                        s.id,
                        s.name,
                        COUNT(e.id) as execution_count,
                        AVG(e.duration_ms) as avg_duration,
                        MAX(e.started_at) as last_execution
                    FROM scripts s
                    LEFT JOIN script_executions e ON s.id = e.script_id
                    WHERE s.created_by = ?
                    GROUP BY s.id, s.name
                    ORDER BY execution_count DESC
                """
                parameters: ["user_id"]
                return_type: "ScriptStats[]"
            }

            getPopularScripts: {
                sql: """
                    SELECT
                        s.id,
                        s.name,
                        COUNT(e.id) as execution_count
                    FROM scripts s
                    JOIN script_executions e ON s.id = e.script_id
                    WHERE e.started_at >= NOW() - INTERVAL '30 days'
                    GROUP BY s.id, s.name
                    ORDER BY execution_count DESC
                    LIMIT 10
                """
                parameters: []
                return_type: "PopularScript[]"
            }
        }
    }

    // Execution-Repository
    ExecutionRepository: {
        entity: "Execution"

        methods: {
            findById: {
                sql: "SELECT * FROM script_executions WHERE id = ?"
                parameters: ["id"]
                return_type: "Execution"
            }

            findByScript: {
                sql: "SELECT * FROM script_executions WHERE script_id = ? ORDER BY started_at DESC"
                parameters: ["script_id"]
                return_type: "Execution[]"
            }

            findByUser: {
                sql: "SELECT * FROM script_executions WHERE user_id = ? ORDER BY started_at DESC"
                parameters: ["user_id"]
                return_type: "Execution[]"
            }

            findByStatus: {
                sql: "SELECT * FROM script_executions WHERE status = ? ORDER BY started_at DESC"
                parameters: ["status"]
                return_type: "Execution[]"
            }

            getRunningExecutions: {
                sql: "SELECT * FROM script_executions WHERE status = 'running' ORDER BY started_at ASC"
                parameters: []
                return_type: "Execution[]"
            }

            create: {
                sql: "INSERT INTO script_executions (id, script_id, user_id, status, environment, metadata) VALUES (?, ?, ?, ?, ?, ?)"
                parameters: ["id", "script_id", "user_id", "status", "environment", "metadata"]
                return_type: "Execution"
            }

            updateStatus: {
                sql: "UPDATE script_executions SET status = ?, completed_at = ?, duration_ms = ?, result = ?, error_message = ? WHERE id = ?"
                parameters: ["status", "completed_at", "duration_ms", "result", "error_message", "id"]
                return_type: "boolean"
            }

            // Performance-Abfragen
            getPerformanceStats: {
                sql: """
                    SELECT
                        DATE_TRUNC('hour', started_at) as hour,
                        COUNT(*) as execution_count,
                        AVG(duration_ms) as avg_duration,
                        MAX(duration_ms) as max_duration,
                        COUNT(CASE WHEN status = 'failed' THEN 1 END) as error_count
                    FROM script_executions
                    WHERE started_at >= NOW() - INTERVAL '24 hours'
                    GROUP BY DATE_TRUNC('hour', started_at)
                    ORDER BY hour
                """
                parameters: []
                return_type: "PerformanceStats[]"
            }
        }
    }
}
```

## Transaktionsmanagement

### Transaktions-Konfiguration

```hyp
// Transaktionsmanagement
transactions {
    // Transaktions-Einstellungen
    settings: {
        default_isolation_level: "read_committed"
        default_timeout: 30  // Sekunden
        max_retries: 3
        retry_delay: 1000  // Millisekunden
    }

    // Transaktions-Templates
    templates: {
        // Script-Erstellung mit Validierung
        createScript: {
            isolation_level: "serializable"
            timeout: 60
            retry_policy: {
                max_retries: 3
                backoff_strategy: "exponential"
            }

            steps: [
                {
                    name: "validate_script"
                    operation: "validate_script_content"
                    rollback_on_failure: true
                },
                {
                    name: "check_duplicate_name"
                    operation: "check_script_name_unique"
                    rollback_on_failure: true
                },
                {
                    name: "create_script"
                    operation: "insert_script"
                    rollback_on_failure: true
                },
                {
                    name: "create_audit_log"
                    operation: "insert_audit_log"
                    rollback_on_failure: false
                }
            ]
        }

        // Script-Ausführung
        executeScript: {
            isolation_level: "read_committed"
            timeout: 300

            steps: [
                {
                    name: "create_execution_record"
                    operation: "insert_execution"
                    rollback_on_failure: true
                },
                {
                    name: "execute_script"
                    operation: "run_script"
                    rollback_on_failure: true
                },
                {
                    name: "update_execution_result"
                    operation: "update_execution"
                    rollback_on_failure: false
                },
                {
                    name: "log_execution"
                    operation: "insert_execution_log"
                    rollback_on_failure: false
                }
            ]
        }
    }
}
```

### Transaktions-Beispiele

```hyp
// Transaktions-Beispiele
transaction_examples {
    // Script mit Abhängigkeiten erstellen
    createScriptWithDependencies: {
        description: "Erstellt ein Script mit allen Abhängigkeiten in einer Transaktion"

        transaction: {
            isolation_level: "serializable"
            timeout: 120

            operations: [
                {
                    name: "create_script"
                    sql: "INSERT INTO scripts (id, name, content, created_by) VALUES (?, ?, ?, ?)"
                    parameters: ["script_id", "script_name", "script_content", "user_id"]
                },
                {
                    name: "create_dependencies"
                    sql: "INSERT INTO script_dependencies (script_id, dependency_id) VALUES (?, ?)"
                    parameters: ["script_id", "dependency_ids"]
                    loop: "dependency_ids"
                },
                {
                    name: "create_permissions"
                    sql: "INSERT INTO script_permissions (script_id, user_id, permission) VALUES (?, ?, ?)"
                    parameters: ["script_id", "user_ids", "permissions"]
                    loop: "user_permissions"
                }
            ]

            rollback: {
                on_failure: true
                cleanup_operations: [
                    "DELETE FROM script_dependencies WHERE script_id = ?",
                    "DELETE FROM script_permissions WHERE script_id = ?",
                    "DELETE FROM scripts WHERE id = ?"
                ]
            }
        }
    }

    // Batch-Script-Ausführung
    batchScriptExecution: {
        description: "Führt mehrere Scripts in einer Batch-Transaktion aus"

        transaction: {
            isolation_level: "read_committed"
            timeout: 600

            operations: [
                {
                    name: "create_batch_record"
                    sql: "INSERT INTO batch_executions (id, user_id, script_count) VALUES (?, ?, ?)"
                    parameters: ["batch_id", "user_id", "script_count"]
                },
                {
                    name: "execute_scripts"
                    operation: "execute_script_batch"
                    parameters: ["script_ids", "batch_id"]
                    loop: "script_ids"
                },
                {
                    name: "update_batch_status"
                    sql: "UPDATE batch_executions SET status = 'completed', completed_at = now() WHERE id = ?"
                    parameters: ["batch_id"]
                }
            ]

            rollback: {
                on_failure: true
                cleanup_operations: [
                    "UPDATE batch_executions SET status = 'failed' WHERE id = ?",
                    "UPDATE script_executions SET status = 'cancelled' WHERE batch_id = ?"
                ]
            }
        }
    }
}
```

## Datenbank-Migrationen

### Migrations-System

```hyp
// Migrations-Konfiguration
migrations {
    // Migrations-Einstellungen
    settings: {
        table_name: "schema_migrations"
        version_column: "version"
        applied_at_column: "applied_at"
        checksum_column: "checksum"

        // Migrations-Verzeichnis
        directory: "migrations"

        // Versionierung
        version_format: "timestamp"
        version_separator: "_"
    }

    // Migrations-Templates
    templates: {
        // Tabelle erstellen
        create_table: {
            template: """
                CREATE TABLE {table_name} (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    created_at TIMESTAMP DEFAULT NOW(),
                    updated_at TIMESTAMP DEFAULT NOW()
                );

                CREATE INDEX idx_{table_name}_created_at ON {table_name}(created_at);
            """
        }

        // Index erstellen
        create_index: {
            template: "CREATE INDEX {index_name} ON {table_name}({columns});"
        }

        // Foreign Key hinzufügen
        add_foreign_key: {
            template: "ALTER TABLE {table_name} ADD CONSTRAINT {constraint_name} FOREIGN KEY ({column}) REFERENCES {referenced_table}({referenced_column});"
        }
    }
}
```

### Migrations-Beispiele

```hyp
// Migrations-Beispiele
migration_examples {
    // Initiale Schema-Erstellung
    initial_schema: {
        version: "20240101000001"
        description: "Initial schema creation"

        up: [
            """
            CREATE TABLE users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                email VARCHAR(255) UNIQUE NOT NULL,
                username VARCHAR(100) UNIQUE NOT NULL,
                password_hash VARCHAR(255) NOT NULL,
                first_name VARCHAR(100),
                last_name VARCHAR(100),
                is_active BOOLEAN DEFAULT true,
                last_login TIMESTAMP,
                created_at TIMESTAMP DEFAULT NOW(),
                updated_at TIMESTAMP DEFAULT NOW()
            );
            """,
            """
            CREATE TABLE scripts (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(255) UNIQUE NOT NULL,
                content TEXT NOT NULL,
                version INTEGER DEFAULT 1,
                created_at TIMESTAMP DEFAULT NOW(),
                updated_at TIMESTAMP DEFAULT NOW(),
                created_by UUID NOT NULL REFERENCES users(id),
                status VARCHAR(50) DEFAULT 'draft',
                metadata JSONB
            );
            """,
            """
            CREATE TABLE script_executions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                script_id UUID NOT NULL REFERENCES scripts(id),
                user_id UUID NOT NULL REFERENCES users(id),
                started_at TIMESTAMP DEFAULT NOW(),
                completed_at TIMESTAMP,
                duration_ms BIGINT,
                status VARCHAR(50) DEFAULT 'running',
                result JSONB,
                error_message TEXT,
                environment VARCHAR(50) DEFAULT 'production',
                metadata JSONB
            );
            """
        ]

        down: [
            "DROP TABLE IF EXISTS script_executions;",
            "DROP TABLE IF EXISTS scripts;",
            "DROP TABLE IF EXISTS users;"
        ]
    }

    // Performance-Optimierungen
    performance_optimizations: {
        version: "20240102000001"
        description: "Add performance indexes and optimizations"

        up: [
            "CREATE INDEX idx_scripts_created_by ON scripts(created_by);",
            "CREATE INDEX idx_scripts_status ON scripts(status);",
            "CREATE INDEX idx_scripts_created_at ON scripts(created_at);",
            "CREATE INDEX idx_executions_script_id ON script_executions(script_id);",
            "CREATE INDEX idx_executions_user_id ON script_executions(user_id);",
            "CREATE INDEX idx_executions_started_at ON script_executions(started_at);",
            "CREATE INDEX idx_executions_status ON script_executions(status);",
            "CREATE INDEX idx_users_email ON users(email);",
            "CREATE INDEX idx_users_username ON users(username);",
            "CREATE INDEX idx_users_is_active ON users(is_active);"
        ]

        down: [
            "DROP INDEX IF EXISTS idx_scripts_created_by;",
            "DROP INDEX IF EXISTS idx_scripts_status;",
            "DROP INDEX IF EXISTS idx_scripts_created_at;",
            "DROP INDEX IF EXISTS idx_executions_script_id;",
            "DROP INDEX IF EXISTS idx_executions_user_id;",
            "DROP INDEX IF EXISTS idx_executions_started_at;",
            "DROP INDEX IF EXISTS idx_executions_status;",
            "DROP INDEX IF EXISTS idx_users_email;",
            "DROP INDEX IF EXISTS idx_users_username;",
            "DROP INDEX IF EXISTS idx_users_is_active;"
        ]
    }

    // Audit-Logging hinzufügen
    add_audit_logging: {
        version: "20240103000001"
        description: "Add audit logging tables"

        up: [
            """
            CREATE TABLE audit_logs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID REFERENCES users(id),
                action VARCHAR(100) NOT NULL,
                table_name VARCHAR(100) NOT NULL,
                record_id UUID,
                old_values JSONB,
                new_values JSONB,
                ip_address INET,
                user_agent TEXT,
                created_at TIMESTAMP DEFAULT NOW()
            );
            """,
            "CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);",
            "CREATE INDEX idx_audit_logs_action ON audit_logs(action);",
            "CREATE INDEX idx_audit_logs_table_name ON audit_logs(table_name);",
            "CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);"
        ]

        down: [
            "DROP INDEX IF EXISTS idx_audit_logs_created_at;",
            "DROP INDEX IF EXISTS idx_audit_logs_table_name;",
            "DROP INDEX IF EXISTS idx_audit_logs_action;",
            "DROP INDEX IF EXISTS idx_audit_logs_user_id;",
            "DROP TABLE IF EXISTS audit_logs;"
        ]
    }
}
```

## Datenbank-Optimierung

### Performance-Optimierung

```hyp
// Datenbank-Optimierung
database_optimization {
    // Query-Optimierung
    query_optimization: {
        // Query-Caching
        query_cache: {
            enabled: true
            max_size: 1000
            ttl: 300  // 5 Minuten
            cache_key_strategy: "sql_hash"
        }

        // Prepared Statements
        prepared_statements: {
            enabled: true
            max_prepared_statements: 100
            statement_timeout: 30
        }

        // Query-Analyse
        query_analysis: {
            slow_query_threshold: 1000  // Millisekunden
            log_slow_queries: true
            explain_plans: true
        }
    }

    // Index-Optimierung
    index_optimization: {
        // Automatische Index-Empfehlungen
        auto_recommendations: {
            enabled: true
            analysis_interval: "daily"
            min_query_frequency: 10
        }

        // Index-Monitoring
        index_monitoring: {
            unused_indexes: true
            duplicate_indexes: true
            index_fragmentation: true
        }
    }

    // Partitionierung
    partitioning: {
        // Zeitbasierte Partitionierung
        time_based: {
            table: "script_executions"
            partition_column: "started_at"
            partition_interval: "month"
            retention_period: "12 months"
        }

        // Hash-Partitionierung
        hash_based: {
            table: "audit_logs"
            partition_column: "id"
            partition_count: 8
        }
    }
}
```

## Best Practices

### Datenbank-Best-Practices

1. **Verbindungsmanagement**

   - Connection Pooling verwenden
   - Verbindungen ordnungsgemäß schließen
   - Timeouts konfigurieren

2. **Transaktionsmanagement**

   - Kurze Transaktionen bevorzugen
   - Isolation Levels bewusst wählen
   - Rollback-Strategien definieren

3. **Query-Optimierung**

   - Indizes strategisch platzieren
   - N+1 Query Problem vermeiden
   - Prepared Statements verwenden

4. **Sicherheit**

   - SQL Injection verhindern
   - Parameterized Queries verwenden
   - Berechtigungen minimieren

5. **Monitoring**
   - Query-Performance überwachen
   - Connection Pool-Metriken tracken
   - Slow Query-Logging aktivieren

### Datenbank-Checkliste

- [ ] Verbindungskonfiguration getestet
- [ ] Connection Pooling konfiguriert
- [ ] Entity-Modelle definiert
- [ ] Repository-Pattern implementiert
- [ ] Transaktionsmanagement eingerichtet
- [ ] Migrations-System konfiguriert
- [ ] Performance-Optimierungen implementiert
- [ ] Backup-Strategie definiert
- [ ] Monitoring konfiguriert
- [ ] Sicherheitsrichtlinien umgesetzt

Diese Datenbankintegrationsfunktionen stellen sicher, dass HypnoScript in Runtime-Umgebungen effizient und sicher mit verschiedenen Datenbanksystemen arbeitet.

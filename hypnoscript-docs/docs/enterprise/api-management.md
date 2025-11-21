# Runtime API Management

HypnoScript provides comprehensive API management features for runtime environments, including API design, versioning, rate limiting, authentication, and comprehensive documentation.

## API Design

### RESTful API Structure

```hyp
// API Base Configuration
api {
    // Base URL Configuration
    base_url: {
        development: "http://localhost:8080/api/v1"
        staging: "https://api-staging.example.com/api/v1"
        production: "https://api.example.com/api/v1"
    }

    // API Versioning
    versioning: {
        strategy: "url_path"
        current_version: "v1"
        supported_versions: ["v1", "v2"]
        deprecated_versions: ["v0"]

        // Version Migration
        migration: {
            grace_period: 365  // days
            notification_interval: 30  // days
            auto_redirect: true
        }
    }

    // Content-Type Configuration
    content_types: {
        request: ["application/json", "application/xml"]
        response: ["application/json", "application/xml"]
        default: "application/json"
    }
}
```

### Endpoint Definitions

```hyp
// API Endpoints
endpoints {
    // Script Management
    scripts: {
        // List scripts
        list: {
            method: "GET"
            path: "/scripts"
            description: "Retrieve list of all scripts"

            // Query Parameters
            query_params: {
                page: {
                    type: "integer"
                    default: 1
                    min: 1
                    description: "Page number"
                }

                size: {
                    type: "integer"
                    default: 20
                    min: 1
                    max: 100
                    description: "Number of entries per page"
                }

                status: {
                    type: "string"
                    enum: ["draft", "active", "archived"]
                    description: "Filter by script status"
                }

                created_by: {
                    type: "uuid"
                    description: "Filter by creator"
                }

                search: {
                    type: "string"
                    min_length: 2
                    description: "Search in name and content"
                }

                sort: {
                    type: "string"
                    enum: ["name", "created_at", "updated_at", "execution_count"]
                    default: "created_at"
                    description: "Sort field"
                }

                order: {
                    type: "string"
                    enum: ["asc", "desc"]
                    default: "desc"
                    description: "Sort order"
                }
            }

            // Response Schema
            response: {
                200: {
                    description: "Successful query"
                    schema: {
                        type: "object"
                        properties: {
                            data: {
                                type: "array"
                                items: {
                                    $ref: "#/components/schemas/Script"
                                }
                            }
                            pagination: {
                                $ref: "#/components/schemas/Pagination"
                            }
                            meta: {
                                $ref: "#/components/schemas/Meta"
                            }
                        }
                    }
                }

                400: {
                    description: "Invalid parameters"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }

                401: {
                    description: "Not authenticated"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }

                403: {
                    description: "No permission"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }

        // Create script
        create: {
            method: "POST"
            path: "/scripts"
            description: "Create new script"

            // Request Schema
            request: {
                content_type: "application/json"
                schema: {
                    type: "object"
                    required: ["name", "content"]
                    properties: {
                        name: {
                            type: "string"
                            min_length: 1
                            max_length: 255
                            pattern: "^[a-zA-Z0-9_\\-\\.]+$"
                            description: "Unique script name"
                        }

                        content: {
                            type: "string"
                            min_length: 1
                            max_length: 100000
                            description: "Script content"
                        }

                        description: {
                            type: "string"
                            max_length: 1000
                            description: "Script description"
                        }

                        tags: {
                            type: "array"
                            items: {
                                type: "string"
                                max_length: 50
                            }
                            max_items: 10
                            description: "Script tags"
                        }

                        metadata: {
                            type: "object"
                            description: "Additional metadata"
                        }
                    }
                }
            }

            // Response Schema
            response: {
                201: {
                    description: "Script successfully created"
                    schema: {
                        $ref: "#/components/schemas/Script"
                    }
                }

                400: {
                    description: "Invalid input data"
                    schema: {
                        $ref: "#/components/schemas/ValidationError"
                    }
                }

                409: {
                    description: "Script name already exists"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }

        // Retrieve script
        get: {
            method: "GET"
            path: "/scripts/{script_id}"
            description: "Retrieve single script"

            // Path Parameters
            path_params: {
                script_id: {
                    type: "uuid"
                    description: "Script ID"
                }
            }

            // Response Schema
            response: {
                200: {
                    description: "Script found"
                    schema: {
                        $ref: "#/components/schemas/Script"
                    }
                }

                404: {
                    description: "Script not found"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }

        // Update script
        update: {
            method: "PUT"
            path: "/scripts/{script_id}"
            description: "Update script"

            path_params: {
                script_id: {
                    type: "uuid"
                    description: "Script ID"
                }
            }

            request: {
                content_type: "application/json"
                schema: {
                    type: "object"
                    properties: {
                        name: {
                            type: "string"
                            min_length: 1
                            max_length: 255
                            pattern: "^[a-zA-Z0-9_\\-\\.]+$"
                        }

                        content: {
                            type: "string"
                            min_length: 1
                            max_length: 100000
                        }

                        description: {
                            type: "string"
                            max_length: 1000
                        }

                        tags: {
                            type: "array"
                            items: {
                                type: "string"
                                max_length: 50
                            }
                            max_items: 10
                        }

                        metadata: {
                            type: "object"
                        }
                    }
                }
            }

            response: {
                200: {
                    description: "Script successfully updated"
                    schema: {
                        $ref: "#/components/schemas/Script"
                    }
                }

                404: {
                    description: "Script not found"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }

        // Delete script
        delete: {
            method: "DELETE"
            path: "/scripts/{script_id}"
            description: "Delete script"

            path_params: {
                script_id: {
                    type: "uuid"
                    description: "Script ID"
                }
            }

            response: {
                204: {
                    description: "Script successfully deleted"
                }

                404: {
                    description: "Script not found"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }
    }

    // Script Execution
    executions: {
        // Execute script
        execute: {
            method: "POST"
            path: "/scripts/{script_id}/execute"
            description: "Execute script"

            path_params: {
                script_id: {
                    type: "uuid"
                    description: "Script ID"
                }
            }

            request: {
                content_type: "application/json"
                schema: {
                    type: "object"
                    properties: {
                        parameters: {
                            type: "object"
                            description: "Script parameters"
                        }

                        timeout: {
                            type: "integer"
                            min: 1
                            max: 3600
                            default: 300
                            description: "Timeout in seconds"
                        }

                        environment: {
                            type: "string"
                            enum: ["development", "staging", "production"]
                            default: "production"
                            description: "Execution environment"
                        }

                        metadata: {
                            type: "object"
                            description: "Additional metadata"
                        }
                    }
                }
            }

            response: {
                202: {
                    description: "Execution started"
                    schema: {
                        type: "object"
                        properties: {
                            execution_id: {
                                type: "uuid"
                                description: "Execution ID"
                            }

                            status: {
                                type: "string"
                                enum: ["queued", "running"]
                                description: "Execution status"
                            }

                            estimated_duration: {
                                type: "integer"
                                description: "Estimated duration in seconds"
                            }
                        }
                    }
                }

                404: {
                    description: "Script not found"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }

                422: {
                    description: "Script cannot be executed"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }

        // Get execution status
        get_status: {
            method: "GET"
            path: "/executions/{execution_id}"
            description: "Get execution status"

            path_params: {
                execution_id: {
                    type: "uuid"
                    description: "Execution ID"
                }
            }

            response: {
                200: {
                    description: "Execution status"
                    schema: {
                        $ref: "#/components/schemas/Execution"
                    }
                }

                404: {
                    description: "Execution not found"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }

        // Cancel execution
        cancel: {
            method: "POST"
            path: "/executions/{execution_id}/cancel"
            description: "Cancel execution"

            path_params: {
                execution_id: {
                    type: "uuid"
                    description: "Execution ID"
                }
            }

            response: {
                200: {
                    description: "Execution cancelled"
                    schema: {
                        $ref: "#/components/schemas/Execution"
                    }
                }

                404: {
                    description: "Execution not found"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }

                409: {
                    description: "Execution cannot be cancelled"
                    schema: {
                        $ref: "#/components/schemas/Error"
                    }
                }
            }
        }
    }
}
```

## API Security

### Authentication

```hyp
// API Authentication
authentication {
    // OAuth2 Configuration
    oauth2: {
        enabled: true

        // Authorization Server
        authorization_server: {
            issuer: "https://auth.example.com"
            authorization_endpoint: "https://auth.example.com/oauth/authorize"
            token_endpoint: "https://auth.example.com/oauth/token"
            introspection_endpoint: "https://auth.example.com/oauth/introspect"
            revocation_endpoint: "https://auth.example.com/oauth/revoke"
        }

        // Client Configuration
        client: {
            client_id: env.OAUTH_CLIENT_ID
            client_secret: env.OAUTH_CLIENT_SECRET
            redirect_uri: "https://api.example.com/oauth/callback"

            // Scopes
            scopes: [
                "read:scripts",
                "write:scripts",
                "execute:scripts",
                "read:executions",
                "admin:scripts"
            ]
        }

        // Token Configuration
        token: {
            access_token_lifetime: 3600  // 1 hour
            refresh_token_lifetime: 2592000  // 30 days
            token_type: "Bearer"
        }
    }

    // API Key Authentication
    api_key: {
        enabled: true

        // API Key Header
        header_name: "X-API-Key"

        // API Key Validation
        validation: {
            key_format: "uuid"
            key_length: 36
            check_expiration: true
            check_revocation: true
        }

        // API Key Permissions
        permissions: {
            "read:scripts": ["GET /api/v1/scripts", "GET /api/v1/scripts/{id}"]
            "write:scripts": ["POST /api/v1/scripts", "PUT /api/v1/scripts/{id}", "DELETE /api/v1/scripts/{id}"]
            "execute:scripts": ["POST /api/v1/scripts/{id}/execute"]
            "read:executions": ["GET /api/v1/executions/{id}"]
            "admin:scripts": ["*"]
        }
    }

    // JWT Authentication
    jwt: {
        enabled: true

        // JWT Configuration
        configuration: {
            issuer: "hypnoscript-api"
            audience: "hypnoscript-clients"
            signing_algorithm: "RS256"
            public_key_url: "https://auth.example.com/.well-known/jwks.json"
        }

        // Token Validation
        validation: {
            validate_issuer: true
            validate_audience: true
            validate_expiration: true
            validate_signature: true
            clock_skew: 30  // seconds
        }
    }
}
```

### Authorization

```hyp
// API Authorization
authorization {
    // Role-Based Access Control (RBAC)
    rbac: {
        enabled: true

        // Role Definitions
        roles: {
            admin: {
                permissions: ["*"]
                description: "Full access to all API endpoints"
            }

            developer: {
                permissions: [
                    "read:scripts",
                    "write:scripts",
                    "execute:scripts",
                    "read:executions"
                ]
                description: "Developer with script access"
            }

            analyst: {
                permissions: [
                    "read:scripts",
                    "read:executions"
                ]
                description: "Analyst with read access"
            }

            viewer: {
                permissions: [
                    "read:scripts"
                ]
                description: "Read-only access to scripts"
            }
        }

        // User Role Assignment
        user_roles: {
            "john.doe@example.com": ["admin"]
            "jane.smith@example.com": ["developer", "analyst"]
            "bob.wilson@example.com": ["viewer"]
        }
    }

    // Attribute-Based Access Control (ABAC)
    abac: {
        enabled: true

        // ABAC Policies
        policies: {
            script_access: {
                condition: {
                    user.department == resource.department &&
                    user.security_level >= resource.classification &&
                    time.hour >= 8 && time.hour <= 18
                }
                action: "allow"
                resource: "scripts"
            }

            script_execution: {
                condition: {
                    user.role in ["admin", "developer"] &&
                    script.risk_level <= user.max_risk_level &&
                    environment == "production" ? user.prod_access : true
                }
                action: "allow"
                resource: "script_execution"
            }
        }
    }
}
```

## Rate Limiting

### Rate Limiting Configuration

```hyp
// Rate Limiting
rate_limiting {
    // General Settings
    general: {
        enabled: true
        storage: "redis"
        redis_url: env.REDIS_URL

        // Default Limits
        default_limits: {
            requests_per_minute: 100
            requests_per_hour: 1000
            requests_per_day: 10000
        }
    }

    // Endpoint-Specific Limits
    endpoint_limits: {
        // Script List
        "GET /api/v1/scripts": {
            requests_per_minute: 200
            requests_per_hour: 2000
            requests_per_day: 20000
        }

        // Script Creation
        "POST /api/v1/scripts": {
            requests_per_minute: 10
            requests_per_hour: 100
            requests_per_day: 1000
        }

        // Script Execution
        "POST /api/v1/scripts/{id}/execute": {
            requests_per_minute: 5
            requests_per_hour: 50
            requests_per_day: 500
        }

        // Script Deletion
        "DELETE /api/v1/scripts/{id}": {
            requests_per_minute: 2
            requests_per_hour: 20
            requests_per_day: 200
        }
    }

    // User-Specific Limits
    user_limits: {
        // Premium Users
        premium: {
            requests_per_minute: 500
            requests_per_hour: 5000
            requests_per_day: 50000
        }

        // Enterprise Users
        enterprise: {
            requests_per_minute: 1000
            requests_per_hour: 10000
            requests_per_day: 100000
        }
    }

    // Rate Limiting Headers
    headers: {
        enabled: true
        limit_header: "X-RateLimit-Limit"
        remaining_header: "X-RateLimit-Remaining"
        reset_header: "X-RateLimit-Reset"
        retry_after_header: "Retry-After"
    }

    // Rate Limiting Responses
    responses: {
        429: {
            description: "Rate limit exceeded"
            schema: {
                type: "object"
                properties: {
                    error: {
                        type: "string"
                        example: "Rate limit exceeded"
                    }

                    retry_after: {
                        type: "integer"
                        description: "Seconds until next attempt"
                    }

                    limit: {
                        type: "integer"
                        description: "Current limit"
                    }

                    remaining: {
                        type: "integer"
                        description: "Remaining requests"
                    }
                }
            }
        }
    }
}
```

## API Documentation

### OpenAPI Specification

```hyp
// OpenAPI Configuration
openapi {
    // Basic Information
    info: {
        title: "HypnoScript API"
        version: "1.0.0"
        description: "Runtime API for HypnoScript scripting and execution"
        contact: {
            name: "HypnoScript Support"
            email: "api-support@example.com"
            url: "https://docs.example.com/api"
        }
        license: {
            name: "MIT"
            url: "https://opensource.org/licenses/MIT"
        }
    }

    // Server Configuration
    servers: [
        {
            url: "https://api.example.com/api/v1"
            description: "Production Server"
        },
        {
            url: "https://api-staging.example.com/api/v1"
            description: "Staging Server"
        },
        {
            url: "http://localhost:8080/api/v1"
            description: "Development Server"
        }
    ]

    // Security Schemes
    security_schemes: {
        oauth2: {
            type: "oauth2"
            flows: {
                authorizationCode: {
                    authorizationUrl: "https://auth.example.com/oauth/authorize"
                    tokenUrl: "https://auth.example.com/oauth/token"
                    scopes: {
                        "read:scripts": "Read scripts"
                        "write:scripts": "Create and edit scripts"
                        "execute:scripts": "Execute scripts"
                        "read:executions": "Read executions"
                        "admin:scripts": "Full access to scripts"
                    }
                }
            }
        }

        apiKey: {
            type: "apiKey"
            in: "header"
            name: "X-API-Key"
            description: "API Key for authentication"
        }

        bearerAuth: {
            type: "http"
            scheme: "bearer"
            bearerFormat: "JWT"
            description: "JWT Token for authentication"
        }
    }

    // Global Security
    security: [
        {
            oauth2: ["read:scripts"]
        },
        {
            apiKey: []
        },
        {
            bearerAuth: []
        }
    ]

    // Component Schemas
    components: {
        schemas: {
            Script: {
                type: "object"
                properties: {
                    id: {
                        type: "string"
                        format: "uuid"
                        description: "Unique script ID"
                    }

                    name: {
                        type: "string"
                        description: "Script name"
                    }

                    content: {
                        type: "string"
                        description: "Script content"
                    }

                    description: {
                        type: "string"
                        description: "Script description"
                    }

                    version: {
                        type: "integer"
                        description: "Script version"
                    }

                    status: {
                        type: "string"
                        enum: ["draft", "active", "archived"]
                        description: "Script status"
                    }

                    tags: {
                        type: "array"
                        items: {
                            type: "string"
                        }
                        description: "Script tags"
                    }

                    created_by: {
                        type: "string"
                        format: "uuid"
                        description: "Creator ID"
                    }

                    created_at: {
                        type: "string"
                        format: "date-time"
                        description: "Creation date"
                    }

                    updated_at: {
                        type: "string"
                        format: "date-time"
                        description: "Update date"
                    }

                    metadata: {
                        type: "object"
                        description: "Additional metadata"
                    }
                }
                required: ["id", "name", "content", "version", "status", "created_by", "created_at"]
            }

            Execution: {
                type: "object"
                properties: {
                    id: {
                        type: "string"
                        format: "uuid"
                        description: "Unique execution ID"
                    }

                    script_id: {
                        type: "string"
                        format: "uuid"
                        description: "Script ID"
                    }

                    user_id: {
                        type: "string"
                        format: "uuid"
                        description: "User ID"
                    }

                    status: {
                        type: "string"
                        enum: ["queued", "running", "completed", "failed", "cancelled"]
                        description: "Execution status"
                    }

                    started_at: {
                        type: "string"
                        format: "date-time"
                        description: "Start time"
                    }

                    completed_at: {
                        type: "string"
                        format: "date-time"
                        description: "End time"
                    }

                    duration_ms: {
                        type: "integer"
                        description: "Execution duration in milliseconds"
                    }

                    result: {
                        type: "object"
                        description: "Execution result"
                    }

                    error_message: {
                        type: "string"
                        description: "Error message"
                    }

                    environment: {
                        type: "string"
                        enum: ["development", "staging", "production"]
                        description: "Execution environment"
                    }

                    metadata: {
                        type: "object"
                        description: "Additional metadata"
                    }
                }
                required: ["id", "script_id", "user_id", "status", "started_at"]
            }

            Error: {
                type: "object"
                properties: {
                    error: {
                        type: "string"
                        description: "Error type"
                    }

                    message: {
                        type: "string"
                        description: "Error message"
                    }

                    code: {
                        type: "string"
                        description: "Error code"
                    }

                    details: {
                        type: "object"
                        description: "Additional error details"
                    }

                    timestamp: {
                        type: "string"
                        format: "date-time"
                        description: "Error timestamp"
                    }

                    request_id: {
                        type: "string"
                        description: "Request ID for tracing"
                    }
                }
                required: ["error", "message", "timestamp"]
            }

            ValidationError: {
                type: "object"
                properties: {
                    error: {
                        type: "string"
                        example: "validation_error"
                    }

                    message: {
                        type: "string"
                        example: "Validation failed"
                    }

                    field_errors: {
                        type: "array"
                        items: {
                            type: "object"
                            properties: {
                                field: {
                                    type: "string"
                                    description: "Field name"
                                }

                                message: {
                                    type: "string"
                                    description: "Field-specific error message"
                                }

                                code: {
                                    type: "string"
                                    description: "Validation error code"
                                }
                            }
                        }
                    }
                }
            }

            Pagination: {
                type: "object"
                properties: {
                    page: {
                        type: "integer"
                        description: "Current page"
                    }

                    size: {
                        type: "integer"
                        description: "Page size"
                    }

                    total_elements: {
                        type: "integer"
                        description: "Total number of elements"
                    }

                    total_pages: {
                        type: "integer"
                        description: "Total number of pages"
                    }

                    has_next: {
                        type: "boolean"
                        description: "Has next page"
                    }

                    has_previous: {
                        type: "boolean"
                        description: "Has previous page"
                    }
                }
            }

            Meta: {
                type: "object"
                properties: {
                    version: {
                        type: "string"
                        description: "API version"
                    }

                    timestamp: {
                        type: "string"
                        format: "date-time"
                        description: "Response timestamp"
                    }

                    request_id: {
                        type: "string"
                        description: "Request ID"
                    }
                }
            }
        }
    }
}
```

## API Monitoring

### API Metrics

```hyp
// API Monitoring
api_monitoring {
    // Metrics Collection
    metrics: {
        // Request Metrics
        requests: {
            total_requests: true
            requests_per_endpoint: true
            requests_per_method: true
            requests_per_status_code: true
            requests_per_user: true
            requests_per_ip: true
        }

        // Performance Metrics
        performance: {
            response_time: {
                p50: true
                p95: true
                p99: true
                p999: true
            }

            throughput: {
                requests_per_second: true
                bytes_per_second: true
            }

            error_rate: true
            availability: true
        }

        // Business Metrics
        business: {
            active_users: true
            api_usage_by_feature: true
            popular_endpoints: true
            user_satisfaction: true
        }
    }

    // Alerting
    alerting: {
        // Performance Alerts
        performance: {
            high_response_time: {
                threshold: 5000  // 5 seconds
                alert_level: "warning"
                window_size: 300  // 5 minutes
            }

            high_error_rate: {
                threshold: 0.05  // 5%
                alert_level: "critical"
                window_size: 300
            }

            low_availability: {
                threshold: 0.99  // 99%
                alert_level: "critical"
                window_size: 600  // 10 minutes
            }
        }

        // Security Alerts
        security: {
            high_failed_auth: {
                threshold: 10
                alert_level: "warning"
                window_size: 300
            }

            suspicious_activity: {
                threshold: "ai_detection"
                alert_level: "critical"
            }
        }
    }

    // Logging
    logging: {
        // Request Logging
        request_logging: {
            enabled: true
            log_level: "info"

            // Fields to Log
            fields: [
                "timestamp",
                "method",
                "path",
                "status_code",
                "response_time",
                "user_id",
                "ip_address",
                "user_agent",
                "request_id"
            ]

            // Mask Sensitive Data
            sensitive_fields: [
                "password",
                "api_key",
                "token",
                "authorization"
            ]
        }

        // Error Logging
        error_logging: {
            enabled: true
            log_level: "error"

            // Error Details
            include_stack_trace: true
            include_request_context: true
            include_user_context: true
        }
    }
}
```

## Best Practices

### API Best Practices

1. **API Design**

   - Follow RESTful principles
   - Use consistent naming conventions
   - Implement versioning

2. **Security**

   - OAuth2/JWT for authentication
   - Implement rate limiting
   - Perform input validation

3. **Performance**

   - Implement caching strategies
   - Pagination for large datasets
   - Enable compression

4. **Monitoring**

   - Collect comprehensive metrics
   - Proactive alerting systems
   - Implement request tracing

5. **Documentation**
   - OpenAPI specifications
   - Provide code examples
   - Maintain changelog

### API Checklist

- [ ] API endpoints defined
- [ ] Authentication implemented
- [ ] Authorization configured
- [ ] Rate limiting enabled
- [ ] OpenAPI documentation created
- [ ] Monitoring configured
- [ ] Error handling implemented
- [ ] Versioning configured
- [ ] Security tests performed
- [ ] Performance tests performed

These API management features ensure that HypnoScript provides secure, scalable, and well-documented APIs in runtime environments.

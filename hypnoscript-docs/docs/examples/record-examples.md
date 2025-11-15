---
title: Record (Tranceify) Examples
---

# Record (Tranceify) Examples

This page demonstrates practical examples of using the `tranceify` keyword to create custom record types in HypnoScript.

## Patient Management System

A complete example of managing patient records in a hypnotherapy practice:

```hypnoscript
Focus {
    // Define patient record type
    tranceify Patient {
        id: number;
        name: string;
        age: number;
        contactNumber: string;
        isActive: boolean;
    }

    // Define session record type
    tranceify TherapySession {
        sessionId: number;
        patientId: number;
        date: string;
        duration: number;
        tranceDepth: number;
        notes: string;
        successful: boolean;
    }

    // Create patient records
    induce patient1 = Patient {
        id: 1001,
        name: "Alice Johnson",
        age: 32,
        contactNumber: "555-0101",
        isActive: true
    };

    induce patient2 = Patient {
        id: 1002,
        name: "Bob Smith",
        age: 45,
        contactNumber: "555-0102",
        isActive: true
    };

    // Create session records
    induce session1 = TherapySession {
        sessionId: 5001,
        patientId: 1001,
        date: "2024-01-15",
        duration: 60,
        tranceDepth: 8.5,
        notes: "Deep relaxation achieved. Patient very responsive.",
        successful: true
    };

    induce session2 = TherapySession {
        sessionId: 5002,
        patientId: 1002,
        date: "2024-01-16",
        duration: 45,
        tranceDepth: 7.0,
        notes: "Good progress, some initial resistance.",
        successful: true
    };

    // Display patient information
    observe "Patient ID: " + patient1.id;
    observe "Name: " + patient1.name;
    observe "Age: " + patient1.age;
    observe "Contact: " + patient1.contactNumber;

    // Display session summary
    observe "\nSession Summary:";
    observe "Session ID: " + session1.sessionId;
    observe "Duration: " + session1.duration + " minutes";
    observe "Trance Depth: " + session1.tranceDepth + "/10";
    observe "Success: " + session1.successful;
    observe "Notes: " + session1.notes;
}
```

## E-Commerce Product Catalog

Managing products with nested records:

```hypnoscript
Focus {
    // Define dimension record
    tranceify Dimensions {
        width: number;
        height: number;
        depth: number;
    }

    // Define pricing record
    tranceify Pricing {
        basePrice: number;
        discount: number;
        finalPrice: number;
    }

    // Define product record with nested types
    tranceify Product {
        sku: string;
        name: string;
        description: string;
        dimensions: Dimensions;
        pricing: Pricing;
        inStock: boolean;
        quantity: number;
    }

    // Create a product
    induce laptop = Product {
        sku: "TECH-001",
        name: "HypnoBook Pro",
        description: "Premium laptop with mesmerizing display",
        dimensions: Dimensions {
            width: 35.5,
            height: 2.5,
            depth: 24.0
        },
        pricing: Pricing {
            basePrice: 1299.99,
            discount: 15.0,
            finalPrice: 1104.99
        },
        inStock: true,
        quantity: 42
    };

    // Display product information
    observe "Product: " + laptop.name;
    observe "SKU: " + laptop.sku;
    observe "Description: " + laptop.description;
    observe "\nDimensions (cm):";
    observe "  Width: " + laptop.dimensions.width;
    observe "  Height: " + laptop.dimensions.height;
    observe "  Depth: " + laptop.dimensions.depth;
    observe "\nPricing:";
    observe "  Base Price: $" + laptop.pricing.basePrice;
    observe "  Discount: " + laptop.pricing.discount + "%";
    observe "  Final Price: $" + laptop.pricing.finalPrice;
    observe "\nAvailability:";
    observe "  In Stock: " + laptop.inStock;
    observe "  Quantity: " + laptop.quantity;
}
```

## Geographic Coordinates

Working with location data:

```hypnoscript
Focus {
    // Define coordinate record
    tranceify Coordinate {
        latitude: number;
        longitude: number;
        altitude: number;
    }

    // Define location record
    tranceify Location {
        name: string;
        address: string;
        coordinates: Coordinate;
        category: string;
    }

    // Create locations
    induce clinic = Location {
        name: "Peaceful Mind Hypnotherapy Clinic",
        address: "123 Serenity Lane, Tranquil City",
        coordinates: Coordinate {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: 52.0
        },
        category: "Medical"
    };

    induce retreat = Location {
        name: "Mountain Trance Retreat",
        address: "456 Summit Road, Peak Valley",
        coordinates: Coordinate {
            latitude: 39.7392,
            longitude: -104.9903,
            altitude: 1655.0
        },
        category: "Wellness"
    };

    // Display location details
    observe clinic.name;
    observe "Address: " + clinic.address;
    observe "Coordinates: " + clinic.coordinates.latitude + ", " + clinic.coordinates.longitude;
    observe "Altitude: " + clinic.coordinates.altitude + "m";
    observe "Category: " + clinic.category;
}
```

## Event Management

Tracking events and attendees:

```hypnoscript
Focus {
    // Define attendee record
    tranceify Attendee {
        id: number;
        name: string;
        email: string;
        ticketType: string;
        checkedIn: boolean;
    }

    // Define event record
    tranceify Event {
        eventId: number;
        title: string;
        date: string;
        venue: string;
        capacity: number;
        ticketsSold: number;
    }

    // Create event
    induce workshop = Event {
        eventId: 2024,
        title: "Introduction to Self-Hypnosis",
        date: "2024-02-20",
        venue: "Mindfulness Center",
        capacity: 50,
        ticketsSold: 42
    };

    // Create attendees
    induce att1 = Attendee {
        id: 1,
        name: "Emma Watson",
        email: "emma@example.com",
        ticketType: "VIP",
        checkedIn: false
    };

    induce att2 = Attendee {
        id: 2,
        name: "John Doe",
        email: "john@example.com",
        ticketType: "Standard",
        checkedIn: true
    };

    // Store attendees in array
    induce attendees: array = [att1, att2];

    // Display event information
    observe "Event: " + workshop.title;
    observe "Date: " + workshop.date;
    observe "Venue: " + workshop.venue;
    observe "Capacity: " + workshop.capacity;
    observe "Tickets Sold: " + workshop.ticketsSold;
    observe "Available: " + (workshop.capacity - workshop.ticketsSold);

    // Display attendee information
    observe "\nAttendees:";
    observe "Total: " + Length(attendees);
    observe "\n1. " + attendees[0].name;
    observe "   Email: " + attendees[0].email;
    observe "   Ticket: " + attendees[0].ticketType;
    observe "   Checked In: " + attendees[0].checkedIn;
}
```

## Financial Transactions

Managing financial records:

```hypnoscript
Focus {
    // Define transaction record
    tranceify Transaction {
        id: string;
        date: string;
        amount: number;
        currency: string;
        category: string;
        description: string;
        completed: boolean;
    }

    // Define account record
    tranceify Account {
        accountNumber: string;
        accountHolder: string;
        balance: number;
        currency: string;
        active: boolean;
    }

    // Create account
    induce account = Account {
        accountNumber: "ACC-12345",
        accountHolder: "Dr. Sarah Chen",
        balance: 15750.50,
        currency: "USD",
        active: true
    };

    // Create transactions
    induce tx1 = Transaction {
        id: "TXN-001",
        date: "2024-01-15",
        amount: 250.00,
        currency: "USD",
        category: "Income",
        description: "Patient consultation fee",
        completed: true
    };

    induce tx2 = Transaction {
        id: "TXN-002",
        date: "2024-01-16",
        amount: 85.50,
        currency: "USD",
        category: "Expense",
        description: "Office supplies",
        completed: true
    };

    // Calculate net change
    induce netChange = tx1.amount - tx2.amount;

    // Display account summary
    observe "Account Information:";
    observe "Account #: " + account.accountNumber;
    observe "Holder: " + account.accountHolder;
    observe "Balance: " + account.currency + " " + account.balance;
    observe "Status: " + (account.active ? "Active" : "Inactive");

    observe "\nRecent Transactions:";
    observe "Transaction 1: " + tx1.description;
    observe "  Amount: " + tx1.currency + " " + tx1.amount;
    observe "  Date: " + tx1.date;

    observe "\nTransaction 2: " + tx2.description;
    observe "  Amount: " + tx2.currency + " " + tx2.amount;
    observe "  Date: " + tx2.date;

    observe "\nNet Change: " + account.currency + " " + netChange;
}
```

## Configuration Management

Using records for application configuration:

```hypnoscript
Focus {
    // Define database config
    tranceify DatabaseConfig {
        host: string;
        port: number;
        database: string;
        username: string;
        encrypted: boolean;
    }

    // Define logging config
    tranceify LoggingConfig {
        level: string;
        outputPath: string;
        maxFileSize: number;
        rotationEnabled: boolean;
    }

    // Define app config
    tranceify AppConfig {
        appName: string;
        version: string;
        environment: string;
        database: DatabaseConfig;
        logging: LoggingConfig;
        debugMode: boolean;
    }

    // Create configuration
    induce config = AppConfig {
        appName: "HypnoScript Runtime",
        version: "1.0.0",
        environment: "production",
        database: DatabaseConfig {
            host: "localhost",
            port: 5432,
            database: "hypnoscript_db",
            username: "admin",
            encrypted: true
        },
        logging: LoggingConfig {
            level: "INFO",
            outputPath: "/var/log/hypnoscript.log",
            maxFileSize: 10485760,
            rotationEnabled: true
        },
        debugMode: false
    };

    // Display configuration
    observe "Application: " + config.appName + " v" + config.version;
    observe "Environment: " + config.environment;
    observe "Debug Mode: " + config.debugMode;

    observe "\nDatabase Configuration:";
    observe "  Host: " + config.database.host;
    observe "  Port: " + config.database.port;
    observe "  Database: " + config.database.database;
    observe "  Encryption: " + config.database.encrypted;

    observe "\nLogging Configuration:";
    observe "  Level: " + config.logging.level;
    observe "  Output: " + config.logging.outputPath;
    observe "  Max Size: " + config.logging.maxFileSize + " bytes";
    observe "  Rotation: " + config.logging.rotationEnabled;
}
```

## Best Practices Demonstrated

1. **Descriptive Naming**: Record types use clear, domain-specific names
2. **Composition**: Complex data structures built from simpler records
3. **Type Safety**: All fields explicitly typed for validation
4. **Organization**: Related data grouped logically
5. **Calculations**: Record fields used in expressions and computations
6. **Arrays**: Collections of records managed efficiently

## See Also

- [Tranceify Language Reference](/language-reference/tranceify.md)
- [Type System](/language-reference/types.md)
- [Arrays](/language-reference/arrays.md)

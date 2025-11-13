---
title: Hashing & Encoding Functions
---

# Hashing & Encoding Functions

HypnoScript bietet umfangreiche Funktionen für Hashing, Verschlüsselung und Encoding von Daten.

## Übersicht

Hashing- und Encoding-Funktionen ermöglichen es Ihnen, Daten sicher zu verarbeiten, zu übertragen und zu speichern. Diese Funktionen sind besonders wichtig für Sicherheitsanwendungen und Datenintegrität.

## Hashing-Funktionen

### MD5

Erstellt einen MD5-Hash einer Zeichenkette.

```hyp
induce hash = MD5("Hello World");
observe "MD5 Hash: " + hash;
// Ausgabe: 5eb63bbbe01eeed093cb22bb8f5acdc3
```

**Parameter:**

- `input`: Die zu hashende Zeichenkette

**Rückgabewert:** MD5-Hash als Hexadezimal-String

### SHA1

Erstellt einen SHA1-Hash einer Zeichenkette.

```hyp
induce hash = SHA1("Hello World");
observe "SHA1 Hash: " + hash;
// Ausgabe: 2aae6c35c94fcfb415dbe95f408b9ce91ee846ed
```

**Parameter:**

- `input`: Die zu hashende Zeichenkette

**Rückgabewert:** SHA1-Hash als Hexadezimal-String

### SHA256

Erstellt einen SHA256-Hash einer Zeichenkette.

```hyp
induce hash = SHA256("Hello World");
observe "SHA256 Hash: " + hash;
// Ausgabe: a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e
```

**Parameter:**

- `input`: Die zu hashende Zeichenkette

**Rückgabewert:** SHA256-Hash als Hexadezimal-String

### SHA512

Erstellt einen SHA512-Hash einer Zeichenkette.

```hyp
induce hash = SHA512("Hello World");
observe "SHA512 Hash: " + hash;
// Ausgabe: 2c74fd17edafd80e8447b0d46741ee243b7eb74dd2149a0ab1b9246fb30382f27e853d8585719e0e67cbda0daa8f51671064615d645ae27acb15bfb1447f459b
```

**Parameter:**

- `input`: Die zu hashende Zeichenkette

**Rückgabewert:** SHA512-Hash als Hexadezimal-String

### HMAC

Erstellt einen HMAC-Hash mit einem geheimen Schlüssel.

```hyp
induce secret = "my-secret-key";
induce message = "Hello World";
induce hmac = HMAC(message, secret, "SHA256");
observe "HMAC: " + hmac;
```

**Parameter:**

- `message`: Die zu hashende Nachricht
- `key`: Der geheime Schlüssel
- `algorithm`: Der Hash-Algorithmus (MD5, SHA1, SHA256, SHA512)

**Rückgabewert:** HMAC-Hash als Hexadezimal-String

## Encoding-Funktionen

### Base64Encode

Kodiert eine Zeichenkette in Base64.

```hyp
induce original = "Hello World";
induce encoded = Base64Encode(original);
observe "Base64 encoded: " + encoded;
// Ausgabe: SGVsbG8gV29ybGQ=
```

**Parameter:**

- `input`: Die zu kodierende Zeichenkette

**Rückgabewert:** Base64-kodierte Zeichenkette

### Base64Decode

Dekodiert eine Base64-kodierte Zeichenkette.

```hyp
induce encoded = "SGVsbG8gV29ybGQ=";
induce decoded = Base64Decode(encoded);
observe "Base64 decoded: " + decoded;
// Ausgabe: Hello World
```

**Parameter:**

- `input`: Die Base64-kodierte Zeichenkette

**Rückgabewert:** Dekodierte Zeichenkette

### URLEncode

Kodiert eine Zeichenkette für URLs.

```hyp
induce original = "Hello World!";
induce encoded = URLEncode(original);
observe "URL encoded: " + encoded;
// Ausgabe: Hello+World%21
```

**Parameter:**

- `input`: Die zu kodierende Zeichenkette

**Rückgabewert:** URL-kodierte Zeichenkette

### URLDecode

Dekodiert eine URL-kodierte Zeichenkette.

```hyp
induce encoded = "Hello+World%21";
induce decoded = URLDecode(encoded);
observe "URL decoded: " + decoded;
// Ausgabe: Hello World!
```

**Parameter:**

- `input`: Die URL-kodierte Zeichenkette

**Rückgabewert:** Dekodierte Zeichenkette

### HTMLEncode

Kodiert eine Zeichenkette für HTML.

```hyp
induce original = "<script>alert('Hello')</script>";
induce encoded = HTMLEncode(original);
observe "HTML encoded: " + encoded;
// Ausgabe: &lt;script&gt;alert(&#39;Hello&#39;)&lt;/script&gt;
```

**Parameter:**

- `input`: Die zu kodierende Zeichenkette

**Rückgabewert:** HTML-kodierte Zeichenkette

### HTMLDecode

Dekodiert eine HTML-kodierte Zeichenkette.

```hyp
induce encoded = "&lt;script&gt;alert(&#39;Hello&#39;)&lt;/script&gt;";
induce decoded = HTMLDecode(encoded);
observe "HTML decoded: " + decoded;
// Ausgabe: <script>alert('Hello')</script>
```

**Parameter:**

- `input`: Die HTML-kodierte Zeichenkette

**Rückgabewert:** Dekodierte Zeichenkette

## Verschlüsselungs-Funktionen

### AESEncrypt

Verschlüsselt eine Zeichenkette mit AES.

```hyp
induce plaintext = "Secret message";
induce key = "my-secret-key-32-chars-long!!";
induce encrypted = AESEncrypt(plaintext, key);
observe "Encrypted: " + encrypted;
```

**Parameter:**

- `plaintext`: Der zu verschlüsselnde Text
- `key`: Der Verschlüsselungsschlüssel (32 Zeichen für AES-256)

**Rückgabewert:** Verschlüsselter Text als Base64-String

### AESDecrypt

Entschlüsselt einen AES-verschlüsselten Text.

```hyp
induce encrypted = "encrypted-base64-string";
induce key = "my-secret-key-32-chars-long!!";
induce decrypted = AESDecrypt(encrypted, key);
observe "Decrypted: " + decrypted;
```

**Parameter:**

- `encrypted`: Der verschlüsselte Text (Base64)
- `key`: Der Verschlüsselungsschlüssel

**Rückgabewert:** Entschlüsselter Text

### GenerateRandomKey

Generiert einen zufälligen Schlüssel für Verschlüsselung.

```hyp
induce key = GenerateRandomKey(32);
observe "Random key: " + key;
```

**Parameter:**

- `length`: Länge des Schlüssels in Bytes

**Rückgabewert:** Zufälliger Schlüssel als Hexadezimal-String

## Erweiterte Hashing-Funktionen

### PBKDF2

Erstellt einen PBKDF2-Hash für Passwort-Speicherung.

```hyp
induce password = "my-password";
induce salt = GenerateRandomKey(16);
induce hash = PBKDF2(password, salt, 10000, 32);
observe "PBKDF2 hash: " + hash;
```

**Parameter:**

- `password`: Das Passwort
- `salt`: Der Salt-Wert
- `iterations`: Anzahl der Iterationen
- `keyLength`: Länge des generierten Schlüssels

**Rückgabewert:** PBKDF2-Hash als Hexadezimal-String

### BCrypt

Erstellt einen BCrypt-Hash für Passwort-Speicherung.

```hyp
induce password = "my-password";
induce hash = BCrypt(password, 12);
observe "BCrypt hash: " + hash;
```

**Parameter:**

- `password`: Das Passwort
- `workFactor`: Arbeitsfaktor (10-12 empfohlen)

**Rückgabewert:** BCrypt-Hash

### VerifyBCrypt

Überprüft ein Passwort gegen einen BCrypt-Hash.

```hyp
induce password = "my-password";
induce hash = BCrypt(password, 12);
induce isValid = VerifyBCrypt(password, hash);
observe "Password valid: " + isValid;
```

**Parameter:**

- `password`: Das zu überprüfende Passwort
- `hash`: Der BCrypt-Hash

**Rückgabewert:** `true` wenn das Passwort korrekt ist, sonst `false`

## Utility-Funktionen

### GenerateSalt

Generiert einen zufälligen Salt-Wert.

```hyp
induce salt = GenerateSalt(16);
observe "Salt: " + salt;
```

**Parameter:**

- `length`: Länge des Salt-Werts in Bytes

**Rückgabewert:** Salt als Hexadezimal-String

### HashFile

Erstellt einen Hash einer Datei.

```hyp
induce filePath = "document.txt";
induce hash = HashFile(filePath, "SHA256");
observe "File hash: " + hash;
```

**Parameter:**

- `filePath`: Pfad zur Datei
- `algorithm`: Hash-Algorithmus (MD5, SHA1, SHA256, SHA512)

**Rückgabewert:** Hash der Datei als Hexadezimal-String

### VerifyHash

Überprüft, ob ein Hash mit einem Wert übereinstimmt.

```hyp
induce input = "Hello World";
induce expectedHash = "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e";
induce actualHash = SHA256(input);
induce isValid = VerifyHash(actualHash, expectedHash);
observe "Hash valid: " + isValid;
```

**Parameter:**

- `actualHash`: Der tatsächliche Hash
- `expectedHash`: Der erwartete Hash

**Rückgabewert:** `true` wenn die Hashes übereinstimmen, sonst `false`

## Best Practices

### Sichere Passwort-Speicherung

```hyp
Focus {
    entrance {
        // Passwort vom Benutzer erhalten
        induce password = InputProvider("Enter password: ");

        // Salt generieren
        induce salt = GenerateSalt(16);

        // Passwort hashen
        induce hash = PBKDF2(password, salt, 10000, 32);

        // Hash und Salt speichern (ohne Passwort)
        induce userData = {
            username: "john_doe",
            passwordHash: hash,
            salt: salt,
            createdAt: GetCurrentDateTime()
        };

        // In Datenbank speichern
        SaveUserData(userData);

        observe "Benutzer sicher gespeichert!";
    }
} Relax;
```

### Datei-Integrität prüfen

```hyp
Focus {
    entrance {
        induce filePath = "important-document.pdf";

        // Hash der Original-Datei
        induce originalHash = HashFile(filePath, "SHA256");
        observe "Original hash: " + originalHash;

        // Datei übertragen oder verarbeiten
        // ...

        // Hash nach Übertragung prüfen
        induce currentHash = HashFile(filePath, "SHA256");
        induce isIntegrityValid = VerifyHash(currentHash, originalHash);

        if (isIntegrityValid) {
            observe "Datei-Integrität bestätigt!";
        } else {
            observe "WARNUNG: Datei wurde verändert!";
        }
    }
} Relax;
```

### Sichere Datenübertragung

```hyp
Focus {
    entrance {
        induce secretMessage = "Vertrauliche Daten";
        induce key = GenerateRandomKey(32);

        // Nachricht verschlüsseln
        induce encrypted = AESEncrypt(secretMessage, key);
        observe "Verschlüsselt: " + encrypted;

        // Nachricht übertragen (simuliert)
        induce transmittedData = encrypted;

        // Nachricht entschlüsseln
        induce decrypted = AESDecrypt(transmittedData, key);
        observe "Entschlüsselt: " + decrypted;

        if (decrypted == secretMessage) {
            observe "Sichere Übertragung erfolgreich!";
        }
    }
} Relax;
```

### API-Sicherheit

```hyp
Focus {
    entrance {
        induce apiKey = "my-api-key";
        induce timestamp = GetCurrentTime();
        induce data = "request-data";

        // HMAC für API-Authentifizierung erstellen
        induce message = timestamp + ":" + data;
        induce signature = HMAC(message, apiKey, "SHA256");

        // API-Request mit Signatur
        induce request = {
            timestamp: timestamp,
            data: data,
            signature: signature
        };

        observe "API-Request: " + ToJson(request);

        // Auf der Server-Seite würde die Signatur überprüft werden
        induce isValidSignature = VerifyHMAC(message, signature, apiKey, "SHA256");
        observe "Signatur gültig: " + isValidSignature;
    }
} Relax;
```

## Sicherheitshinweise

### Wichtige Sicherheitsaspekte

1. **Salt-Werte**: Verwenden Sie immer zufällige Salt-Werte für Passwort-Hashing
2. **Iterationen**: Verwenden Sie mindestens 10.000 Iterationen für PBKDF2
3. **Schlüssellänge**: Verwenden Sie mindestens 256-Bit-Schlüssel für AES
4. **Algorithmen**: Vermeiden Sie MD5 und SHA1 für Sicherheitsanwendungen
5. **Schlüssel-Management**: Speichern Sie Schlüssel sicher und niemals im Code

### Deprecated-Funktionen

```hyp
// VERMEIDEN: MD5 für Sicherheitsanwendungen
induce weakHash = MD5("password");

// VERWENDEN: Starke Hash-Funktionen
induce strongHash = SHA256("password");
induce secureHash = PBKDF2("password", salt, 10000, 32);
```

## Fehlerbehandlung

Hashing- und Encoding-Funktionen können bei ungültigen Eingaben Fehler werfen:

```hyp
Focus {
    entrance {
        try {
            induce hash = SHA256("valid-input");
            observe "Hash erfolgreich: " + hash;
        } catch (error) {
            observe "Fehler beim Hashing: " + error;
        }

        try {
            induce decoded = Base64Decode("invalid-base64");
            observe "Dekodierung erfolgreich: " + decoded;
        } catch (error) {
            observe "Fehler beim Dekodieren: " + error;
        }
    }
} Relax;
```

## Nächste Schritte

- [Validation Functions](./validation-functions) - Validierungsfunktionen
- [Network Functions](./network-functions) - Netzwerk-Funktionen
- [Security Best Practices](../enterprise/security) - Sicherheitsrichtlinien

---

**Hashing & Encoding gemeistert? Dann lerne [Validation Functions](./validation-functions) kennen!** ✅

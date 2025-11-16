---
title: Hashing & Encoding Functions
---

# Hashing & Encoding Functions

HypnoScript bietet umfangreiche Functionen für Hashing, Verschlüsselung und Encoding von Daten.

## Overview

Hashing- und Encoding-Functionen ermöglichen es Ihnen, Daten sicher zu verarbeiten, zu übertragen und zu speichern. Diese Functionen sind besonders wichtig für Sicherheitsanwendungen und Datenintegrität.

## Hashing-Functionen

### MD5

Creates einen MD5-Hash einer Zeichenkette.

```hyp
induce hash = MD5("Hello World");
observe "MD5 Hash: " + hash;
// Ausgabe: 5eb63bbbe01eeed093cb22bb8f5acdc3
```

**Parameters:**

- `input`: Die zu hashende Zeichenkette

**Return value:** MD5-Hash als Hexadezimal-String

### SHA1

Creates einen SHA1-Hash einer Zeichenkette.

```hyp
induce hash = SHA1("Hello World");
observe "SHA1 Hash: " + hash;
// Ausgabe: 2aae6c35c94fcfb415dbe95f408b9ce91ee846ed
```

**Parameters:**

- `input`: Die zu hashende Zeichenkette

**Return value:** SHA1-Hash als Hexadezimal-String

### SHA256

Creates einen SHA256-Hash einer Zeichenkette.

```hyp
induce hash = SHA256("Hello World");
observe "SHA256 Hash: " + hash;
// Ausgabe: a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e
```

**Parameters:**

- `input`: Die zu hashende Zeichenkette

**Return value:** SHA256-Hash als Hexadezimal-String

### SHA512

Creates einen SHA512-Hash einer Zeichenkette.

```hyp
induce hash = SHA512("Hello World");
observe "SHA512 Hash: " + hash;
// Ausgabe: 2c74fd17edafd80e8447b0d46741ee243b7eb74dd2149a0ab1b9246fb30382f27e853d8585719e0e67cbda0daa8f51671064615d645ae27acb15bfb1447f459b
```

**Parameters:**

- `input`: Die zu hashende Zeichenkette

**Return value:** SHA512-Hash als Hexadezimal-String

### HMAC

Creates einen HMAC-Hash mit einem geheimen Schlüssel.

```hyp
induce secret = "my-secret-key";
induce message = "Hello World";
induce hmac = HMAC(message, secret, "SHA256");
observe "HMAC: " + hmac;
```

**Parameters:**

- `message`: Die zu hashende Nachricht
- `key`: Der geheime Schlüssel
- `algorithm`: Der Hash-Algorithmus (MD5, SHA1, SHA256, SHA512)

**Return value:** HMAC-Hash als Hexadezimal-String

## Encoding-Functionen

### Base64Encode

Kodiert eine Zeichenkette in Base64.

```hyp
induce original = "Hello World";
induce encoded = Base64Encode(original);
observe "Base64 encoded: " + encoded;
// Ausgabe: SGVsbG8gV29ybGQ=
```

**Parameters:**

- `input`: Die zu kodierende Zeichenkette

**Return value:** Base64-kodierte Zeichenkette

### Base64Decode

Dekodiert eine Base64-kodierte Zeichenkette.

```hyp
induce encoded = "SGVsbG8gV29ybGQ=";
induce decoded = Base64Decode(encoded);
observe "Base64 decoded: " + decoded;
// Ausgabe: Hello World
```

**Parameters:**

- `input`: Die Base64-kodierte Zeichenkette

**Return value:** Dekodierte Zeichenkette

### URLEncode

Kodiert eine Zeichenkette für URLs.

```hyp
induce original = "Hello World!";
induce encoded = URLEncode(original);
observe "URL encoded: " + encoded;
// Ausgabe: Hello+World%21
```

**Parameters:**

- `input`: Die zu kodierende Zeichenkette

**Return value:** URL-kodierte Zeichenkette

### URLDecode

Dekodiert eine URL-kodierte Zeichenkette.

```hyp
induce encoded = "Hello+World%21";
induce decoded = URLDecode(encoded);
observe "URL decoded: " + decoded;
// Ausgabe: Hello World!
```

**Parameters:**

- `input`: Die URL-kodierte Zeichenkette

**Return value:** Dekodierte Zeichenkette

### HTMLEncode

Kodiert eine Zeichenkette für HTML.

```hyp
induce original = "<script>alert('Hello')</script>";
induce encoded = HTMLEncode(original);
observe "HTML encoded: " + encoded;
// Ausgabe: &lt;script&gt;alert(&#39;Hello&#39;)&lt;/script&gt;
```

**Parameters:**

- `input`: Die zu kodierende Zeichenkette

**Return value:** HTML-kodierte Zeichenkette

### HTMLDecode

Dekodiert eine HTML-kodierte Zeichenkette.

```hyp
induce encoded = "&lt;script&gt;alert(&#39;Hello&#39;)&lt;/script&gt;";
induce decoded = HTMLDecode(encoded);
observe "HTML decoded: " + decoded;
// Ausgabe: <script>alert('Hello')</script>
```

**Parameters:**

- `input`: Die HTML-kodierte Zeichenkette

**Return value:** Dekodierte Zeichenkette

## Verschlüsselungs-Functionen

### AESEncrypt

Verschlüsselt eine Zeichenkette mit AES.

```hyp
induce plaintext = "Secret message";
induce key = "my-secret-key-32-chars-long!!";
induce encrypted = AESEncrypt(plaintext, key);
observe "Encrypted: " + encrypted;
```

**Parameters:**

- `plaintext`: Der zu verschlüsselnde Text
- `key`: Der Verschlüsselungsschlüssel (32 Zeichen für AES-256)

**Return value:** Verschlüsselter Text als Base64-String

### AESDecrypt

Entschlüsselt einen AES-verschlüsselten Text.

```hyp
induce encrypted = "encrypted-base64-string";
induce key = "my-secret-key-32-chars-long!!";
induce decrypted = AESDecrypt(encrypted, key);
observe "Decrypted: " + decrypted;
```

**Parameters:**

- `encrypted`: Der verschlüsselte Text (Base64)
- `key`: Der Verschlüsselungsschlüssel

**Return value:** Entschlüsselter Text

### GenerateRandomKey

Generates einen zufälligen Schlüssel für Verschlüsselung.

```hyp
induce key = GenerateRandomKey(32);
observe "Random key: " + key;
```

**Parameters:**

- `length`: Length des Schlüssels in Bytes

**Return value:** Zufälliger Schlüssel als Hexadezimal-String

## Advanced Hashing-Functionen

### PBKDF2

Creates einen PBKDF2-Hash für Passwort-Speicherung.

```hyp
induce password = "my-password";
induce salt = GenerateRandomKey(16);
induce hash = PBKDF2(password, salt, 10000, 32);
observe "PBKDF2 hash: " + hash;
```

**Parameters:**

- `password`: Das Passwort
- `salt`: Der Salt-Wert
- `iterations`: Anzahl der Iterationen
- `keyLength`: Length des generierten Schlüssels

**Return value:** PBKDF2-Hash als Hexadezimal-String

### BCrypt

Creates einen BCrypt-Hash für Passwort-Speicherung.

```hyp
induce password = "my-password";
induce hash = BCrypt(password, 12);
observe "BCrypt hash: " + hash;
```

**Parameters:**

- `password`: Das Passwort
- `workFactor`: Arbeitsfaktor (10-12 empfohlen)

**Return value:** BCrypt-Hash

### VerifyBCrypt

Überprüft ein Passwort gegen einen BCrypt-Hash.

```hyp
induce password = "my-password";
induce hash = BCrypt(password, 12);
induce isValid = VerifyBCrypt(password, hash);
observe "Password valid: " + isValid;
```

**Parameters:**

- `password`: Das zu überprüfende Passwort
- `hash`: Der BCrypt-Hash

**Return value:** `true` wenn das Passwort korrekt ist, sonst `false`

## Utility-Functionen

### GenerateSalt

Generates einen zufälligen Salt-Wert.

```hyp
induce salt = GenerateSalt(16);
observe "Salt: " + salt;
```

**Parameters:**

- `length`: Length des Salt-Werts in Bytes

**Return value:** Salt als Hexadezimal-String

### HashFile

Creates einen Hash einer File.

```hyp
induce filePath = "document.txt";
induce hash = HashFile(filePath, "SHA256");
observe "File hash: " + hash;
```

**Parameters:**

- `filePath`: Pfad zur File
- `algorithm`: Hash-Algorithmus (MD5, SHA1, SHA256, SHA512)

**Return value:** Hash der File als Hexadezimal-String

### VerifyHash

Überprüft, ob ein Hash mit einem Wert übereinstimmt.

```hyp
induce input = "Hello World";
induce expectedHash = "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e";
induce actualHash = SHA256(input);
induce isValid = VerifyHash(actualHash, expectedHash);
observe "Hash valid: " + isValid;
```

**Parameters:**

- `actualHash`: Der tatsächliche Hash
- `expectedHash`: Der erwartete Hash

**Return value:** `true` wenn die Hashes übereinstimmen, sonst `false`

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

### File-Integrität prüfen

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

### Deprecated-Functionen

```hyp
// VERMEIDEN: MD5 für Sicherheitsanwendungen
induce weakHash = MD5("password");

// VERWENDEN: Starke Hash-Funktionen
induce strongHash = SHA256("password");
induce secureHash = PBKDF2("password", salt, 10000, 32);
```

## Fehlerbehandlung

Hashing- und Encoding-Functionen können bei ungültigen Inputn Fehler werfen:

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

## Next Steps

- [Validation Functions](./validation-functions) - Validierungsfunktionen
- [Network Functions](./network-functions) - Netzwerk-Functionen
- [Security Best Practices](../enterprise/security) - Sicherheitsrichtlinien

---

**Hashing & Encoding gemeistert? Dann lerne [Validation Functions](./validation-functions) kennen!** ✅

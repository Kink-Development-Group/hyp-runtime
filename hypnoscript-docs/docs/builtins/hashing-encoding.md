---
title: Hashing & Encoding Functions
---

# Hashing & Encoding Functions

HypnoScript provides comprehensive functions for hashing, encryption, and encoding of data.

## Overview

Hashing and encoding functions enable you to securely process, transfer, and store data. These functions are particularly important for security applications and data integrity.

## Hashing Functions

### MD5

Creates an MD5 hash of a string.

```hyp
induce hash = MD5("Hello World");
observe "MD5 Hash: " + hash;
// Output: 5eb63bbbe01eeed093cb22bb8f5acdc3
```

**Parameters:**

- `input`: The string to hash

**Return value:** MD5 hash as hexadecimal string

### SHA1

Creates a SHA1 hash of a string.

```hyp
induce hash = SHA1("Hello World");
observe "SHA1 Hash: " + hash;
// Output: 2aae6c35c94fcfb415dbe95f408b9ce91ee846ed
```

**Parameters:**

- `input`: The string to hash

**Return value:** SHA1 hash as hexadecimal string

### SHA256

Creates a SHA256 hash of a string.

```hyp
induce hash = SHA256("Hello World");
observe "SHA256 Hash: " + hash;
// Output: a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e
```

**Parameters:**

- `input`: The string to hash

**Return value:** SHA256 hash as hexadecimal string

### SHA512

Creates a SHA512 hash of a string.

```hyp
induce hash = SHA512("Hello World");
observe "SHA512 Hash: " + hash;
// Output: 2c74fd17edafd80e8447b0d46741ee243b7eb74dd2149a0ab1b9246fb30382f27e853d8585719e0e67cbda0daa8f51671064615d645ae27acb15bfb1447f459b
```

**Parameters:**

- `input`: The string to hash

**Return value:** SHA512 hash as hexadecimal string

### HMAC

Creates an HMAC hash with a secret key.

```hyp
induce secret = "my-secret-key";
induce message = "Hello World";
induce hmac = HMAC(message, secret, "SHA256");
observe "HMAC: " + hmac;
```

**Parameters:**

- `message`: The message to hash
- `key`: The secret key
- `algorithm`: The hash algorithm (MD5, SHA1, SHA256, SHA512)

**Return value:** HMAC hash as hexadecimal string

## Encoding Functions

### Base64Encode

Encodes a string in Base64.

```hyp
induce original = "Hello World";
induce encoded = Base64Encode(original);
observe "Base64 encoded: " + encoded;
// Output: SGVsbG8gV29ybGQ=
```

**Parameters:**

- `input`: The string to encode

**Return value:** Base64-encoded string

### Base64Decode

Decodes a Base64-encoded string.

```hyp
induce encoded = "SGVsbG8gV29ybGQ=";
induce decoded = Base64Decode(encoded);
observe "Base64 decoded: " + decoded;
// Output: Hello World
```

**Parameters:**

- `input`: The Base64-encoded string

**Return value:** Decoded string

### URLEncode

Encodes a string for URLs.

```hyp
induce original = "Hello World!";
induce encoded = URLEncode(original);
observe "URL encoded: " + encoded;
// Output: Hello+World%21
```

**Parameters:**

- `input`: The string to encode

**Return value:** URL-encoded string

### URLDecode

Decodes a URL-encoded string.

```hyp
induce encoded = "Hello+World%21";
induce decoded = URLDecode(encoded);
observe "URL decoded: " + decoded;
// Output: Hello World!
```

**Parameters:**

- `input`: The URL-encoded string

**Return value:** Decoded string

### HTMLEncode

Encodes a string for HTML.

```hyp
induce original = "<script>alert('Hello')</script>";
induce encoded = HTMLEncode(original);
observe "HTML encoded: " + encoded;
// Output: &lt;script&gt;alert(&#39;Hello&#39;)&lt;/script&gt;
```

**Parameters:**

- `input`: The string to encode

**Return value:** HTML-encoded string

### HTMLDecode

Decodes an HTML-encoded string.

```hyp
induce encoded = "&lt;script&gt;alert(&#39;Hello&#39;)&lt;/script&gt;";
induce decoded = HTMLDecode(encoded);
observe "HTML decoded: " + decoded;
// Output: <script>alert('Hello')</script>
```

**Parameters:**

- `input`: The HTML-encoded string

**Return value:** Decoded string

## Encryption Functions

### AESEncrypt

Encrypts a string with AES.

```hyp
induce plaintext = "Secret message";
induce key = "my-secret-key-32-chars-long!!";
induce encrypted = AESEncrypt(plaintext, key);
observe "Encrypted: " + encrypted;
```

**Parameters:**

- `plaintext`: The text to encrypt
- `key`: The encryption key (32 characters for AES-256)

**Return value:** Encrypted text as Base64 string

### AESDecrypt

Decrypts an AES-encrypted text.

```hyp
induce encrypted = "encrypted-base64-string";
induce key = "my-secret-key-32-chars-long!!";
induce decrypted = AESDecrypt(encrypted, key);
observe "Decrypted: " + decrypted;
```

**Parameters:**

- `encrypted`: The encrypted text (Base64)
- `key`: The encryption key

**Return value:** Decrypted text

### GenerateRandomKey

Generates a random key for encryption.

```hyp
induce key = GenerateRandomKey(32);
observe "Random key: " + key;
```

**Parameters:**

- `length`: Length of the key in bytes

**Return value:** Random key as hexadecimal string

## Advanced Hashing Functions

### PBKDF2

Creates a PBKDF2 hash for password storage.

```hyp
induce password = "my-password";
induce salt = GenerateRandomKey(16);
induce hash = PBKDF2(password, salt, 10000, 32);
observe "PBKDF2 hash: " + hash;
```

**Parameters:**

- `password`: The password
- `salt`: The salt value
- `iterations`: Number of iterations
- `keyLength`: Length of the generated key

**Return value:** PBKDF2 hash as hexadecimal string

### BCrypt

Creates a BCrypt hash for password storage.

```hyp
induce password = "my-password";
induce hash = BCrypt(password, 12);
observe "BCrypt hash: " + hash;
```

**Parameters:**

- `password`: The password
- `workFactor`: Work factor (10-12 recommended)

**Return value:** BCrypt hash

### VerifyBCrypt

Verifies a password against a BCrypt hash.

```hyp
induce password = "my-password";
induce hash = BCrypt(password, 12);
induce isValid = VerifyBCrypt(password, hash);
observe "Password valid: " + isValid;
```

**Parameters:**

- `password`: The password to verify
- `hash`: The BCrypt hash

**Return value:** `true` if the password is correct, otherwise `false`

## Utility Functions

### GenerateSalt

Generates a random salt value.

```hyp
induce salt = GenerateSalt(16);
observe "Salt: " + salt;
```

**Parameters:**

- `length`: Length of the salt value in bytes

**Return value:** Salt as hexadecimal string

### HashFile

Creates a hash of a file.

```hyp
induce filePath = "document.txt";
induce hash = HashFile(filePath, "SHA256");
observe "File hash: " + hash;
```

**Parameters:**

- `filePath`: Path to the file
- `algorithm`: Hash algorithm (MD5, SHA1, SHA256, SHA512)

**Return value:** Hash of the file as hexadecimal string

### VerifyHash

Verifies if a hash matches a value.

```hyp
induce input = "Hello World";
induce expectedHash = "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e";
induce actualHash = SHA256(input);
induce isValid = VerifyHash(actualHash, expectedHash);
observe "Hash valid: " + isValid;
```

**Parameters:**

- `actualHash`: The actual hash
- `expectedHash`: The expected hash

**Return value:** `true` if the hashes match, otherwise `false`

## Best Practices

### Secure Password Storage

```hyp
Focus {
    entrance {
        // Get password from user
        induce password = InputProvider("Enter password: ");

        // Generate salt
        induce salt = GenerateSalt(16);

        // Hash password
        induce hash = PBKDF2(password, salt, 10000, 32);

        // Store hash and salt (without password)
        induce userData = {
            username: "john_doe",
            passwordHash: hash,
            salt: salt,
            createdAt: GetCurrentDateTime()
        };

        // Store in database
        SaveUserData(userData);

        observe "User stored securely!";
    }
} Relax;
```

### Check File Integrity

```hyp
Focus {
    entrance {
        induce filePath = "important-document.pdf";

        // Hash of the original file
        induce originalHash = HashFile(filePath, "SHA256");
        observe "Original hash: " + originalHash;

        // Transfer or process file
        // ...

        // Check hash after transfer
        induce currentHash = HashFile(filePath, "SHA256");
        induce isIntegrityValid = VerifyHash(currentHash, originalHash);

        if (isIntegrityValid) {
            observe "File integrity verified!";
        } else {
            observe "WARNING: File was modified!";
        }
    }
} Relax;
```

### Secure Data Transfer

```hyp
Focus {
    entrance {
        induce secretMessage = "Confidential data";
        induce key = GenerateRandomKey(32);

        // Encrypt message
        induce encrypted = AESEncrypt(secretMessage, key);
        observe "Encrypted: " + encrypted;

        // Transfer message (simulated)
        induce transmittedData = encrypted;

        // Decrypt message
        induce decrypted = AESDecrypt(transmittedData, key);
        observe "Decrypted: " + decrypted;

        if (decrypted == secretMessage) {
            observe "Secure transfer successful!";
        }
    }
} Relax;
```

### API Security

```hyp
Focus {
    entrance {
        induce apiKey = "my-api-key";
        induce timestamp = GetCurrentTime();
        induce data = "request-data";

        // Create HMAC for API authentication
        induce message = timestamp + ":" + data;
        induce signature = HMAC(message, apiKey, "SHA256");

        // API request with signature
        induce request = {
            timestamp: timestamp,
            data: data,
            signature: signature
        };

        observe "API request: " + ToJson(request);

        // On the server side, the signature would be verified
        induce isValidSignature = VerifyHMAC(message, signature, apiKey, "SHA256");
        observe "Signature valid: " + isValidSignature;
    }
} Relax;
```

## Security Notes

### Important Security Considerations

1. **Salt values**: Always use random salt values for password hashing
2. **Iterations**: Use at least 10,000 iterations for PBKDF2
3. **Key length**: Use at least 256-bit keys for AES
4. **Algorithms**: Avoid MD5 and SHA1 for security applications
5. **Key management**: Store keys securely and never in code

### Deprecated Functions

```hyp
// AVOID: MD5 for security applications
induce weakHash = MD5("password");

// USE: Strong hash functions
induce strongHash = SHA256("password");
induce secureHash = PBKDF2("password", salt, 10000, 32);
```

## Error Handling

Hashing and encoding functions can throw errors with invalid input:

```hyp
Focus {
    entrance {
        try {
            induce hash = SHA256("valid-input");
            observe "Hash successful: " + hash;
        } catch (error) {
            observe "Error hashing: " + error;
        }

        try {
            induce decoded = Base64Decode("invalid-base64");
            observe "Decoding successful: " + decoded;
        } catch (error) {
            observe "Error decoding: " + error;
        }
    }
} Relax;
```

## Next Steps

- [Validation Functions](./validation-functions) - Validation functions
- [Network Functions](./network-functions) - Network functions
- [Security Best Practices](../enterprise/security) - Security guidelines

---

**Hashing & Encoding mastered? Then learn about [Validation Functions](./validation-functions)!** ✅

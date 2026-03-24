#![allow(dead_code)]
//! Apple CryptoKit — cryptographic operations from Rust.
//!
//! **Platform:** all Apple platforms.
//!
//! Wraps CommonCrypto (part of libSystem) — pure C, no Swift needed.
//! Provides hashing (SHA-256/384/512, SHA-1, MD5), HMAC, symmetric
//! encryption (AES-CBC/ECB), key derivation (PBKDF2), and secure random.
//!
//! # Quick start
//!
//! ```ignore
//! // Hash
//! let digest = cryptokit::sha256(b"hello world");
//!
//! // HMAC
//! let mac = cryptokit::hmac::sha256(b"secret-key", b"message");
//!
//! // AES-256-CBC encrypt
//! let key = cryptokit::random_bytes(32);
//! let iv = cryptokit::random_bytes(16);
//! let ct = cryptokit::aes::encrypt(&key, Some(&iv), b"plaintext").unwrap();
//! let pt = cryptokit::aes::decrypt(&key, Some(&iv), &ct).unwrap();
//!
//! // PBKDF2 key derivation
//! let derived = cryptokit::pbkdf2::derive("password", b"salt", 10_000, 32);
//! ```

//!
//! ## License
//!
//! GPL-3.0 — Copyright © 2025 [Eugene Hauptmann](https://github.com/eugenehp)

pub fn is_available() -> bool { true }

// ── Raw CommonCrypto FFI ────────────────────────────────────────────────────

type CCCryptorStatus = i32;
type CCRNGStatus = i32;

const CC_SUCCESS: i32 = 0;

// Digest lengths
const CC_MD5_DIGEST_LENGTH: usize = 16;
const CC_SHA1_DIGEST_LENGTH: usize = 20;
const CC_SHA224_DIGEST_LENGTH: usize = 28;
const CC_SHA256_DIGEST_LENGTH: usize = 32;
const CC_SHA384_DIGEST_LENGTH: usize = 48;
const CC_SHA512_DIGEST_LENGTH: usize = 64;

// CCOperation
const CC_ENCRYPT: u32 = 0;
const CC_DECRYPT: u32 = 1;

// CCAlgorithm
const CC_ALGORITHM_AES: u32 = 0;
const CC_ALGORITHM_DES: u32 = 1;
const CC_ALGORITHM_3DES: u32 = 2;
const CC_ALGORITHM_BLOWFISH: u32 = 6;

// CCOptions
const CC_OPTION_PKCS7: u32 = 0x0001;
const CC_OPTION_ECB: u32 = 0x0002;

// Key sizes
/// AES-128 key size.
pub const AES128_KEY_SIZE: usize = 16;
/// AES-192 key size.
pub const AES192_KEY_SIZE: usize = 24;
/// AES-256 key size.
pub const AES256_KEY_SIZE: usize = 32;
/// AES block size.
pub const AES_BLOCK_SIZE: usize = 16;

// CCHmacAlgorithm
const CC_HMAC_ALG_SHA1: u32 = 0;
const CC_HMAC_ALG_MD5: u32 = 1;
const CC_HMAC_ALG_SHA256: u32 = 2;
const CC_HMAC_ALG_SHA384: u32 = 3;
const CC_HMAC_ALG_SHA512: u32 = 4;
const CC_HMAC_ALG_SHA224: u32 = 5;

// CCPBKDFAlgorithm / CCPseudoRandomAlgorithm
const CC_PBKDF2: u32 = 2;
const CC_PRF_HMAC_SHA1: u32 = 1;
const CC_PRF_HMAC_SHA224: u32 = 2;
const CC_PRF_HMAC_SHA256: u32 = 3;
const CC_PRF_HMAC_SHA384: u32 = 4;
const CC_PRF_HMAC_SHA512: u32 = 5;

#[allow(non_snake_case)]
unsafe extern "C" {
    // Digests
    fn CC_MD5(data: *const u8, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA1(data: *const u8, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA224(data: *const u8, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA256(data: *const u8, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA384(data: *const u8, len: u32, md: *mut u8) -> *mut u8;
    fn CC_SHA512(data: *const u8, len: u32, md: *mut u8) -> *mut u8;

    // HMAC
    fn CCHmac(
        algorithm: u32, key: *const u8, key_len: usize,
        data: *const u8, data_len: usize, mac_out: *mut u8,
    );

    // Symmetric encryption
    fn CCCrypt(
        op: u32, alg: u32, options: u32,
        key: *const u8, key_len: usize,
        iv: *const u8,
        data_in: *const u8, data_in_len: usize,
        data_out: *mut u8, data_out_available: usize,
        data_out_moved: *mut usize,
    ) -> CCCryptorStatus;

    // Key derivation
    fn CCKeyDerivationPBKDF(
        algorithm: u32,
        password: *const u8, password_len: usize,
        salt: *const u8, salt_len: usize,
        prf: u32, rounds: u32,
        derived_key: *mut u8, derived_key_len: usize,
    ) -> i32;

    // Random
    fn CCRandomGenerateBytes(bytes: *mut u8, count: usize) -> CCRNGStatus;
}

// ── Hashing ─────────────────────────────────────────────────────────────────

/// MD5 hash (16 bytes). **Not secure** — use for checksums only.
pub fn md5(data: &[u8]) -> [u8; CC_MD5_DIGEST_LENGTH] {
    let mut out = [0u8; CC_MD5_DIGEST_LENGTH];
    unsafe { CC_MD5(data.as_ptr(), data.len() as u32, out.as_mut_ptr()); }
    out
}

/// SHA-1 hash (20 bytes). **Not secure** — use SHA-256+ for new code.
pub fn sha1(data: &[u8]) -> [u8; CC_SHA1_DIGEST_LENGTH] {
    let mut out = [0u8; CC_SHA1_DIGEST_LENGTH];
    unsafe { CC_SHA1(data.as_ptr(), data.len() as u32, out.as_mut_ptr()); }
    out
}

/// SHA-224 hash (28 bytes).
pub fn sha224(data: &[u8]) -> [u8; CC_SHA224_DIGEST_LENGTH] {
    let mut out = [0u8; CC_SHA224_DIGEST_LENGTH];
    unsafe { CC_SHA224(data.as_ptr(), data.len() as u32, out.as_mut_ptr()); }
    out
}

/// SHA-256 hash (32 bytes).
pub fn sha256(data: &[u8]) -> [u8; CC_SHA256_DIGEST_LENGTH] {
    let mut out = [0u8; CC_SHA256_DIGEST_LENGTH];
    unsafe { CC_SHA256(data.as_ptr(), data.len() as u32, out.as_mut_ptr()); }
    out
}

/// SHA-384 hash (48 bytes).
pub fn sha384(data: &[u8]) -> [u8; CC_SHA384_DIGEST_LENGTH] {
    let mut out = [0u8; CC_SHA384_DIGEST_LENGTH];
    unsafe { CC_SHA384(data.as_ptr(), data.len() as u32, out.as_mut_ptr()); }
    out
}

/// SHA-512 hash (64 bytes).
pub fn sha512(data: &[u8]) -> [u8; CC_SHA512_DIGEST_LENGTH] {
    let mut out = [0u8; CC_SHA512_DIGEST_LENGTH];
    unsafe { CC_SHA512(data.as_ptr(), data.len() as u32, out.as_mut_ptr()); }
    out
}

/// Format a digest as a lowercase hex string.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// ── HMAC ────────────────────────────────────────────────────────────────────

/// HMAC (keyed-hash message authentication code).
pub mod hmac {
    use super::*;

    fn compute(alg: u32, digest_len: usize, key: &[u8], data: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; digest_len];
        unsafe { CCHmac(alg, key.as_ptr(), key.len(), data.as_ptr(), data.len(), out.as_mut_ptr()); }
        out
    }

    /// HMAC-MD5 (16 bytes).
    pub fn md5(key: &[u8], data: &[u8]) -> Vec<u8> { compute(CC_HMAC_ALG_MD5, CC_MD5_DIGEST_LENGTH, key, data) }
    /// HMAC-SHA1 (20 bytes).
    pub fn sha1(key: &[u8], data: &[u8]) -> Vec<u8> { compute(CC_HMAC_ALG_SHA1, CC_SHA1_DIGEST_LENGTH, key, data) }
    /// HMAC-SHA224 (28 bytes).
    pub fn sha224(key: &[u8], data: &[u8]) -> Vec<u8> { compute(CC_HMAC_ALG_SHA224, CC_SHA224_DIGEST_LENGTH, key, data) }
    /// HMAC-SHA256 (32 bytes).
    pub fn sha256(key: &[u8], data: &[u8]) -> Vec<u8> { compute(CC_HMAC_ALG_SHA256, CC_SHA256_DIGEST_LENGTH, key, data) }
    /// HMAC-SHA384 (48 bytes).
    pub fn sha384(key: &[u8], data: &[u8]) -> Vec<u8> { compute(CC_HMAC_ALG_SHA384, CC_SHA384_DIGEST_LENGTH, key, data) }
    /// HMAC-SHA512 (64 bytes).
    pub fn sha512(key: &[u8], data: &[u8]) -> Vec<u8> { compute(CC_HMAC_ALG_SHA512, CC_SHA512_DIGEST_LENGTH, key, data) }

    /// Verify an HMAC in constant time.
    pub fn verify(expected: &[u8], actual: &[u8]) -> bool {
        if expected.len() != actual.len() { return false; }
        // Constant-time comparison to prevent timing attacks
        let mut diff = 0u8;
        for (a, b) in expected.iter().zip(actual.iter()) {
            diff |= a ^ b;
        }
        diff == 0
    }
}

// ── AES Symmetric Encryption ────────────────────────────────────────────────

/// AES symmetric encryption (CBC mode with PKCS7 padding by default).
pub mod aes {
    use super::*;

    /// Encryption/decryption error.
    #[derive(Debug, Clone)]
    pub struct CryptoError {
        pub status: i32,
        pub message: String,
    }

    impl std::fmt::Display for CryptoError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CryptoError({}): {}", self.status, self.message)
        }
    }

    impl std::error::Error for CryptoError {}

    fn crypt(op: u32, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // Output buffer: data + one block for padding
        let out_len = data.len() + AES_BLOCK_SIZE;
        let mut out = vec![0u8; out_len];
        let mut out_moved = 0usize;

        let iv_ptr = iv.map(|v| v.as_ptr()).unwrap_or(core::ptr::null());

        let status = unsafe {
            CCCrypt(
                op, CC_ALGORITHM_AES, CC_OPTION_PKCS7,
                key.as_ptr(), key.len(),
                iv_ptr,
                data.as_ptr(), data.len(),
                out.as_mut_ptr(), out_len,
                &mut out_moved,
            )
        };

        if status == CC_SUCCESS {
            out.truncate(out_moved);
            Ok(out)
        } else {
            Err(CryptoError {
                status,
                message: match status {
                    -4300 => "Parameter error".into(),
                    -4301 => "Buffer too small".into(),
                    -4302 => "Memory allocation failure".into(),
                    -4303 => "Alignment error".into(),
                    -4304 => "Decode error (bad padding or corrupt data)".into(),
                    -4305 => "Unimplemented".into(),
                    _ => format!("Unknown error {status}"),
                },
            })
        }
    }

    /// AES-CBC encrypt with PKCS7 padding.
    ///
    /// - `key`: 16 (AES-128), 24 (AES-192), or 32 (AES-256) bytes.
    /// - `iv`: 16-byte initialization vector (None for ECB, which is insecure).
    /// - `plaintext`: data to encrypt.
    pub fn encrypt(key: &[u8], iv: Option<&[u8]>, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        crypt(CC_ENCRYPT, key, iv, plaintext)
    }

    /// AES-CBC decrypt with PKCS7 padding removal.
    pub fn decrypt(key: &[u8], iv: Option<&[u8]>, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        crypt(CC_DECRYPT, key, iv, ciphertext)
    }

    /// AES-ECB encrypt (no IV, **insecure** — use CBC instead).
    pub fn encrypt_ecb(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let out_len = plaintext.len() + AES_BLOCK_SIZE;
        let mut out = vec![0u8; out_len];
        let mut out_moved = 0usize;
        let status = unsafe {
            CCCrypt(
                CC_ENCRYPT, CC_ALGORITHM_AES, CC_OPTION_PKCS7 | CC_OPTION_ECB,
                key.as_ptr(), key.len(),
                core::ptr::null(),
                plaintext.as_ptr(), plaintext.len(),
                out.as_mut_ptr(), out_len,
                &mut out_moved,
            )
        };
        if status == CC_SUCCESS { out.truncate(out_moved); Ok(out) }
        else { Err(CryptoError { status, message: format!("ECB encrypt error {status}") }) }
    }

    /// AES-ECB decrypt.
    pub fn decrypt_ecb(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let out_len = ciphertext.len() + AES_BLOCK_SIZE;
        let mut out = vec![0u8; out_len];
        let mut out_moved = 0usize;
        let status = unsafe {
            CCCrypt(
                CC_DECRYPT, CC_ALGORITHM_AES, CC_OPTION_PKCS7 | CC_OPTION_ECB,
                key.as_ptr(), key.len(),
                core::ptr::null(),
                ciphertext.as_ptr(), ciphertext.len(),
                out.as_mut_ptr(), out_len,
                &mut out_moved,
            )
        };
        if status == CC_SUCCESS { out.truncate(out_moved); Ok(out) }
        else { Err(CryptoError { status, message: format!("ECB decrypt error {status}") }) }
    }
}

// ── PBKDF2 Key Derivation ───────────────────────────────────────────────────

/// PBKDF2 password-based key derivation.
pub mod pbkdf2 {
    use super::*;

    /// Pseudo-random function for PBKDF2.
    #[derive(Debug, Clone, Copy)]
    pub enum Prf {
        HmacSha1 = 1,
        HmacSha224 = 2,
        HmacSha256 = 3,
        HmacSha384 = 4,
        HmacSha512 = 5,
    }

    /// Derive a key from a password using PBKDF2.
    ///
    /// - `password`: the text password.
    /// - `salt`: random salt bytes (use `random_bytes(16)` or longer).
    /// - `rounds`: iteration count (10,000+ recommended).
    /// - `key_len`: desired output key length in bytes.
    pub fn derive(password: &str, salt: &[u8], rounds: u32, key_len: usize) -> Vec<u8> {
        derive_with_prf(password, salt, rounds, key_len, Prf::HmacSha256)
    }

    /// Derive with a specific PRF.
    pub fn derive_with_prf(
        password: &str, salt: &[u8], rounds: u32, key_len: usize, prf: Prf,
    ) -> Vec<u8> {
        let mut out = vec![0u8; key_len];
        unsafe {
            CCKeyDerivationPBKDF(
                CC_PBKDF2,
                password.as_ptr(), password.len(),
                salt.as_ptr(), salt.len(),
                prf as u32, rounds,
                out.as_mut_ptr(), key_len,
            );
        }
        out
    }
}

// ── Secure Random ───────────────────────────────────────────────────────────

/// Generate cryptographically secure random bytes.
pub fn random_bytes(count: usize) -> Vec<u8> {
    let mut buf = vec![0u8; count];
    unsafe { CCRandomGenerateBytes(buf.as_mut_ptr(), count); }
    buf
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Hashing ─────────────────────────────────────────────────────────

    #[test]
    fn test_sha256() {
        let h = sha256(b"hello");
        assert_eq!(hex(&h), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test_sha512() {
        let h = sha512(b"hello");
        assert_eq!(&hex(&h)[..16], "9b71d224bd62f378");
    }

    #[test]
    fn test_sha256_empty() {
        let h = sha256(b"");
        assert_eq!(&hex(&h)[..8], "e3b0c442");
    }

    #[test]
    fn test_sha1() {
        let h = sha1(b"hello");
        assert_eq!(hex(&h), "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }

    #[test]
    fn test_md5() {
        let h = md5(b"hello");
        assert_eq!(hex(&h), "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_sha384() {
        let h = sha384(b"hello");
        assert_eq!(h.len(), 48);
    }

    #[test]
    fn test_sha224() {
        let h = sha224(b"hello");
        assert_eq!(h.len(), 28);
    }

    // ── HMAC ────────────────────────────────────────────────────────────

    #[test]
    fn test_hmac_sha256() {
        let mac = hmac::sha256(b"key", b"message");
        assert_eq!(mac.len(), 32);
        // Known HMAC-SHA256("key", "message")
        assert_eq!(&hex(&mac)[..16], "6e9ef29b75fffc5b");
    }

    #[test]
    fn test_hmac_sha1() {
        let mac = hmac::sha1(b"key", b"message");
        assert_eq!(mac.len(), 20);
    }

    #[test]
    fn test_hmac_verify() {
        let mac = hmac::sha256(b"key", b"data");
        let mac2 = hmac::sha256(b"key", b"data");
        assert!(hmac::verify(&mac, &mac2));

        let bad = hmac::sha256(b"wrong", b"data");
        assert!(!hmac::verify(&mac, &bad));
    }

    // ── AES ─────────────────────────────────────────────────────────────

    #[test]
    fn test_aes_cbc_roundtrip() {
        let key = [0u8; 32]; // AES-256
        let iv = [0u8; 16];
        let plaintext = b"hello world from Rust!";

        let ct = aes::encrypt(&key, Some(&iv), plaintext).unwrap();
        assert_ne!(&ct[..], plaintext);

        let pt = aes::decrypt(&key, Some(&iv), &ct).unwrap();
        assert_eq!(&pt, plaintext);
    }

    #[test]
    fn test_aes_128() {
        let key = random_bytes(16);
        let iv = random_bytes(16);
        let msg = b"AES-128 test";

        let ct = aes::encrypt(&key, Some(&iv), msg).unwrap();
        let pt = aes::decrypt(&key, Some(&iv), &ct).unwrap();
        assert_eq!(&pt, msg);
    }

    #[test]
    fn test_aes_ecb_roundtrip() {
        let key = [42u8; 16];
        let msg = b"ECB mode test!!"; // 15 bytes, will be padded

        let ct = aes::encrypt_ecb(&key, msg).unwrap();
        let pt = aes::decrypt_ecb(&key, &ct).unwrap();
        assert_eq!(&pt, msg);
    }

    #[test]
    fn test_aes_wrong_key_garbles() {
        let key = random_bytes(32);
        let iv = random_bytes(16);
        let plaintext = b"secret message that is long enough to span blocks!!";
        let ct = aes::encrypt(&key, Some(&iv), plaintext).unwrap();

        let wrong_key = random_bytes(32);
        // With a wrong key, decrypt either fails (bad padding) or produces garbage
        match aes::decrypt(&wrong_key, Some(&iv), &ct) {
            Err(_) => {} // Expected: padding error
            Ok(pt) => assert_ne!(&pt[..], &plaintext[..], "Wrong key should not produce correct plaintext"),
        }
    }

    // ── PBKDF2 ──────────────────────────────────────────────────────────

    #[test]
    fn test_pbkdf2_deterministic() {
        let dk1 = pbkdf2::derive("password", b"salt", 1000, 32);
        let dk2 = pbkdf2::derive("password", b"salt", 1000, 32);
        assert_eq!(dk1, dk2);
        assert_eq!(dk1.len(), 32);
    }

    #[test]
    fn test_pbkdf2_different_passwords() {
        let dk1 = pbkdf2::derive("password1", b"salt", 1000, 32);
        let dk2 = pbkdf2::derive("password2", b"salt", 1000, 32);
        assert_ne!(dk1, dk2);
    }

    #[test]
    fn test_pbkdf2_different_salts() {
        let dk1 = pbkdf2::derive("password", b"salt1", 1000, 32);
        let dk2 = pbkdf2::derive("password", b"salt2", 1000, 32);
        assert_ne!(dk1, dk2);
    }

    #[test]
    fn test_pbkdf2_sha512() {
        let dk = pbkdf2::derive_with_prf("pass", b"salt", 10000, 64, pbkdf2::Prf::HmacSha512);
        assert_eq!(dk.len(), 64);
    }

    // ── Random ──────────────────────────────────────────────────────────

    #[test]
    fn test_random_bytes() {
        let a = random_bytes(32);
        let b = random_bytes(32);
        assert_eq!(a.len(), 32);
        assert_ne!(a, b);
    }
}

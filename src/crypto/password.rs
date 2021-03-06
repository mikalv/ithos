//! password.rs: Password hashing functionality and password-based key derivation
//!
//! scrypt is presently the only supported password hashing function / KDF
//!

use alg::PasswordAlg;
use errors::*;
use pwhash::scrypt::{self, ScryptParams};
use ring::constant_time;
use ring::rand::SecureRandom;
use rpassword;

// Size of a random salt value to use with a password
const RANDOM_SALT_SIZE: usize = 16;

// Prefix added to all randomly generated passwords
const GENPASS_PREFIX: &'static str = "ITHOS-GENPASS";

#[cfg(not(test))]
#[inline]
fn params() -> ScryptParams {
    ScryptParams::new(16, 8, 1)
}

// Use a weak set of parameters when running tests to reduce test times
// WARNING: do not use these params in release versions of this software!
#[cfg(test)]
fn params() -> ScryptParams {
    ScryptParams::new(1, 1, 1)
}

/// Generate an easy-to-type password from a random number generator
pub fn generate(rng: &SecureRandom) -> String {
    let mut bytes = [0u8; 8];

    // TODO: Don't Panic
    rng.fill(&mut bytes).unwrap();

    format!(
        "{prefix}-{password}-{digits1:02}{digits2:03}",
        prefix = GENPASS_PREFIX,
        password = String::from_utf8(encode_bubblebabble(&bytes[0..6])).unwrap(),
        digits1 = bytes[6] % 100,
        digits2 = bytes[7]
    )
}

/// Generate a random salt to use with a password
pub fn random_salt(rng: &SecureRandom) -> Result<[u8; RANDOM_SALT_SIZE]> {
    let mut salt = [0u8; RANDOM_SALT_SIZE];
    rng.fill(&mut salt)?;
    Ok(salt)
}

/// Prompt for a password from standard input
pub fn prompt(message: &str) -> Result<String> {
    Ok(rpassword::prompt_password_stdout(message)?)
}

/// Derive a cryptographically secure key from the given password and salt
pub fn derive(alg: PasswordAlg, salt: &[u8], password: &str, output_key: &mut [u8]) {
    // scrypt is the only password hashing algorithm we support for now
    assert_eq!(alg, PasswordAlg::SCRYPT);

    scrypt::scrypt(password.as_bytes(), salt, &params(), output_key);
}

/// Verify a password is correct against a previously derived key/digest
#[allow(dead_code)]
pub fn verify(alg: PasswordAlg, salt: &[u8], password: &str, previously_derived: &[u8]) -> bool {
    // scrypt is the only password hashing algorithm we support for now
    assert_eq!(alg, PasswordAlg::SCRYPT);

    let mut out = vec![0u8; previously_derived.len()];
    scrypt::scrypt(password.as_bytes(), &*salt, &params(), &mut out);

    constant_time::verify_slices_are_equal(previously_derived, &out).is_ok()
}

/// Encode random data with the Bubble Babble encoding
fn encode_bubblebabble(bytes: &[u8]) -> Vec<u8> {
    let vowels = b"aeiouy";
    let consonants = b"bcdfghklmnprstvzx";

    let mut result = Vec::new();
    let mut c: usize = 1;

    result.push(b'x');

    #[allow(unknown_lints, needless_range_loop)]
    for i in 0..(bytes.len() + 1) {
        if i % 2 != 0 {
            continue;
        }

        if i >= bytes.len() {
            result.push(vowels[c % 6]);
            result.push(consonants[16]);
            result.push(vowels[c / 6]);
            break;
        }

        let byte1 = bytes[i] as usize;
        result.push(vowels[(((byte1 >> 6) & 3) + c) % 6]);
        result.push(consonants[(byte1 >> 2) & 15]);
        result.push(vowels[((byte1 & 3) + (c / 6)) % 6]);

        if i + 1 >= bytes.len() {
            break;
        }

        let byte2 = bytes[i + 1] as usize;
        result.push(consonants[(byte2 >> 4) & 15]);
        result.push(b'-');
        result.push(consonants[byte2 & 15]);

        c = (c * 5 + byte1 * 7 + byte2) % 36;
    }

    result.push(b'x');
    result
}

#[cfg(test)]
mod tests {
    use alg::PasswordAlg;
    use crypto::password;

    use ring::rand;

    const PASSWORD: &'static str = "The Magic Words are Squeamish Ossifrage";

    #[test]
    fn test_password() {
        let salt = [0u8; 32];
        let mut derived_buf = [0u8; 32];

        password::derive(PasswordAlg::SCRYPT, &salt, PASSWORD, &mut derived_buf);

        assert!(password::verify(
            PasswordAlg::SCRYPT,
            &salt,
            PASSWORD,
            &derived_buf,
        ));
        assert!(!password::verify(
            PasswordAlg::SCRYPT,
            &salt,
            "WRONG",
            &derived_buf,
        ));
    }

    #[test]
    fn test_generator() {
        password::generate(&rand::SystemRandom::new());
    }

    #[test]
    fn test_bubblebabble() {
        assert_eq!(&*password::encode_bubblebabble(b""), b"xexax".as_ref());
        assert_eq!(
            &*password::encode_bubblebabble(b"abcd"),
            b"ximek-domek-gyxox".as_ref()
        );
        assert_eq!(
            &*password::encode_bubblebabble(b"asdf"),
            b"ximel-finek-koxex".as_ref()
        );
        assert_eq!(
            &*password::encode_bubblebabble(b"0123456789"),
            b"xesaf-casef-fytef-hutif-lovof-nixix".as_ref()
        );
        assert_eq!(
            &*password::encode_bubblebabble(b"Testing a sentence."),
            b"xihak-hysul-gapak-venyd-bumud-besek-heryl-gynek-vumuk-hyrox".as_ref()
        );
        assert_eq!(
            &*password::encode_bubblebabble(b"1234567890"),
            b"xesef-disof-gytuf-katof-movif-baxux".as_ref()
        );
        assert_eq!(
            &*password::encode_bubblebabble(b"Pineapple"),
            b"xigak-nyryk-humil-bosek-sonax".as_ref()
        );
    }
}

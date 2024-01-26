use md5::{Digest, Md5};
use sha1::Sha1;
use std::path::PathBuf;

/// import {join} from 'path'
///
/// export function hashToFilename(hash: string): string {
///     // eslint-disable-next-line @typescript-eslint/no-magic-numbers
///     return join(hash.substring(0, 2), hash)
///   }
pub fn hash_to_filename(hash: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(&hash[0..2]);
    path.push(hash);
    path
}

/// import {createHash, Hash} from 'crypto'
///
/// export function validateFile(buffer: Buffer, checkSum: string): boolean {
///     let hash: Hash
///     if (checkSum.length === 32) {
///       hash = createHash('md5')
///     } else {
///       hash = createHash('sha1')
///     }
///     hash.update(buffer)
///     return hash.digest('hex') === checkSum
///   }
pub fn validate_file(buffer: &[u8], check_sum: &str) -> bool {
    match check_sum.len() {
        32 => {
            let mut hasher = Md5::new();
            hasher.update(buffer);
            let result = hasher.finalize();
            let result_str = format!("{:x}", result);
            result_str == check_sum
        }
        _ => {
            let mut hasher = Sha1::new();
            hasher.update(buffer);
            let result = hasher.finalize();
            let result_str = format!("{:x}", result);
            result_str == check_sum
        }
    }
}

#[test]
fn test_hash_to_filename() {
    assert_eq!(
        hash_to_filename("1234567890abcdef"),
        PathBuf::from("12/1234567890abcdef")
    );
}

#[test]
fn test_validate_file() {
    assert_eq!(
        validate_file(b"hello", "5d41402abc4b2a76b9719d911017c592"),
        true
    );
    assert_eq!(
        validate_file(b"hello", "5d41402abc4b2a76b9719d911017c593"),
        false
    );
}

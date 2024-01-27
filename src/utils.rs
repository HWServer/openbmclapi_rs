use crate::cluster::SyncFile;

use std::io::Cursor;

use apache_avro::{from_avro_datum, from_value, types::Value};
use log::{info, warn};
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

/// BYD avro 格式的文件列表
pub const SYNC_FILE_LIST_SCHEMA: &str = r#"
{
    "type": "array",
    "items": {
        "type": "record",
        "name": "file",
        "fields": [
            {"name": "path", "type": "string"},
            {"name": "hash", "type": "string"},
            {"name": "size", "type": "long"}
        ]
    }
}
"#;

/// 用来将 BYD avro 格式的数据转换成文件列表
pub fn avro_data_to_file_list(data: Vec<u8>) -> Option<Vec<SyncFile>> {
    let chema = apache_avro::Schema::parse_str(SYNC_FILE_LIST_SCHEMA).unwrap();
    let mut cur = Cursor::new(data);
    let reader = from_avro_datum(&chema, &mut cur, Some(&chema));
    if reader.is_err() {
        warn!("parse avro data error: {:?}", reader.err());
        return None;
    }
    let value = reader.unwrap();
    match &value {
        Value::Array(arr) => {
            let len = arr.len();
            let mut files = Vec::with_capacity(len);
            info!("got {} files, parsing", len);
            for i in 0..len {
                let item = &arr[i];
                let try_item = from_value::<SyncFile>(item);
                if try_item.is_err() {
                    warn!("parse file error: {:?}", try_item.err());
                    continue;
                }
                files.push(try_item.unwrap());
            }
            info!("parsed {} files", len);
            Some(files)
        }
        _ => {
            warn!("file list avro data execpet a array, got a {:?}", value);
            None
        }
    }
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {
        use log::error;
        // error!() + panic!()
        error!($($arg)+);
        panic!($($arg)+);
    };
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

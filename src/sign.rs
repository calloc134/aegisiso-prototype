use crate::error::ArchisoError;
use pgp::Deserializable; // ← from_string を使うためのトレイト
use pgp::composed::SignedSecretKey;
use pgp::crypto::hash::HashAlgorithm;
use pgp::types::SecretKeyTrait;
use sha2::{Digest, Sha512};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Compute SHA-512 checksum of a file and write to `<file>.sha512`
pub fn sha512_sum_to_file(path: &Path) -> Result<(), ArchisoError> {
    let mut f = File::open(path)?;
    let mut hasher = Sha512::new();
    let mut buf = [0u8; 8192];
    while let Ok(n) = f.read(&mut buf) {
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let sum = hasher.finalize();
    let mut out = File::create(path.with_extension("sha512"))?;
    writeln!(
        out,
        "{:x}  {}",
        sum,
        path.file_name().unwrap().to_string_lossy()
    )?;
    Ok(())
}

/// GPG 風の detached signature を `<file>.sig` として出力します
pub fn sign_detached(path: &Path, keyfile: &str) -> Result<(), ArchisoError> {
    // 1. キーを ASCII-armored から読み込む
    let key_reader = File::open(keyfile)?;
    let (tpk, _) = SignedSecretKey::from_armor_single(key_reader)
        .map_err(|e| ArchisoError::Process(format!("pgp load key failed: {}", e)))?;

    // 2. 対象ファイルの読み込み
    let data = std::fs::read(path)?;

    // 3. SHA-512 ダイジェスト作成
    let digest = Sha512::digest(&data);

    // 4. detached 署名をバイト列として生成
    let sig_bytes = tpk
        .create_signature(|| String::new(), HashAlgorithm::SHA2_512, &digest)
        .map_err(|e| ArchisoError::Process(format!("pgp sign failed: {}", e)))?;

    // 5. .sig ファイルへ書き出し
    let raw: &[u8] = <&[u8]>::try_from(&sig_bytes)
        .map_err(|e| ArchisoError::Process(format!("signature to bytes failed: {}", e)))?;
    let mut out = File::create(path.with_extension("sig"))?;
    out.write_all(raw)?;
    Ok(())
}

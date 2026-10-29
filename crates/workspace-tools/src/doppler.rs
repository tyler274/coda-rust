//! Install the pinned Doppler CLI release for images and runners.

use std::fs::{self, OpenOptions};
use std::io::{self, Cursor, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Component, Path};

use sha2::{Digest, Sha256};
use tar::Archive;

use crate::failure::Failure;

pub const VERSION: &str = "v3.77.1";
// The convenience download endpoint can serve latest despite a version query.
// Pin the release asset itself; Nix reads this same URL and checksum.
pub const URL: &str =
    "https://github.com/DopplerHQ/cli/releases/download/3.77.1/doppler_3.77.1_linux_amd64.tar.gz";
pub const SHA256: &str = "001f89a42629f9cb827cd86683c4c2dbf87c6299a8d6130643b98044bf5ed80a";

pub fn from_args(args: &[String]) -> Result<(), Failure> {
    let [output] = args else {
        return Err(Failure::new(
            2,
            "usage: workspace-tools install-doppler OUTPUT_PATH",
        ));
    };
    install(Path::new(output))
}

pub fn install(output: &Path) -> Result<(), Failure> {
    let bytes = download()?;
    install_verified(&bytes, output)
}

fn download() -> Result<Vec<u8>, Failure> {
    let client = reqwest::blocking::Client::builder()
        .build()
        .map_err(|error| Failure::new(1, format!("could not build the HTTP client: {error}")))?;
    let bytes = client
        .get(URL)
        .send()
        .map_err(|error| Failure::new(1, format!("could not download Doppler: {error}")))?
        .error_for_status()
        .map_err(|error| Failure::new(1, format!("could not download Doppler: {error}")))?
        .bytes()
        .map_err(|error| Failure::new(1, format!("could not download Doppler: {error}")))?;
    Ok(bytes.to_vec())
}

pub fn install_verified(bytes: &[u8], output: &Path) -> Result<(), Failure> {
    let actual = sha256_hex(bytes);
    if actual != SHA256 {
        return Err(Failure::new(
            1,
            format!("Doppler checksum mismatch for {VERSION}"),
        ));
    }
    extract_doppler(bytes, output)?;
    println!("Installed Doppler CLI {VERSION}.");
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        hex.push(char::from(b"0123456789abcdef"[(byte >> 4) as usize]));
        hex.push(char::from(b"0123456789abcdef"[(byte & 0xf) as usize]));
    }
    hex
}

fn extract_doppler(bytes: &[u8], output: &Path) -> Result<(), Failure> {
    // Verify the compressed release asset before decoding or writing anything.
    let mut archive = Archive::new(flate2::read::GzDecoder::new(Cursor::new(bytes)));
    let entries = archive
        .entries()
        .map_err(|error| Failure::new(1, format!("could not read the Doppler archive: {error}")))?;
    for entry in entries {
        let mut entry = entry.map_err(|error| {
            Failure::new(1, format!("could not read the Doppler archive: {error}"))
        })?;
        let path = entry.path().map_err(|error| {
            Failure::new(1, format!("could not read a Doppler archive path: {error}"))
        })?;
        if !is_doppler_entry(&path) {
            continue;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o755)
            .open(output)
            .map_err(|error| {
                Failure::new(1, format!("could not create {}: {error}", output.display()))
            })?;
        io::copy(&mut entry, &mut file).map_err(|error| {
            Failure::new(1, format!("could not write {}: {error}", output.display()))
        })?;
        file.flush().map_err(|error| {
            Failure::new(1, format!("could not write {}: {error}", output.display()))
        })?;
        fs::set_permissions(output, fs::Permissions::from_mode(0o755)).map_err(|error| {
            Failure::new(1, format!("could not chmod {}: {error}", output.display()))
        })?;
        return Ok(());
    }
    Err(Failure::new(1, "the Doppler archive has no doppler entry"))
}

fn is_doppler_entry(path: &Path) -> bool {
    let mut parts = path
        .components()
        .filter(|component| !matches!(component, Component::CurDir));
    matches!(parts.next(), Some(Component::Normal(name)) if name == "doppler")
        && parts.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "doppler-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|duration| duration.as_nanos())
                    .unwrap_or(0)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn sample_archive() -> Vec<u8> {
        let mut builder = tar::Builder::new(Vec::new());
        let data = b"doppler-binary";
        let mut header = tar::Header::new_gnu();
        header.set_path("doppler").unwrap();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder.append(&header, &data[..]).unwrap();
        let tar = builder.into_inner().unwrap();
        let mut gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        gzip.write_all(&tar).unwrap();
        gzip.finish().unwrap()
    }

    #[test]
    fn release_pins_stay_in_sync() {
        let version = VERSION.trim_start_matches('v');
        assert_eq!(
            URL,
            format!(
                "https://github.com/DopplerHQ/cli/releases/download/{version}/doppler_{version}_linux_amd64.tar.gz"
            )
        );
        let nix = include_str!("../../../nix/packages.nix");
        assert!(nix.contains("url = pin \"URL\""));
        assert!(nix.contains("inherit (dopplerPin) url sha256"));
    }

    #[test]
    fn a_bad_checksum_is_rejected_without_network() {
        let temp = TempDir::new("checksum");
        let output = temp.0.join("doppler");
        let error = install_verified(&sample_archive(), &output).unwrap_err();
        assert_eq!(error.code, 1);
        assert!(error.message.contains("checksum"));
        assert!(!output.exists());
    }

    #[test]
    fn usage_exits_2() {
        let missing = from_args(&[]).unwrap_err();
        assert_eq!(missing.code, 2);
        let extra = from_args(&["one".to_owned(), "two".to_owned()]).unwrap_err();
        assert_eq!(extra.code, 2);
        assert!(extra.message.contains("OUTPUT_PATH"));
    }

    #[test]
    fn the_doppler_entry_is_written_mode_0755() {
        let temp = TempDir::new("extract");
        let output = temp.0.join("doppler-cli");
        extract_doppler(&sample_archive(), &output).unwrap();
        assert_eq!(fs::read(&output).unwrap(), b"doppler-binary");
        let mode = fs::metadata(&output).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o755);
    }
}

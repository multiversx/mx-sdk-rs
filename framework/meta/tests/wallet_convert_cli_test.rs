use multiversx_sc_meta::cli::{WalletAction, WalletArgs, WalletConvertArgs};
use multiversx_sc_meta::cmd::wallet_cmd::wallet;
use multiversx_sdk::wallet::Wallet;
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

/// A well-known mnemonic with deterministic key material, taken from the wallet SDK tests.
const KNOWN_MNEMONIC: &str = "acid twice post genre topic observe valid viable gesture fortune funny dawn around blood enemy page update reduce decline van bundle zebra rookie real";
/// Expected private key derived from `KNOWN_MNEMONIC` at account 0, index 0.
const KNOWN_PRIVATE_KEY: &str = "0b7966138e80b8f3bb64046f56aea4250fd7bacad6ed214165cea6767fd0bc2c";
/// Expected public key derived from `KNOWN_MNEMONIC` at account 0, index 0.
const KNOWN_PUBLIC_KEY: &str = "dfefe0453840e5903f2bd519de9b0ed6e9621e57e28ba0b4c1b15115091dd72f";
/// Expected bech32 address (default "erd" HRP) for the above key pair.
const KNOWN_BECH32_ERD: &str = "erd1mlh7q3fcgrjeq0et65vaaxcw6m5ky8jhu296pdxpk9g32zga6uhsemxx2a";

/// Base directory for temporary test artefacts (ignored by git).
const TEMP_DIR: &str = "tests/temp";

/// Returns a path inside `tests/temp/` using the given filename.
/// The directory is created on first use.
fn temp_path(filename: &str) -> PathBuf {
    let dir = PathBuf::from(TEMP_DIR);
    fs::create_dir_all(&dir).expect("failed to create tests/temp");
    dir.join(filename)
}

/// Silently removes a file if it exists, ignoring errors.
fn remove_if_exists(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

// ── mnemonic → pem ───────────────────────────────────────────────────────────

/// Happy path: mnemonic from a file, PEM written to a file.
/// Verifies that the resulting PEM contains exactly the expected private key,
/// public key, and bech32 address.
#[test]
fn test_convert_mnemonic_to_pem_file_in_file_out() {
    let mnemonic_file = temp_path("mnemonic_file_in_file_out.txt");
    let output_pem = temp_path("mnemonic_file_in_file_out.pem");
    remove_if_exists(&mnemonic_file);
    remove_if_exists(&output_pem);

    fs::write(&mnemonic_file, KNOWN_MNEMONIC).unwrap();

    wallet(&WalletArgs {
        command: WalletAction::Convert(WalletConvertArgs {
            from: "mnemonic".to_string(),
            to: "pem".to_string(),
            infile: Some(mnemonic_file.to_str().unwrap().to_string()),
            outfile: Some(output_pem.to_str().unwrap().to_string()),
            ..Default::default()
        }),
    });

    let (private_key, public_key) = Wallet::get_wallet_keys_pem(&output_pem);
    assert_eq!(private_key, KNOWN_PRIVATE_KEY);
    assert_eq!(public_key, KNOWN_PUBLIC_KEY);

    let pem_content = fs::read_to_string(&output_pem).unwrap();
    assert!(
        pem_content.contains(KNOWN_BECH32_ERD),
        "PEM header should contain the bech32 address"
    );

    remove_if_exists(&mnemonic_file);
    remove_if_exists(&output_pem);
}

/// Mnemonic piped via stdin, PEM returned on stdout.
/// Exercises the interactive (no --infile) code path with a controlled stdin.
/// This is the only test that must remain a subprocess invocation, because the
/// code path reads directly from `io::stdin()`.
#[test]
fn test_convert_mnemonic_to_pem_stdin_in_stdout() {
    let sc_meta_bin = env!("CARGO_BIN_EXE_sc-meta");

    let mut child = Command::new(sc_meta_bin)
        .args([
            "wallet",
            "convert",
            "--in-format",
            "mnemonic",
            "--out-format",
            "pem",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn sc-meta");

    // Write the mnemonic to the child's stdin, then drop to signal EOF.
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(KNOWN_MNEMONIC.as_bytes())
            .expect("failed to write to stdin");
    }

    let output = child.wait_with_output().expect("failed to wait for child");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("-----BEGIN PRIVATE KEY for"));
    assert!(stdout.contains(KNOWN_BECH32_ERD));
    assert!(stdout.contains("-----END PRIVATE KEY for"));
}

/// Custom --hrp flag changes the bech32 prefix in the PEM header while keeping
/// the underlying key material identical.
#[test]
fn test_convert_mnemonic_to_pem_custom_hrp() {
    let custom_hrp = "test";
    let mnemonic_file = temp_path("mnemonic_custom_hrp.txt");
    let output_pem = temp_path("mnemonic_custom_hrp.pem");
    remove_if_exists(&mnemonic_file);
    remove_if_exists(&output_pem);

    fs::write(&mnemonic_file, KNOWN_MNEMONIC).unwrap();

    wallet(&WalletArgs {
        command: WalletAction::Convert(WalletConvertArgs {
            from: "mnemonic".to_string(),
            to: "pem".to_string(),
            infile: Some(mnemonic_file.to_str().unwrap().to_string()),
            outfile: Some(output_pem.to_str().unwrap().to_string()),
            hrp: Some(custom_hrp.to_string()),
            ..Default::default()
        }),
    });

    let pem_content = fs::read_to_string(&output_pem).unwrap();
    // bech32 separator between HRP and data is always "1"
    assert!(
        pem_content.contains(&format!("-----BEGIN PRIVATE KEY for {custom_hrp}1")),
        "PEM header should use the custom HRP; got:\n{pem_content}"
    );

    // Key material must be the same regardless of the HRP
    let (private_key, public_key) = Wallet::get_wallet_keys_pem(&output_pem);
    assert_eq!(private_key, KNOWN_PRIVATE_KEY);
    assert_eq!(public_key, KNOWN_PUBLIC_KEY);

    remove_if_exists(&mnemonic_file);
    remove_if_exists(&output_pem);
}

// ── unsupported format pair ───────────────────────────────────────────────────

/// Any conversion path that is not explicitly handled should return without
/// panicking (it prints "Unsupported conversion" to stdout).
#[test]
fn test_convert_unsupported_format_does_not_panic() {
    // mnemonic → keystore-secret is not a supported path; should be a no-op.
    wallet(&WalletArgs {
        command: WalletAction::Convert(WalletConvertArgs {
            from: "mnemonic".to_string(),
            to: "keystore-secret".to_string(),
            ..Default::default()
        }),
    });
}

// ── missing --infile errors ───────────────────────────────────────────────────

/// pem → keystore-secret requires --infile; omitting it must panic with an
/// informative message.
#[test]
fn test_convert_pem_to_keystore_panics_without_infile() {
    let result = std::panic::catch_unwind(|| {
        wallet(&WalletArgs {
            command: WalletAction::Convert(WalletConvertArgs {
                from: "pem".to_string(),
                to: "keystore-secret".to_string(),
                ..Default::default()
            }),
        });
    });
    let err = result.expect_err("expected a panic when --infile is missing");
    let msg = err
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| err.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("<non-string panic>");
    assert!(
        msg.contains("Input file is required for pem format"),
        "unexpected panic message: {msg}"
    );
}

/// keystore-secret → pem requires --infile; omitting it must panic with an
/// informative message.
#[test]
fn test_convert_keystore_to_pem_panics_without_infile() {
    let result = std::panic::catch_unwind(|| {
        wallet(&WalletArgs {
            command: WalletAction::Convert(WalletConvertArgs {
                from: "keystore-secret".to_string(),
                to: "pem".to_string(),
                ..Default::default()
            }),
        });
    });
    let err = result.expect_err("expected a panic when --infile is missing");
    let msg = err
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| err.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("<non-string panic>");
    assert!(
        msg.contains("Input file is required for keystore-secret format"),
        "unexpected panic message: {msg}"
    );
}

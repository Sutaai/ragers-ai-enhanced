//! Integration tests for configuration parsing and validation.
//!
//! These exercise the config → `RawConfig` → `ConfigValidator` seam through real
//! YAML files, matching how the CLI loads `.ragers.yaml`.

use ragers::config::RawConfig;
use ragers::error::{ConfigError, ConfigValidationError};

/// Valid age X25519 public keys (the same fixtures used in `tests/definitions/`).
const USER1: &str = "age1uqwrj0efvkgph205v0629vfdl2p9c2czpqhgd5uscqg7wkzvzf8s0dagj6";
const USER2: &str = "age1he2dpmrv5q3l8u6gurygsuxxqjv647w6fy0fxx6wj2wd2d863szq0dlemx";

/// Write `yaml` to a temp `.ragers.yaml` and parse it.
fn parse(yaml: &str) -> Result<RawConfig, ConfigError> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".ragers.yaml");
    std::fs::write(&path, yaml).unwrap();
    RawConfig::new(&path)
}

/// Assert that parsing `yaml` yields a validation error matching `check`.
fn expect_validation_error(yaml: impl AsRef<str>, check: fn(&ConfigValidationError) -> bool) {
    let err = match parse(yaml.as_ref()) {
        Ok(_config) => panic!("expected a validation error, got a valid config"),
        Err(err) => err,
    };
    let errs = match err {
        ConfigError::ValidationError(errs) => errs,
        other => panic!("expected a validation error, got {other:?}"),
    };
    assert!(
        errs.iter().any(check),
        "expected a matching validation error, got {errs:?}"
    );
}

#[test]
fn accepts_a_valid_config() {
    let yaml = format!(
        "recipients:\n  user1: {USER1}\n  user2: {USER2}\nfiles:\n  - src: file1.txt\n    out: file1.txt.age\n    recipients: [user1]\n"
    );
    assert!(parse(&yaml).is_ok());
}

#[test]
fn rejects_an_invalid_recipient_key() {
    let yaml = "recipients:\n  user1: not-a-valid-age-key\n";
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::InvalidRecipient { .. })
    });
}

#[test]
fn rejects_a_duplicated_recipient_value() {
    let yaml = format!("recipients:\n  user1: {USER1}\n  user2: {USER1}\n");
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::DuplicatedRecipient { .. })
    });
}

#[test]
fn rejects_a_missing_recipients_file() {
    let yaml =
        format!("recipients:\n  user1: {USER1}\n  files:\n    team: /nonexistent/recipients.txt\n");
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::RecipientsFileNotFound { .. })
    });
}

#[test]
fn rejects_a_group_referencing_an_unknown_recipient() {
    let yaml = format!("recipients:\n  user1: {USER1}\n  groups:\n    admins: [user1, ghost]\n");
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::RecipientInGroupNotFound { .. })
    });
}

#[test]
fn rejects_a_duplicated_file_source() {
    let yaml = format!(
        "recipients:\n  user1: {USER1}\nfiles:\n  - src: a.txt\n    out: a.txt.age\n    recipients: [user1]\n  - src: a.txt\n    out: b.txt.age\n    recipients: [user1]\n"
    );
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::FileDuplicatedSource { .. })
    });
}

#[test]
fn rejects_a_duplicated_file_destination() {
    let yaml = format!(
        "recipients:\n  user1: {USER1}\nfiles:\n  - src: a.txt\n    out: out.txt.age\n    recipients: [user1]\n  - src: b.txt\n    out: out.txt.age\n    recipients: [user1]\n"
    );
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::FileDuplicatedDestination { .. })
    });
}

#[test]
fn rejects_a_file_referencing_an_unknown_alias() {
    let yaml = format!(
        "recipients:\n  user1: {USER1}\nfiles:\n  - src: a.txt\n    out: a.txt.age\n    recipients: [ghost]\n"
    );
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::FileAliasNotFound { .. })
    });
}

#[test]
fn rejects_a_duplicated_recipient_in_a_file_entry() {
    let yaml = format!(
        "recipients:\n  user1: {USER1}\nfiles:\n  - src: a.txt\n    out: a.txt.age\n    recipients: [user1, user1]\n"
    );
    expect_validation_error(yaml, |e| {
        matches!(e, ConfigValidationError::FileAliasDuplicated { .. })
    });
}

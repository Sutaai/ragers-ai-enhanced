//! Integration tests for recipient resolution.
//!
//! These exercise the `RecipientsFactory::obtain_for_alias` seam, checking that a
//! direct alias, a group alias, and a recipients-file alias each expand to the
//! right number of recipients, and that an unknown alias is rejected.

use std::collections::HashMap;

use ragers::config::{RawConfigRecipients, RecipientsFactory};

const USER1: &str = "age1uqwrj0efvkgph205v0629vfdl2p9c2czpqhgd5uscqg7wkzvzf8s0dagj6";
const USER2: &str = "age1he2dpmrv5q3l8u6gurygsuxxqjv647w6fy0fxx6wj2wd2d863szq0dlemx";

fn recipients() -> RawConfigRecipients {
    RawConfigRecipients {
        direct: HashMap::from([
            ("user1".to_string(), USER1.to_string()),
            ("user2".to_string(), USER2.to_string()),
        ]),
        groups: HashMap::from([(
            "admins".to_string(),
            vec!["user1".to_string(), "user2".to_string()],
        )]),
        files: HashMap::new(),
    }
}

#[test]
fn a_direct_alias_resolves_to_one_recipient() {
    let recipients = recipients();
    let factory = RecipientsFactory::new(&recipients);
    let mut guard = age::cli_common::StdinGuard::new(false);

    let resolved = factory.obtain_for_alias("user1", &mut guard).unwrap();
    assert_eq!(resolved.len(), 1);
}

#[test]
fn a_group_alias_resolves_to_every_member() {
    let recipients = recipients();
    let factory = RecipientsFactory::new(&recipients);
    let mut guard = age::cli_common::StdinGuard::new(false);

    let resolved = factory
        .obtain_for_alias("group:admins", &mut guard)
        .unwrap();
    assert_eq!(resolved.len(), 2);
}

#[test]
fn a_recipients_file_alias_resolves_to_every_key_in_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let rec_file = dir.path().join("recipients.txt");
    std::fs::write(&rec_file, format!("{USER1}\n{USER2}\n")).unwrap();

    let recipients = RawConfigRecipients {
        direct: HashMap::new(),
        groups: HashMap::new(),
        files: HashMap::from([("team".to_string(), rec_file)]),
    };
    let factory = RecipientsFactory::new(&recipients);
    let mut guard = age::cli_common::StdinGuard::new(false);

    let resolved = factory.obtain_for_alias("file:team", &mut guard).unwrap();
    assert_eq!(resolved.len(), 2);
}

#[test]
fn an_unknown_alias_is_rejected() {
    let recipients = recipients();
    let factory = RecipientsFactory::new(&recipients);
    let mut guard = age::cli_common::StdinGuard::new(false);

    assert!(factory.obtain_for_alias("ghost", &mut guard).is_err());
}

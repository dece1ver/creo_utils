#[cfg(test)]
use super::*;
#[cfg(test)]
use std::fs::{self, File};

#[test]
fn test_filtered() {
    let temp_dir = std::env::temp_dir();
    for i in 0..5 {
        let _ = File::create(temp_dir.join(format!("{i}.igs_postexp_test")));
    }
    let result = temp_dir.filtered(&["igs_postexp_test"]);
    assert!(result.is_ok_and(|r| r.len() == 5));
}

#[test]
fn failed_test_lastest() {
    let empty_dir = vec![];
    assert!(empty_dir.lastest().is_none());
}

#[test]
fn success_test_lastest() {
    let temp_dir = std::env::temp_dir();
    for i in 0..5 {
        let _ = File::create(temp_dir.join(format!("{i}.igs_postexp_test")));
    }
    let result = temp_dir.filtered(&["igs_postexp_test"]).unwrap();
    assert_eq!(
        result.lastest().unwrap().path(),
        temp_dir.join("4.igs_postexp_test")
    );
    for i in 0..5 {
        let _ = fs::remove_file(temp_dir.join(format!("{i}.igs_postexp_test")));
    }
}

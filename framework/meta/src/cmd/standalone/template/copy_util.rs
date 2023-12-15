use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::version_history::validate_template_tag;

const VERSION_0_44_0: usize = 39;

/// Will copy an entire folder according to a whitelist of allowed paths.
///
/// The whitelist is expected to contain paths relative from the source path.
///
/// If a folder is whitelisted, then everything in the folder is considered whitelisted too.
///
/// The function creates all necessary folder structure in the target, no additional preparation required.
pub fn whitelisted_deep_copy(
    source_root: &Path,
    target_root: &Path,
    whitelist: &[String],
    args_tag: &str,
) {
    perform_file_copy(source_root, &PathBuf::new(), target_root, whitelist);
    create_multiversx_json_file(target_root, args_tag);
}

fn is_whitelisted(path: &Path, whitelist: &[String]) -> bool {
    for whitelist_item in whitelist {
        if path.starts_with(whitelist_item) {
            return true;
        }
    }

    false
}

fn create_parent_dir(target: &Path) {
    fs::create_dir_all(target.parent().unwrap()).expect("failed to create copy target dir");
}

fn perform_file_copy(
    source_root: &Path,
    current_relative_path: &Path,
    target_root: &Path,
    whitelist: &[String],
) {
    let source_full = source_root.join(current_relative_path);
    let target_full = target_root.join(current_relative_path);
    let whitelisted = is_whitelisted(current_relative_path, whitelist);

    if source_full.is_dir() {
        if whitelisted {
            // copy the entire folder, indiscriminately
            create_parent_dir(&target_full);
            copy_dir::copy_dir(&source_full, &target_full).unwrap_or_else(|err| {
                panic!(
                    "failed to copy dir from {} to {}: {err:?}",
                    source_full.display(),
                    target_full.display()
                )
            });
        } else {
            // do the same for children
            let read_dir = fs::read_dir(&source_full).expect("error reading directory");
            for child_result in read_dir {
                let child = child_result.unwrap();
                let child_path = child.path();
                let child_relative = child_path.strip_prefix(source_root).unwrap();
                perform_file_copy(source_root, child_relative, target_root, whitelist);
            }
        }
    } else if whitelisted {
        create_parent_dir(&target_full);

        fs::copy(&source_full, &target_full).unwrap_or_else(|err| {
            panic!(
                "failed to copy file from {} to {}: {err:?}",
                source_full.display(),
                target_full.display()
            )
        });
    }
}

fn create_multiversx_json_file(target_root: &Path, args_tag: &str) {
    if validate_template_tag(VERSION_0_44_0, args_tag) {
        return;
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(target_root.join("multiversx.json"))
        .expect("Failed to open multiversx.json file");

    let mut writer = BufWriter::new(file);

    writer
        .write_all(
            br#"{
        "language": "rust"
}"#,
        )
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::is_whitelisted;

    #[test]
    fn test_is_whitelisted() {
        let whitelist = &["a".to_string(), "b/c".to_string()][..];

        assert!(is_whitelisted("a".as_ref(), whitelist));
        assert!(!is_whitelisted("b".as_ref(), whitelist));
        assert!(is_whitelisted("b/c".as_ref(), whitelist));
        assert!(is_whitelisted("b/c/d".as_ref(), whitelist));
    }
}

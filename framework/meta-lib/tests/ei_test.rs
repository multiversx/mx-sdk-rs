use multiversx_sc_meta_lib::ei;

use std::collections::HashSet;

fn list_to_set<'a>(list: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    for &item in list {
        assert!(!set.contains(item), "duplicate item: {item}");
        set.insert(item);
    }
    set
}

fn test_added_names(base: &[&str], added: &[&str], expected_result: &[&str]) {
    let mut check = list_to_set(base);
    for &added_name in added {
        assert!(
            !check.contains(added_name),
            "added name already present: {added_name}"
        );
        check.insert(added_name);
    }
    assert_eq!(check, list_to_set(expected_result));
}

#[test]
fn test_added_names_ei_1_1() {
    test_added_names(ei::EI_1_0_NAMES, ei::EI_1_1_ADDED_NAMES, ei::EI_1_1_NAMES);
}

#[test]
fn test_added_names_ei_1_2() {
    test_added_names(ei::EI_1_1_NAMES, ei::EI_1_2_ADDED_NAMES, ei::EI_1_2_NAMES);
}

#[test]
fn test_added_names_ei_1_3() {
    test_added_names(ei::EI_1_2_NAMES, ei::EI_1_3_ADDED_NAMES, ei::EI_1_3_NAMES);
}

#[test]
fn test_added_names_ei_1_4() {
    test_added_names(ei::EI_1_3_NAMES, ei::EI_1_4_ADDED_NAMES, ei::EI_1_4_NAMES);
}

#[test]
fn test_added_names_ei_1_5() {
    test_added_names(ei::EI_1_4_NAMES, ei::EI_1_5_ADDED_NAMES, ei::EI_1_5_NAMES);
}

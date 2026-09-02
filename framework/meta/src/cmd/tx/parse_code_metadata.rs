use multiversx_sc_scenario::multiversx_sc::types::CodeMetadata;

use crate::cli::cli_args_tx::MetadataArgs;

/// Build a [`CodeMetadata`] bitfield from the CLI [`MetadataArgs`] flags.
/// Defaults match standard contract conventions: upgradeable + readable, not payable.
pub fn parse_code_metadata(meta: &MetadataArgs) -> CodeMetadata {
    let mut flags = CodeMetadata::DEFAULT;
    if !meta.metadata_not_upgradeable {
        flags |= CodeMetadata::UPGRADEABLE;
    }
    if !meta.metadata_not_readable {
        flags |= CodeMetadata::READABLE;
    }
    if meta.metadata_payable {
        flags |= CodeMetadata::PAYABLE;
    }
    if meta.metadata_payable_by_sc {
        flags |= CodeMetadata::PAYABLE_BY_SC;
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(
        not_upgradeable: bool,
        not_readable: bool,
        payable: bool,
        payable_by_sc: bool,
    ) -> MetadataArgs {
        MetadataArgs {
            metadata_not_upgradeable: not_upgradeable,
            metadata_not_readable: not_readable,
            metadata_payable: payable,
            metadata_payable_by_sc: payable_by_sc,
        }
    }

    // ── default flags ──────────────────────────────────────────────────────────

    #[test]
    fn defaults_are_upgradeable_and_readable() {
        let result = parse_code_metadata(&meta(false, false, false, false));
        assert!(result.is_upgradeable());
        assert!(result.is_readable());
        assert!(!result.is_payable());
        assert!(!result.is_payable_by_sc());
    }

    // ── upgradeable flag ───────────────────────────────────────────────────────

    #[test]
    fn not_upgradeable_clears_upgradeable_bit() {
        let result = parse_code_metadata(&meta(true, false, false, false));
        assert!(!result.is_upgradeable());
        assert!(result.is_readable());
    }

    // ── readable flag ──────────────────────────────────────────────────────────

    #[test]
    fn not_readable_clears_readable_bit() {
        let result = parse_code_metadata(&meta(false, true, false, false));
        assert!(result.is_upgradeable());
        assert!(!result.is_readable());
    }

    // ── payable flag ───────────────────────────────────────────────────────────

    #[test]
    fn payable_sets_payable_bit() {
        let result = parse_code_metadata(&meta(false, false, true, false));
        assert!(result.is_payable());
        assert!(!result.is_payable_by_sc());
    }

    // ── payable_by_sc flag ─────────────────────────────────────────────────────

    #[test]
    fn payable_by_sc_sets_payable_by_sc_bit() {
        let result = parse_code_metadata(&meta(false, false, false, true));
        assert!(!result.is_payable());
        assert!(result.is_payable_by_sc());
    }

    // ── combinations ──────────────────────────────────────────────────────────

    #[test]
    fn all_flags_set() {
        let result = parse_code_metadata(&meta(true, true, true, true));
        assert!(!result.is_upgradeable());
        assert!(!result.is_readable());
        assert!(result.is_payable());
        assert!(result.is_payable_by_sc());
    }

    #[test]
    fn payable_and_payable_by_sc_independent() {
        let both = parse_code_metadata(&meta(false, false, true, true));
        assert!(both.is_payable());
        assert!(both.is_payable_by_sc());
    }
}

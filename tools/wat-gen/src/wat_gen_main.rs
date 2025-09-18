mod wat_gen_single_import;

fn main() {
    let signature_map = multiversx_sc_meta_lib::ei::vm_hook_signature_map();
    for &hook_name in multiversx_sc_meta_lib::ei::delta::EI_1_5_ADDED_NAMES {
        wat_gen_single_import::write_sc_files(hook_name, &signature_map);
    }
}

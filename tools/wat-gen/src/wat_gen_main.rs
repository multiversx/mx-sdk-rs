mod wat_gen_single_import;

fn main() {
    for &hook_name in multiversx_sc_meta_lib::ei::EI_1_5_ADDED_NAMES {
        wat_gen_single_import::write_sc_files(hook_name);
    }
}

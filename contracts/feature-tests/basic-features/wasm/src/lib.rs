// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                          408
// Async Callback:                       1
// Total number of exported functions: 410

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    basic_features
    (
        init => init
        panicWithMessage => panic_with_message
        count_ones => count_ones
        endpoint_with_mutable_arg => endpoint_with_mutable_arg
        sqrt_big_uint => sqrt_big_uint
        sqrt_big_uint_ref => sqrt_big_uint_ref
        log2_big_uint => log2_big_uint
        log2_big_uint_ref => log2_big_uint_ref
        pow_big_int => pow_big_int
        pow_big_int_ref => pow_big_int_ref
        pow_big_uint => pow_big_uint
        pow_big_uint_ref => pow_big_uint_ref
        big_uint_to_u64 => big_uint_to_u64
        biguint_overwrite_u64 => biguint_overwrite_u64
        big_uint_zero => big_uint_zero
        big_uint_from_u64_1 => big_uint_from_u64_1
        big_uint_from_u64_2 => big_uint_from_u64_2
        biguint_from_u128 => biguint_from_u128
        big_uint_from_managed_buffer => big_uint_from_managed_buffer
        big_uint_from_managed_buffer_ref => big_uint_from_managed_buffer_ref
        big_int_zero => big_int_zero
        big_int_from_i64_1 => big_int_from_i64_1
        big_int_from_i64_2 => big_int_from_i64_2
        big_uint_eq_u64 => big_uint_eq_u64
        big_int_to_i64 => big_int_to_i64
        bigint_overwrite_i64 => bigint_overwrite_i64
        big_int_to_parts => big_int_to_parts
        big_int_from_biguint => big_int_from_biguint
        add_big_int => add_big_int
        add_big_int_big_uint => add_big_int_big_uint
        add_big_uint_big_int => add_big_uint_big_int
        add_big_int_big_uint_ref => add_big_int_big_uint_ref
        add_big_uint_big_int_ref => add_big_uint_big_int_ref
        add_big_int_ref => add_big_int_ref
        add_big_uint => add_big_uint
        add_big_uint_ref => add_big_uint_ref
        sub_big_int => sub_big_int
        sub_big_int_ref => sub_big_int_ref
        sub_big_uint => sub_big_uint
        sub_big_uint_ref => sub_big_uint_ref
        mul_big_int => mul_big_int
        mul_big_int_ref => mul_big_int_ref
        mul_big_uint => mul_big_uint
        mul_big_uint_ref => mul_big_uint_ref
        div_big_int => div_big_int
        div_big_int_ref => div_big_int_ref
        div_big_uint => div_big_uint
        div_big_uint_ref => div_big_uint_ref
        rem_big_int => rem_big_int
        rem_big_int_ref => rem_big_int_ref
        rem_big_uint => rem_big_uint
        rem_big_uint_ref => rem_big_uint_ref
        add_assign_big_int => add_assign_big_int
        add_assign_big_int_ref => add_assign_big_int_ref
        add_assign_big_uint => add_assign_big_uint
        add_assign_big_uint_ref => add_assign_big_uint_ref
        sub_assign_big_int => sub_assign_big_int
        sub_assign_big_int_ref => sub_assign_big_int_ref
        sub_assign_big_uint => sub_assign_big_uint
        sub_assign_big_uint_ref => sub_assign_big_uint_ref
        mul_assign_big_int => mul_assign_big_int
        mul_assign_big_int_ref => mul_assign_big_int_ref
        mul_assign_big_uint => mul_assign_big_uint
        mul_assign_big_uint_ref => mul_assign_big_uint_ref
        div_assign_big_int => div_assign_big_int
        div_assign_big_int_ref => div_assign_big_int_ref
        div_assign_big_uint => div_assign_big_uint
        div_assign_big_uint_ref => div_assign_big_uint_ref
        rem_assign_big_int => rem_assign_big_int
        rem_assign_big_int_ref => rem_assign_big_int_ref
        rem_assign_big_uint => rem_assign_big_uint
        rem_assign_big_uint_ref => rem_assign_big_uint_ref
        bit_and_big_uint => bit_and_big_uint
        bit_and_big_uint_ref => bit_and_big_uint_ref
        bit_or_big_uint => bit_or_big_uint
        bit_or_big_uint_ref => bit_or_big_uint_ref
        bit_xor_big_uint => bit_xor_big_uint
        bit_xor_big_uint_ref => bit_xor_big_uint_ref
        bit_and_assign_big_uint => bit_and_assign_big_uint
        bit_and_assign_big_uint_ref => bit_and_assign_big_uint_ref
        bit_or_assign_big_uint => bit_or_assign_big_uint
        bit_or_assign_big_uint_ref => bit_or_assign_big_uint_ref
        bit_xor_assign_big_uint => bit_xor_assign_big_uint
        bit_xor_assign_big_uint_ref => bit_xor_assign_big_uint_ref
        shr_big_uint => shr_big_uint
        shr_big_uint_ref => shr_big_uint_ref
        shl_big_uint => shl_big_uint
        shl_big_uint_ref => shl_big_uint_ref
        shr_assign_big_uint => shr_assign_big_uint
        shr_assign_big_uint_ref => shr_assign_big_uint_ref
        shl_assign_big_uint => shl_assign_big_uint
        shl_assign_big_uint_ref => shl_assign_big_uint_ref
        get_block_timestamp => get_block_timestamp
        get_block_nonce => get_block_nonce
        get_block_round => get_block_round
        get_block_epoch => get_block_epoch
        get_block_random_seed => get_block_random_seed
        get_prev_block_timestamp => get_prev_block_timestamp
        get_prev_block_nonce => get_prev_block_nonce
        get_prev_block_round => get_prev_block_round
        get_prev_block_epoch => get_prev_block_epoch
        get_prev_block_random_seed => get_prev_block_random_seed
        get_caller => get_caller
        get_owner_address => get_owner_address
        get_shard_of_address => get_shard_of_address
        is_smart_contract => is_smart_contract
        get_state_root_hash => get_state_root_hash
        get_tx_hash => get_tx_hash
        get_gas_left => get_gas_left
        get_cumulated_validator_rewards => get_cumulated_validator_rewards
        get_code_metadata => get_code_metadata
        is_builtin_function => is_builtin_function
        codec_err_finish => codec_err_finish
        codec_err_storage_key => codec_err_storage_key
        codec_err_storage_get => codec_err_storage_get
        codec_err_storage_set => codec_err_storage_set
        codec_err_event_topic => codec_err_event_topic
        codec_err_event_data => codec_err_event_data
        codec_err_contract_init => codec_err_contract_init
        codec_err_contract_call => codec_err_contract_call
        compute_sha256 => compute_sha256
        compute_keccak256 => compute_keccak256
        compute_ripemd160 => compute_ripemd160
        verify_bls_signature => verify_bls_signature
        verify_ed25519_signature => verify_ed25519_signature
        verify_secp256k1_signature => verify_secp256k1_signature
        verify_custom_secp256k1_signature => verify_custom_secp256k1_signature
        compute_secp256k1_der_signature => compute_secp256k1_der_signature
        echo_u64 => echo_u64
        echo_i64 => echo_i64
        echo_i32 => echo_i32
        echo_u32 => echo_u32
        echo_isize => echo_isize
        echo_usize => echo_usize
        echo_i8 => echo_i8
        echo_u8 => echo_u8
        echo_bool => echo_bool
        echo_opt_bool => echo_opt_bool
        echo_nothing => echo_nothing
        echo_array_u8 => echo_array_u8
        echo_multi_value_u32 => echo_multi_value_u32
        echo_multi_value_tuples => echo_multi_value_tuples
        echo_ser_example_2 => echo_ser_example_2
        echo_simple_enum => echo_simple_enum
        finish_simple_enum_variant_1 => finish_simple_enum_variant_1
        echo_non_zero_usize => echo_non_zero_usize
        echo_some_args_ignore_others => echo_some_args_ignore_others
        echo_arrayvec => echo_arrayvec
        echo_big_uint => echo_big_uint
        echo_big_int => echo_big_int
        echo_managed_buffer => echo_managed_buffer
        echo_managed_address => echo_managed_address
        echo_managed_option => echo_managed_option
        echo_big_int_managed_vec => echo_big_int_managed_vec
        echo_big_int_tuple => echo_big_int_tuple
        echo_big_int_option => echo_big_int_option
        echo_tuple_into_multiresult => echo_tuple_into_multiresult
        echo_managed_vec_of_managed_vec => echo_managed_vec_of_managed_vec
        echo_managed_vec_of_token_identifier => echo_managed_vec_of_token_identifier
        echo_managed_async_result_empty => echo_managed_async_result_empty
        echo_varags_managed_eager => echo_varags_managed_eager
        echo_varags_managed_sum => echo_varags_managed_sum
        compute_get_values => compute_get_values
        compute_create_ec => compute_create_ec
        compute_get_ec_length => compute_get_ec_length
        compute_get_priv_key_byte_length => compute_get_priv_key_byte_length
        compute_ec_add => compute_ec_add
        compute_ec_double => compute_ec_double
        compute_is_on_curve_ec => compute_is_on_curve_ec
        compute_scalar_mult => compute_scalar_mult
        compute_scalar_base_mult => compute_scalar_base_mult
        compute_marshal_ec => compute_marshal_ec
        compute_marshal_compressed_ec => compute_marshal_compressed_ec
        compute_unmarshal_ec => compute_unmarshal_ec
        compute_unmarshal_compressed_ec => compute_unmarshal_compressed_ec
        compute_generate_key_ec => compute_generate_key_ec
        logEventA => log_event_a
        logEventARepeat => log_event_a_repeat
        logEventB => log_event_b
        only_owner_endpoint => only_owner_endpoint
        only_user_account_endpoint => only_user_account_endpoint
        require_equals => require_equals
        sc_panic => sc_panic
        maddress_from_array => maddress_from_array
        maddress_from_managed_buffer => maddress_from_managed_buffer
        mbuffer_new => mbuffer_new
        mbuffer_concat => mbuffer_concat
        mbuffer_copy_slice => mbuffer_copy_slice
        mbuffer_set_random => mbuffer_set_random
        mbuffer_eq => mbuffer_eq
        managed_address_zero => managed_address_zero
        managed_address_eq => managed_address_eq
        managed_vec_new => managed_vec_new
        managed_vec_biguint_push => managed_vec_biguint_push
        managed_vec_biguint_eq => managed_vec_biguint_eq
        managed_vec_address_push => managed_vec_address_push
        managed_vec_set => managed_vec_set
        managed_vec_remove => managed_vec_remove
        managed_vec_find => managed_vec_find
        managed_vec_contains => managed_vec_contains
        managed_ref_explicit => managed_ref_explicit
        storage_read_raw => storage_read_raw
        storage_write_raw => storage_write_raw
        storage_read_from_address => storage_read_from_address
        load_bytes => load_bytes
        load_big_uint => load_big_uint
        load_big_int => load_big_int
        load_u64 => load_u64
        load_usize => load_usize
        load_i64 => load_i64
        load_bool => load_bool
        load_addr => load_addr
        load_opt_addr => load_opt_addr
        is_empty_opt_addr => is_empty_opt_addr
        get_nr_to_clear => get_nr_to_clear
        clear_storage_value => clear_storage_value
        load_ser_2 => load_ser_2
        load_map1 => load_map1
        load_map2 => load_map2
        load_map3 => load_map3
        load_from_address_raw => load_from_address_raw
        store_bytes => store_bytes
        store_big_uint => store_big_uint
        store_big_int => store_big_int
        store_usize => store_usize
        store_i32 => store_i32
        store_u64 => store_u64
        store_i64 => store_i64
        store_bool => store_bool
        store_addr => store_addr
        store_opt_addr => store_opt_addr
        store_ser_2 => store_ser_2
        store_map1 => store_map1
        store_map2 => store_map2
        store_map3 => store_map3
        store_reserved_i64 => store_reserved_i64
        store_reserved_big_uint => store_reserved_big_uint
        store_reserved_vec_u8 => store_reserved_vec_u8
        address_to_id_mapper_get_id => address_to_id_mapper_get_id
        address_to_id_mapper_get_id_non_zero => address_to_id_mapper_get_id_non_zero
        address_to_id_mapper_get_address => address_to_id_mapper_get_address
        address_to_id_mapper_contains => address_to_id_mapper_contains
        address_to_id_mapper_set => address_to_id_mapper_set
        address_to_id_mapper_get_id_or_insert => address_to_id_mapper_get_id_or_insert
        address_to_id_mapper_remove_by_id => address_to_id_mapper_remove_by_id
        address_to_id_mapper_remove_by_address => address_to_id_mapper_remove_by_address
        getListMapper => list_mapper
        listMapperPushBack => list_mapper_push_back
        listMapperPushFront => list_mapper_push_front
        listMapperPopFront => list_mapper_pop_front
        listMapperPopBack => list_mapper_pop_back
        listMapperFront => list_mapper_front
        listMapperBack => list_mapper_back
        listMapperPushAfter => list_mapper_push_after
        listMapperPushBefore => list_mapper_push_before
        listMapperRemoveNode => list_mapper_remove_node
        listMapperRemoveNodeById => list_mapper_remove_node_by_id
        listMapperSetValue => list_mapper_set_value
        listMapperSetValueById => list_mapper_set_value_by_id
        listMapperIterateByHand => list_mapper_iterate_by_hand
        listMapperIterateByIter => list_mapper_iterate_by_iter
        queue_mapper => queue_mapper
        queue_mapper_push_back => queue_mapper_push_back
        queue_mapper_pop_front => queue_mapper_pop_front
        queue_mapper_front => queue_mapper_front
        map_mapper => map_mapper
        map_mapper_keys => map_mapper_keys
        map_mapper_values => map_mapper_values
        map_mapper_insert => map_mapper_insert
        map_mapper_contains_key => map_mapper_contains_key
        map_mapper_get => map_mapper_get
        map_mapper_remove => map_mapper_remove
        map_mapper_entry_or_default_update_increment => map_mapper_entry_or_default_update_increment
        map_mapper_entry_or_insert_default => map_mapper_entry_or_insert_default
        map_mapper_entry_and_modify => map_mapper_entry_and_modify
        map_mapper_entry_or_insert_with_key => map_mapper_entry_or_insert_with_key
        map_storage_mapper_view => map_storage_mapper_view
        map_storage_mapper_insert_default => map_storage_mapper_insert_default
        map_storage_mapper_contains_key => map_storage_mapper_contains_key
        map_storage_mapper_get => map_storage_mapper_get
        map_storage_mapper_insert_value => map_storage_mapper_insert_value
        map_storage_mapper_get_value => map_storage_mapper_get_value
        map_storage_mapper_remove => map_storage_mapper_remove
        map_storage_mapper_clear => map_storage_mapper_clear
        map_storage_mapper_entry_or_default_update_increment => map_storage_mapper_entry_or_default_update_increment
        map_storage_mapper_entry_and_modify_increment_or_default => map_storage_mapper_entry_and_modify_increment_or_default
        map_storage_mapper_entry_or_default_update => map_storage_mapper_entry_or_default_update
        set_mapper => set_mapper
        set_mapper_insert => set_mapper_insert
        set_mapper_contains => set_mapper_contains
        set_mapper_remove => set_mapper_remove
        set_mapper_front => set_mapper_front
        set_mapper_back => set_mapper_back
        set_mapper_next => set_mapper_next
        set_mapper_previous => set_mapper_previous
        set_mapper_iter_from_and_count => set_mapper_iter_from_and_count
        map_my_single_value_mapper => map_my_single_value_mapper
        my_single_value_mapper_increment_1 => my_single_value_mapper_increment_1
        my_single_value_mapper_increment_2 => my_single_value_mapper_increment_2
        my_single_value_mapper_subtract_with_require => my_single_value_mapper_subtract_with_require
        my_single_value_mapper_set_if_empty => my_single_value_mapper_set_if_empty
        clear_single_value_mapper => clear_single_value_mapper
        get_from_address_single_value_mapper => get_from_address_single_value_mapper
        is_empty_single_value_mapper => is_empty_single_value_mapper
        is_empty_at_address_single_value_mapper => is_empty_at_address_single_value_mapper
        raw_byte_length_single_value_mapper => raw_byte_length_single_value_mapper
        set_single_value_mapper_with_key => set_single_value_mapper_with_key
        vec_mapper => vec_mapper
        vec_mapper_push => vec_mapper_push
        vec_mapper_get => vec_mapper_get
        vec_mapper_get_at_address => vec_mapper_get_at_address
        vec_mapper_len => vec_mapper_len
        vec_mapper_len_at_address => vec_mapper_len_at_address
        token_attributes_set => token_attributes_set
        token_attributes_update => token_attributes_update
        token_attributes_get_attributes => token_attributes_get_attributes
        token_attributes_get_nonce => token_attributes_get_nonce
        token_attributes_clear => token_attributes_clear
        token_attributes_has_attributes => token_attributes_has_attributes
        add_to_whitelist => add_to_whitelist
        remove_from_whitelist => remove_from_whitelist
        check_contains => check_contains
        check_contains_at_address => check_contains_at_address
        require_contains => require_contains
        require_contains_at_address => require_contains_at_address
        issue_fungible_default_callback => issue_fungible_default_callback
        issue_fungible_custom_callback => issue_fungible_custom_callback
        issue_and_set_all_roles_fungible => issue_and_set_all_roles_fungible
        set_local_roles_fungible => set_local_roles_fungible
        mint_fungible => mint_fungible
        mint_and_send_fungible => mint_and_send_fungible
        burn_fungible => burn_fungible
        get_balance_fungible => get_balance_fungible
        require_same_token_fungible => require_same_token_fungible
        require_all_same_token_fungible => require_all_same_token_fungible
        getFungibleTokenId => fungible_token_mapper
        issue_and_set_all_roles_meta => issue_and_set_all_roles_meta
        mapper_nft_set_token_id => mapper_nft_set_token_id
        mapper_nft_create => mapper_nft_create
        mapper_nft_create_and_send => mapper_nft_create_and_send
        mapper_nft_add_quantity => mapper_nft_add_quantity
        mapper_nft_add_quantity_and_send => mapper_nft_add_quantity_and_send
        mapper_nft_burn => mapper_nft_burn
        mapper_nft_get_balance => mapper_nft_get_balance
        mapper_get_token_attributes => mapper_get_token_attributes
        getNonFungibleTokenId => non_fungible_token_mapper
        init_unique_id_mapper => init_unique_id_mapper
        unique_id_mapper_get => unique_id_mapper_get
        unique_id_mapper_swap_remove => unique_id_mapper_swap_remove
        unique_id_mapper_set => unique_id_mapper_set
        unique_id_mapper => unique_id_mapper
        unordered_set_mapper => unordered_set_mapper
        unordered_set_mapper_insert => unordered_set_mapper_insert
        unordered_set_mapper_contains => unordered_set_mapper_contains
        unordered_set_mapper_remove => unordered_set_mapper_remove
        managed_struct_eq => managed_struct_eq
        no_overflow_usize => no_overflow_usize
        no_overflow_u8 => no_overflow_u8
        no_overflow_u16 => no_overflow_u16
        no_overflow_u32 => no_overflow_u32
        no_overflow_u64 => no_overflow_u64
        overflow_usize => overflow_usize
        overflow_u8 => overflow_u8
        overflow_u16 => overflow_u16
        overflow_u32 => overflow_u32
        overflow_u64 => overflow_u64
        no_overflow_isize => no_overflow_isize
        no_overflow_i8 => no_overflow_i8
        no_overflow_i16 => no_overflow_i16
        no_overflow_i32 => no_overflow_i32
        no_overflow_i64 => no_overflow_i64
        overflow_isize => overflow_isize
        overflow_i8 => overflow_i8
        overflow_i16 => overflow_i16
        overflow_i32 => overflow_i32
        overflow_i64 => overflow_i64
        token_identifier_egld => token_identifier_egld
        token_identifier_is_valid_1 => token_identifier_is_valid_1
        token_identifier_is_valid_2 => token_identifier_is_valid_2
        non_zero_usize_iter => non_zero_usize_iter
        non_zero_usize_macro => non_zero_usize_macro
        returns_egld_decimal => returns_egld_decimal
        set_contract_address => set_contract_address
        is_empty_at_address => is_empty_at_address
        contains_at_address => contains_at_address
        len_at_address => len_at_address
        next_at_address => next_at_address
        previous_at_address => previous_at_address
        front_at_address => front_at_address
        back_at_address => back_at_address
        keys_at_address => keys_at_address
        values_at_address => values_at_address
        contains_unordered_at_address => contains_unordered_at_address
        get_by_index => get_by_index
        fill_set_mapper => fill_set_mapper
        fill_map_mapper => fill_map_mapper
        fill_unordered_set_mapper => fill_unordered_set_mapper
        get_value_from_address_with_keys => get_value_from_address_with_keys
        managed_decimal_addition => managed_decimal_addition
        managed_decimal_subtraction => managed_decimal_subtraction
        managed_decimal_eq => managed_decimal_eq
        managed_decimal_trunc => managed_decimal_trunc
        managed_decimal_into_raw_units => managed_decimal_into_raw_units
        managed_decimal_ln => managed_decimal_ln
        managed_decimal_log2 => managed_decimal_log2
        managed_decimal_addition_var => managed_decimal_addition_var
        managed_decimal_subtraction_var => managed_decimal_subtraction_var
        managed_decimal_eq_var => managed_decimal_eq_var
        managed_decimal_ln_var => managed_decimal_ln_var
        managed_decimal_log2_var => managed_decimal_log2_var
    )
}

multiversx_sc_wasm_adapter::async_callback! { basic_features }

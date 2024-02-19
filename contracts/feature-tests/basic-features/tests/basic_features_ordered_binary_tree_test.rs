#![allow(deprecated)]

use basic_features::BasicFeatures;
use multiversx_sc::imports::{OrderedBinaryTreeNode, NULL_NODE_ID};
use multiversx_sc_scenario::{
    managed_biguint, rust_biguint, testing_framework::BlockchainStateWrapper,
};

#[test]
fn ordered_binary_tree_test() {
    let mut b_mock = BlockchainStateWrapper::new();
    let user = b_mock.create_user_account(&rust_biguint!(0));
    let sc_wrapper = b_mock.create_sc_account(
        &rust_biguint!(0),
        Some(&user),
        basic_features::contract_obj,
        "rand wasm path",
    );

    b_mock
        .execute_tx(&user, &sc_wrapper, &rust_biguint!(0), |sc| {
            let mut my_tree_mapper = sc.cool_tree();
            let opt_root = my_tree_mapper.get_root();
            assert_eq!(opt_root, None);

            my_tree_mapper.insert_element(5u32.into());
            //////////////////// 5 /////////////////////////////

            let opt_root = my_tree_mapper.get_root();
            assert_eq!(
                opt_root,
                Some(OrderedBinaryTreeNode {
                    current_node_id: 1,
                    left_id: NULL_NODE_ID,
                    right_id: NULL_NODE_ID,
                    parent_id: NULL_NODE_ID,
                    data: managed_biguint!(5)
                })
            );

            let depth = my_tree_mapper.get_depth(&opt_root.unwrap());
            assert_eq!(depth, 1);

            my_tree_mapper.insert_element(3u32.into());
            my_tree_mapper.insert_element(4u32.into());
            my_tree_mapper.insert_element(8u32.into());
            //////////////////// 5 /////////////////////////////
            //////////// 3 ///////////// 8 //////////////////////
            /////////////// 4 /////////////////////////////////////

            let opt_root = my_tree_mapper.get_root();
            assert_eq!(
                opt_root,
                Some(OrderedBinaryTreeNode {
                    current_node_id: 1,
                    left_id: 2,
                    right_id: 4,
                    parent_id: NULL_NODE_ID,
                    data: managed_biguint!(5)
                })
            );

            let opt_found_item = my_tree_mapper.iterative_search(opt_root.clone(), &4u32.into());
            assert_eq!(
                opt_found_item,
                Some(OrderedBinaryTreeNode {
                    current_node_id: 3,
                    left_id: NULL_NODE_ID,
                    right_id: NULL_NODE_ID,
                    parent_id: 2,
                    data: 4u32.into()
                })
            );

            let opt_found_item = my_tree_mapper.recursive_search(opt_root.clone(), &4u32.into());
            assert_eq!(
                opt_found_item,
                Some(OrderedBinaryTreeNode {
                    current_node_id: 3,
                    left_id: NULL_NODE_ID,
                    right_id: NULL_NODE_ID,
                    parent_id: 2,
                    data: 4u32.into()
                })
            );

            let opt_found_item = my_tree_mapper.recursive_search(opt_root.clone(), &50u32.into());
            assert_eq!(opt_found_item, None);

            let depth = my_tree_mapper.get_depth(&opt_root.unwrap());
            assert_eq!(depth, 3);

            my_tree_mapper.delete_node(4u32.into());

            let opt_root = my_tree_mapper.get_root();
            let depth = my_tree_mapper.get_depth(&opt_root.unwrap());
            assert_eq!(depth, 2);

            my_tree_mapper.insert_element(4u32.into());
            my_tree_mapper.delete_node(5u32.into());

            let opt_root = my_tree_mapper.get_root();
            assert_eq!(
                opt_root,
                Some(OrderedBinaryTreeNode {
                    current_node_id: 4,
                    left_id: 2,
                    right_id: NULL_NODE_ID,
                    parent_id: NULL_NODE_ID,
                    data: 8u32.into()
                })
            );
        })
        .assert_ok();
}

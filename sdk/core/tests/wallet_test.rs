use multiversx_sdk::test_wallets;
use multiversx_sdk::wallet::Mnemonic;
use multiversx_sdk::wallet::PublicKey;

#[test]
fn test_private_key_from_mnemonic() {
    let mnemonic = Mnemonic::parse("acid twice post genre topic observe valid viable gesture fortune funny dawn around blood enemy page update reduce decline van bundle zebra rookie real").unwrap();

    let private_key = mnemonic.to_private_key(0, 0).unwrap();
    let public_key = PublicKey::from(&private_key);
    let address = public_key.to_address();
    assert_eq!(
        "0b7966138e80b8f3bb64046f56aea4250fd7bacad6ed214165cea6767fd0bc2c",
        private_key.to_seed_hex()
    );
    assert_eq!(
        "dfefe0453840e5903f2bd519de9b0ed6e9621e57e28ba0b4c1b15115091dd72f",
        public_key.to_hex()
    );
    assert_eq!(
        "erd1mlh7q3fcgrjeq0et65vaaxcw6m5ky8jhu296pdxpk9g32zga6uhsemxx2a",
        address.to_bech32_default().bech32
    );

    let private_key = mnemonic.to_private_key(0, 1).unwrap();
    let public_key = PublicKey::from(&private_key);
    let address = public_key.to_address();
    assert_eq!(
        "1648ad209d6b157a289884933e3bb30f161ec7113221ec16f87c3578b05830b0",
        private_key.to_seed_hex()
    );
    assert_eq!(
        "af8fef070a581873912ccbafb6a78bb9eb4e003085ac43dbbdfa3e20eb93cede",
        public_key.to_hex()
    );
    assert_eq!(
        "erd147877pc2tqv88yfvewhmdfuth845uqpsskky8kaalglzp6unem0qpwh982",
        address.to_bech32_default().bech32
    );
}

#[test]
fn test_get_shard() {
    let alice = test_wallets::alice(); // [1, 57, 71, 46, 255, 104, 134, 119, 26, 152, 47, 48, 131, 218, 93, 66, 31, 36, 194, 145, 129, 230, 56, 136, 34, 141, 200, 28, 166, 13, 105, 225]
    assert_eq!(1, alice.address.shard_of_3().as_u32());

    let bob = test_wallets::bob(); // [128, 73, 214, 57, 229, 166, 152, 13, 28, 210, 57, 42, 188, 206, 65, 2, 156, 218, 116, 161, 86, 53, 35, 162, 2, 240, 150, 65, 204, 38, 24, 248]
    assert_eq!(0, bob.address.shard_of_3().as_u32());

    let carol = test_wallets::carol(); // [178, 161, 21, 85, 206, 82, 30, 73, 68, 224, 154, 177, 117, 73, 216, 91, 72, 125, 205, 38, 200, 75, 80, 23, 163, 158, 49, 163, 103, 8, 137, 186]
    assert_eq!(2, carol.address.shard_of_3().as_u32());

    let heidi = test_wallets::heidi(); // [110, 34, 65, 24, 217, 6, 138, 230, 38, 135, 138, 28, 251, 235, 203, 106, 149, 164, 113, 93, 184, 109, 27, 81, 224, 106, 4, 34, 108, 243, 15, 214]
    assert_eq!(2, heidi.address.shard_of_3().as_u32());

    let mike = test_wallets::mike(); // [227, 42, 254, 220, 144, 79, 225, 147, 151, 70, 173, 151, 59, 235, 56, 53, 99, 207, 99, 100, 43, 166, 105, 179, 4, 15, 155, 148, 40, 165, 237, 96]
    assert_eq!(0, mike.address.shard_of_3().as_u32());

    assert_eq!(0, test_wallets::socrates().address.shard_of_3().as_u32());
    assert_eq!(0, test_wallets::sophocles().address.shard_of_3().as_u32());
    assert_eq!(0, test_wallets::sophie().address.shard_of_3().as_u32());

    assert_eq!(1, test_wallets::silvia().address.shard_of_3().as_u32());
    assert_eq!(1, test_wallets::siobhan().address.shard_of_3().as_u32());
    assert_eq!(1, test_wallets::simon().address.shard_of_3().as_u32());

    assert_eq!(2, test_wallets::szabolcs().address.shard_of_3().as_u32());
    assert_eq!(2, test_wallets::szilard().address.shard_of_3().as_u32());
    assert_eq!(2, test_wallets::szonja().address.shard_of_3().as_u32());
}

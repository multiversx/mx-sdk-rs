use blst::{
    blst_fp, blst_fp2, blst_fp_from_lendian, blst_p1, blst_p1_affine, blst_p1_to_affine, blst_p2,
    blst_p2_affine, blst_p2_to_affine,
    min_sig::{AggregatePublicKey, PublicKey, Signature},
    Pairing, BLST_ERROR,
};
use mcl_rust::{init, CurveType, G1, G2};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;
pub const BLS_DST_VALUE: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

pub fn sha256(data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn keccak256(data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn verify_ed25519(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let key_32: [u8; 32] = if let Ok(key_32) = key.try_into() {
        key_32
    } else {
        return false;
    };
    let signature_64: [u8; 64] = if let Ok(signature_64) = signature.try_into() {
        signature_64
    } else {
        return false;
    };

    let verifying_key_result = VerifyingKey::from_bytes(&key_32);
    let verifying_key = if let Ok(verifying_key) = verifying_key_result {
        verifying_key
    } else {
        return false;
    };

    let sig = Signature::from_bytes(&signature_64);

    let result = verifying_key.verify(message, &sig);
    result.is_ok()
}

pub fn verify_bls(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    init(CurveType::BLS12_381);

    let public_key = create_bls_public_key(key);
    let signature = create_bls_signature(signature);
    let aug_msg = [BLS_DST_VALUE, message].concat();

    let mut pairing = Pairing::new(true, BLS_DST_VALUE);
    let err = pairing.aggregate(&public_key, true, &signature, true, &aug_msg, &[]);

    if err == blst::BLST_ERROR::BLST_SUCCESS {
        return true;
    }

    false
}

fn create_bls_public_key(key: &[u8]) -> blst_p2_affine {
    let mut point_g2 = G2::from_str("1 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582", 10).expect("failed to create");
    point_g2.deserialize(key);

    let pk_x = mcl_g2_coord_to_blst(point_g2.x.serialize());
    let pk_y = mcl_g2_coord_to_blst(point_g2.y.serialize());
    let pk_z = mcl_g2_coord_to_blst(point_g2.z.serialize());

    let pk_jacobian = blst_p2 {
        x: pk_x,
        y: pk_y,
        z: pk_z,
    };

    let mut key = blst_p2_affine::default();
    unsafe { blst_p2_to_affine(&mut key, &pk_jacobian) };

    key
}

fn create_bls_signature(signature: &[u8]) -> blst_p1_affine {
    let mut point_g1 = G1::from_str("1 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569",10).expect("failed to create");
    point_g1.deserialize(signature);

    let sig_x = mcl_g1_coord_to_blst(point_g1.x.serialize());
    let sig_y = mcl_g1_coord_to_blst(point_g1.y.serialize());
    let sig_z = mcl_g1_coord_to_blst(point_g1.z.serialize());

    let sig_jacobian = blst_p1 {
        x: sig_x,
        y: sig_y,
        z: sig_z,
    };

    let mut signature = blst_p1_affine::default();
    unsafe { blst_p1_to_affine(&mut signature, &sig_jacobian) };

    signature
}

fn mcl_g2_coord_to_blst(mcl_g2_coord: Vec<u8>) -> blst_fp2 {
    let mut out = blst_fp2::default();
    unsafe {
        blst_fp_from_lendian(&mut out.fp[0], mcl_g2_coord[0..48].as_ptr()); // real part
        blst_fp_from_lendian(&mut out.fp[1], mcl_g2_coord[48..96].as_ptr()); // imag part
    }
    out
}

fn mcl_g1_coord_to_blst(mcl_g1_coord: Vec<u8>) -> blst_fp {
    let mut out = blst_fp::default();
    unsafe {
        blst_fp_from_lendian(&mut out, mcl_g1_coord.as_ptr());
    }
    out
}

pub fn verify_bls_aggregated_signature(
    keys: Vec<Vec<u8>>,
    message: &[u8],
    signature: &[u8],
) -> bool {
    let mut aggregate_pk = AggregatePublicKey::from_public_key(&PublicKey::default());

    for (i, key) in keys.iter().enumerate() {
        let public_key = PublicKey::from_bytes(key).unwrap_or_else(|e| {
            panic!("Failed to deserialize public key at index {i}: {key:?}. Error: {e:?}")
        });

        aggregate_pk
            .add_public_key(&public_key, true)
            .unwrap_or_else(|e| {
                panic!("Failed to add public key at index {i} to aggregate. Error: {e:?}")
            });
    }

    let signature = Signature::from_bytes(signature)
        .unwrap_or_else(|e| panic!("Failed to deserialize signature: {signature:?}. Error: {e:?}"));

    let verify_response = signature.verify(
        true,
        message,
        BLS_DST_VALUE,
        &[],
        &aggregate_pk.to_public_key(),
        true,
    );

    matches!(verify_response, BLST_ERROR::BLST_SUCCESS)
}

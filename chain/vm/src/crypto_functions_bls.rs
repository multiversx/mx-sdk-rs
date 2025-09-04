use multiversx_bls::{BlsError, G1, G2};

pub const G2_STR: &str= "1 2345388737500083945391657505708625859903954047151773287623537600586029428359739211026111121073980842558223033704140 3558041178357727243543283929018475959655787667816024413880422701270944718005964809191925861299660390662341819212979 1111454484298065649047920916747797835589661734985194316226909186591481448224600088430816898704234962594609579273169 3988173108836042169913782128392219399166696378042311135661652175544044220584995583525611110036064603671142074680982";

pub fn verify_bls(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    if message.is_empty() {
        return false;
    }

    let public_key = match create_public_key_from_bytes(key) {
        Ok(pk) => pk,
        Err(e) => {
            eprintln!("Failed to deserialize public key: {key:?}. Error: {e}");
            return false;
        }
    };

    if !is_public_key_point_valid(&public_key) {
        return false;
    }

    let sign = match create_signature_from_bytes(signature) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to deserialize signature: {signature:?}. Error: {e}");
            return false;
        }
    };

    if !is_sig_valid_point(&sign) {
        return false;
    }

    sign.verify(public_key, message)
}

pub fn verify_bls_aggregated_signature(
    keys: Vec<Vec<u8>>,
    message: &[u8],
    signature: &[u8],
) -> bool {
    if message.is_empty() {
        return false;
    }

    let public_keys = match keys
        .iter()
        .map(|key| create_public_key_from_bytes(key))
        .collect::<Result<Vec<G2>, BlsError>>()
    {
        Ok(pks) => pks,
        Err(e) => {
            eprintln!("Failed to deserialize public keys. Error: {e}");
            return false;
        }
    };

    if public_keys.is_empty() {
        return false;
    }

    let sign = match create_signature_from_bytes(signature) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to deserialize signature: {signature:?}. Error: {e}");
            return false;
        }
    };

    sign.fast_aggregate_verify(&public_keys, message)
}

pub fn verify_bls_signature_share(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    if signature.is_empty() || key.is_empty() || message.is_empty() {
        return false;
    }

    verify_bls(key, message, signature)
}

fn create_public_key_from_bytes(key: &[u8]) -> Result<G2, BlsError> {
    if key.len() != 96 {
        return Err(BlsError::InvalidData);
    }

    let mut public_key = G2::default();

    public_key.set_str(G2_STR);

    if !public_key.deserialize_g2(key) {
        return Err(BlsError::InvalidData);
    }

    Ok(public_key)
}

fn create_signature_from_bytes(signature: &[u8]) -> Result<G1, BlsError> {
    if signature.len() != 48 {
        return Err(BlsError::InvalidData);
    }

    let mut sign = G1::default();

    if !sign.deserialize(signature) {
        return Err(BlsError::InvalidData);
    }

    Ok(sign)
}

fn is_public_key_point_valid(pk: &G2) -> bool {
    !pk.is_zero() && pk.is_valid_order() && pk.is_valid()
}

fn is_sig_valid_point(sig: &G1) -> bool {
    !sig.is_zero() && sig.is_valid_order() && sig.is_valid()
}

use super::{EsdtTokenPayment, ManagedVec};
use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
        CodecFromSelf,
    },
    types::BigUint,
};

use crate as multiversx_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

/// Encodes any type of payment, which either:
/// - EGLD (can be zero in case of no payment whatsoever);
/// - Multi-ESDT (one or more ESDT transfers).
#[derive(
    TopDecode, TopEncode, TypeAbi, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug,
)]
pub enum EgldOrMultiEsdtPayment<M: ManagedTypeApi> {
    Egld(BigUint<M>),
    MultiEsdt(ManagedVec<M, EsdtTokenPayment<M>>),
}

impl<M> CodecFromSelf for EgldOrMultiEsdtPayment<M> where M: ManagedTypeApi {}

// Manual TypeAbi impl for EgldOrMultiEsdtPayment
// impl<M: ManagedTypeApi> multiversx_sc::abi::TypeAbi for EgldOrMultiEsdtPayment<M> {
//     fn type_name() -> multiversx_sc::abi::TypeName {
//         "EgldOrMultiEsdtPayment".into()
//     }

//     fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
//         accumulator: &mut TDC,
//     ) {
//         let type_name = Self::type_name();
//         if !accumulator.contains_type(&type_name) {
//             accumulator.reserve_type_name(type_name.clone());

//             let mut field_descriptions = multiversx_sc::types::heap::Vec::new();

//             let inner_biguint =
//                 multiversx_sc::abi::StructFieldDescription::new(&[], "", <BigUint<M>>::type_name());
//             let mut inner_vec =
//                 crate::types::heap::Vec::<multiversx_sc::abi::StructFieldDescription>::new();
//             inner_vec.push(inner_biguint);

//             field_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
//                 &[],
//                 "Egld",
//                 0usize,
//                 inner_vec,
//             ));
//             <BigUint<M>>::provide_type_descriptions(accumulator);

//             let mut inner_vec_managed =
//             crate::types::heap::Vec::<multiversx_sc::abi::StructFieldDescription>::new();
//             let inner_vec_managed_struct = multiversx_sc::abi::StructFieldDescription::new(
//                 &[],
//                 "",
//                 multiversx_sc::types::ManagedVec::<M, multiversx_sc::types::EsdtTokenPayment<M>>::type_name(),
//             );
//             inner_vec_managed.push(inner_vec_managed_struct);

//             field_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
//                 &[],
//                 "MultiEsdt",
//                 1usize,
//                 inner_vec_managed,
//             ));
//             <multiversx_sc::types::EsdtTokenPayment<M>>::provide_type_descriptions(accumulator);
//             <multiversx_sc::types::ManagedVec::<M, multiversx_sc::types::EsdtTokenPayment<M>>>::provide_type_descriptions(accumulator);

//             accumulator.insert(
//                 type_name.clone(),
//                 multiversx_sc::abi::TypeDescription::new(
//                     &[],
//                     type_name,
//                     multiversx_sc::abi::TypeContents::Enum(field_descriptions),
//                 ),
//             );
//         }
//     }
// }

use std::path::PathBuf;

use num_bigint::BigUint;

use crate::{
    reconstruct_trait::ReconstructorContext,
    serde_raw::ValueSubTree,
    value_interpreter::{
        reconstruct, reconstruct_from_biguint, reconstruct_from_u64, reconstruction_list,
        ExprReconstructorHint,
    },
};

pub struct ReconstructorBuilder {
    pub context: ReconstructorContext,
}

impl Default for ReconstructorBuilder {
    fn default() -> Self {
        Self::new(std::env::current_dir().unwrap())
    }
}

impl ReconstructorBuilder {
    pub fn new(context_path: PathBuf) -> Self {
        ReconstructorBuilder {
            context: ReconstructorContext { context_path },
        }
    }

    pub fn reconstruct(self, value: &[u8], hint: &ExprReconstructorHint) -> ValueSubTree {
        reconstruct(value, hint, &self.context)
    }

    pub fn reconstruct_from_biguint(self, value: BigUint) -> ValueSubTree {
        reconstruct_from_biguint(value, &self.context)
    }

    pub fn reconstruct_from_u64(self, value: u64) -> ValueSubTree {
        reconstruct_from_u64(value, &self.context)
    }

    pub fn reconstruction_list(
        self,
        value: &[&[u8]],
        hint: &ExprReconstructorHint,
    ) -> ValueSubTree {
        reconstruction_list(value, hint, &self.context)
    }
}

pub trait ReconstructableFrom<T> {
    fn reconstruct_from(from: T, builder: &ReconstructorBuilder) -> Self;
}

impl<T> ReconstructableFrom<T> for T {
    fn reconstruct_from(from: T, _builder: &ReconstructorBuilder) -> Self {
        from
    }
}

impl<T: Clone> ReconstructableFrom<&T> for T {
    fn reconstruct_from(from: &T, _builder: &ReconstructorBuilder) -> Self {
        from.clone()
    }
}

pub trait IntoRaw<R> {
    fn into_raw(self) -> R;
}

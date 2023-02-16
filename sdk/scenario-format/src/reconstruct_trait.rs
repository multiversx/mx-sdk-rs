use std::path::PathBuf;

use crate::reconstructor_builder::ReconstructorBuilder;

#[derive(Default)]
pub struct ReconstructorContext {
    pub context_path: PathBuf,
}

impl ReconstructorContext {
    pub fn new(context_path: PathBuf) -> Self {
        ReconstructorContext { context_path }
    }
    pub fn builder() -> ReconstructorBuilder {
        ReconstructorBuilder::default()
    }
}

pub trait ReconstructableFrom<T> {
    fn reconstruct_from(from: T, builder: &ReconstructorContext) -> Self;
}

impl<T> ReconstructableFrom<T> for T {
    fn reconstruct_from(from: T, _builder: &ReconstructorContext) -> Self {
        from
    }
}

impl<T: Clone> ReconstructableFrom<&T> for T {
    fn reconstruct_from(from: &T, _builder: &ReconstructorContext) -> Self {
        from.clone()
    }
}

pub trait IntoRaw<R> {
    fn into_raw(self) -> R;
}

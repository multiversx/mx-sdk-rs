use std::path::PathBuf;

use crate::reconstructor_builder::ReconstructorBuilder;

pub struct ReconstructorContext {
    pub context_path: PathBuf,
}

impl ReconstructorContext {
    pub fn builder() -> ReconstructorBuilder {
        ReconstructorBuilder::default()
    }
}

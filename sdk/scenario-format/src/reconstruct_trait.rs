use std::path::PathBuf;

#[derive(Default)]
pub struct ReconstructorContext {
    pub context_path: PathBuf,
}

impl ReconstructorContext {
    pub fn new(context_path: PathBuf) -> Self {
        ReconstructorContext { context_path }
    }
}

pub trait ReconstructableFrom<T> {
    fn reconstruct_from(from: T, context: &ReconstructorContext) -> Self;
}

impl<T> ReconstructableFrom<T> for T {
    fn reconstruct_from(from: T, _context: &ReconstructorContext) -> Self {
        from
    }
}

impl<T: Clone> ReconstructableFrom<&T> for T {
    fn reconstruct_from(from: &T, _context: &ReconstructorContext) -> Self {
        from.clone()
    }
}

pub trait IntoRaw<R> {
    fn into_raw(self) -> R;
}
#[derive(Default)]
pub struct ReconstructorContext {}

impl ReconstructorContext {
    pub fn new() -> Self {
        ReconstructorContext {}
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

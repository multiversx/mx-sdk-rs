use super::{
    extended_group_element::ExtendedGroupElement, field_element::FieldElement,
    pre_computed_group_element::PreComputedGroupElement,
    projective_group_element::ProjectiveGroupElement,
};

#[derive(Default)]
pub struct CompletedGroupElement {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
    pub t: FieldElement,
}

impl CompletedGroupElement {
    pub fn ge_mixed_add(&mut self, p: &ExtendedGroupElement, q: &PreComputedGroupElement) {
        let mut t0 = FieldElement::default();
        self.x.fe_add(&p.y, &p.x);
        self.y.fe_sub(&p.y, &p.x);
        self.z.fe_mul(&self.x, &q.y_plus_x);
        self.y.fe_mul(&self.y.clone(), &q.y_minus_x);
        self.t.fe_mul(&q.xy2d, &p.t);
        t0.fe_add(&p.z, &p.z);
        self.x.fe_sub(&self.z, &self.y);
        self.y.fe_add(&self.z, &self.y.clone());
        self.z.fe_add(&t0, &self.t);
        self.t.fe_sub(&t0, &self.t.clone());
    }

    pub fn to_extended(&self, r: &mut ExtendedGroupElement) {
        r.x.fe_mul(&self.x, &self.t);
        r.y.fe_mul(&self.y, &self.z);
        r.z.fe_mul(&self.z, &self.t);
        r.t.fe_mul(&self.x, &self.y);
    }

    pub fn to_projective(&self, r: &mut ProjectiveGroupElement) {
        r.x.fe_mul(&self.x, &self.t);
        r.y.fe_mul(&self.y, &self.z);
        r.z.fe_mul(&self.z, &self.t);
    }
}

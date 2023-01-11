use super::{completed_group_element::CompletedGroupElement, field_element::FieldElement};

#[derive(Default)]
pub struct ProjectiveGroupElement {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
}

impl ProjectiveGroupElement {
    pub fn double(&self, r: &mut CompletedGroupElement) {
        let mut t0 = FieldElement::default();

        r.x.fe_square(&self.x);
        r.z.fe_square(&self.y);
        r.t.fe_square_2(&self.z);
        r.y.fe_add(&self.x, &self.y);
        t0.fe_square(&r.y);
        r.y.fe_add(&r.z, &r.x);
        r.z.fe_sub(&r.z.clone(), &r.x);
        r.x.fe_sub(&t0, &r.y);
        r.t.fe_sub(&r.t.clone(), &r.z.clone());
    }
}

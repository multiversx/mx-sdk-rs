use super::{
    completed_group_element::CompletedGroupElement, field_element::FieldElement,
    pre_computed_group_element::PreComputedGroupElement,
    projective_group_element::ProjectiveGroupElement,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct ExtendedGroupElement {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
    pub t: FieldElement,
}

impl ExtendedGroupElement {
    pub fn to_projective(self, r: &mut ProjectiveGroupElement) {
        r.x.fe_copy(&self.x);
        r.y.fe_copy(&self.y);
        r.z.fe_copy(&self.z);
    }

    pub fn double(&self, r: &mut CompletedGroupElement) {
        let mut q = ProjectiveGroupElement::default();

        self.to_projective(&mut q);
        q.double(r);
    }

    pub fn zero(&mut self) {
        self.x.fe_zero();
        self.y.fe_one();
        self.z.fe_one();
        self.t.fe_zero();
    }

    // ge_scalar_mult_base computes h = a*B, where
    //   a = a[0]+256*a[1]+...+256^31 a[31]
    //   B is the Ed25519 base point (x,4/5) with x positive.
    //
    // Preconditions:
    //   a[31] <= 127
    #[allow(clippy::needless_range_loop)]
    pub fn ge_scalar_mult_base(&mut self, a: [u8; 32]) {
        let mut e = [0i8; 64];
        for (i, v) in a.iter().enumerate() {
            e[2 * i] = (v & 15) as i8;
            e[2 * i + 1] = ((v >> 4) & 15) as i8;
        }

        // each e[i] is between 0 and 15 and e[63] is between 0 and 7.

        let mut carry: i8 = 0;
        for i in 0..63 {
            e[i] += carry;
            carry = (e[i] + 8) >> 4;
            e[i] -= carry << 4;
        }

        e[63] += carry;
        // each e[i] is between -8 and 8.

        self.zero();
        let mut t = PreComputedGroupElement::default();
        let mut r = CompletedGroupElement::default();
        for i in (1..64).step_by(2) {
            t.select_point(i / 2, e[i as usize] as i32);
            r.ge_mixed_add(self, &t);
            r.to_extended(self);
        }

        let mut s = ProjectiveGroupElement::default();

        self.double(&mut r);
        r.to_projective(&mut s);
        s.double(&mut r);
        r.to_projective(&mut s);
        s.double(&mut r);
        r.to_projective(&mut s);
        s.double(&mut r);
        r.to_extended(self);

        for i in (0..64).step_by(2) {
            t.select_point(i / 2, e[i as usize] as i32);
            r.ge_mixed_add(self, &t);
            r.to_extended(self);
        }
    }

    pub fn to_bytes(self) -> [u8; 32] {
        let mut recip = FieldElement::default();
        let mut x = FieldElement::default();
        let mut y = FieldElement::default();

        recip.fe_invert(&self.z);
        x.fe_mul(&self.x, &recip);
        y.fe_mul(&self.y, &recip);
        let mut s = y.to_bytes();

        s[31] ^= x.fe_is_negative() << 7;

        s
    }
}

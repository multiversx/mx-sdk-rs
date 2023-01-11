use super::{constant::BASE, field_element::FieldElement};
use std::marker::Copy;

#[derive(Default, Copy, Clone, Debug)]
pub struct PreComputedGroupElement {
    pub y_plus_x: FieldElement,
    pub y_minus_x: FieldElement,
    pub xy2d: FieldElement,
}

impl PreComputedGroupElement {
    pub fn zero(&mut self) {
        self.y_plus_x.fe_one();
        self.y_minus_x.fe_one();
        self.xy2d.fe_zero();
    }

    pub fn cmove(&mut self, u: &PreComputedGroupElement, b: i32) {
        self.y_plus_x.fe_cmove(&u.y_plus_x, b);
        self.y_minus_x.fe_cmove(&u.y_minus_x, b);
        self.xy2d.fe_cmove(&u.xy2d, b);
    }

    pub fn select_point(&mut self, pos: i32, b: i32) {
        let mut minus_t = PreComputedGroupElement::default();
        let b_negative = (b >> 31) & 1;
        let b_abs = b - (((-b_negative) & b) << 1);

        self.zero();

        for i in 0..8 {
            self.cmove(&BASE[pos as usize][i as usize], equal(b_abs, i + 1));
        }

        minus_t.y_plus_x.fe_copy(&self.y_minus_x);
        minus_t.y_minus_x.fe_copy(&self.y_plus_x);
        minus_t.xy2d.fe_neg(&self.xy2d);
        self.cmove(&minus_t, b_negative);
    }
}

// equal returns 1 if b == c and 0 otherwise, assuming that b and c are
// non-negative.
fn equal(b: i32, c: i32) -> i32 {
    if b == c {
        1
    } else {
        0
    }
}

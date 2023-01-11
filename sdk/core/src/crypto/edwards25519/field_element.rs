use std::marker::Copy;

#[derive(Default, Copy, Clone, Debug)]
pub struct FieldElement(pub [i32; 10]);

const ZERO: FieldElement = FieldElement([0; 10]);

impl FieldElement {
    pub fn fe_zero(&mut self) {
        self.0.copy_from_slice(&ZERO.0);
    }

    pub fn fe_one(&mut self) {
        self.fe_zero();
        self.0[0] = 1;
    }

    pub fn fe_add(&mut self, a: &FieldElement, b: &FieldElement) {
        self.0[0] = a.0[0] + b.0[0];
        self.0[1] = a.0[1] + b.0[1];
        self.0[2] = a.0[2] + b.0[2];
        self.0[3] = a.0[3] + b.0[3];
        self.0[4] = a.0[4] + b.0[4];
        self.0[5] = a.0[5] + b.0[5];
        self.0[6] = a.0[6] + b.0[6];
        self.0[7] = a.0[7] + b.0[7];
        self.0[8] = a.0[8] + b.0[8];
        self.0[9] = a.0[9] + b.0[9];
    }

    pub fn fe_sub(&mut self, a: &FieldElement, b: &FieldElement) {
        self.0[0] = a.0[0] - b.0[0];
        self.0[1] = a.0[1] - b.0[1];
        self.0[2] = a.0[2] - b.0[2];
        self.0[3] = a.0[3] - b.0[3];
        self.0[4] = a.0[4] - b.0[4];
        self.0[5] = a.0[5] - b.0[5];
        self.0[6] = a.0[6] - b.0[6];
        self.0[7] = a.0[7] - b.0[7];
        self.0[8] = a.0[8] - b.0[8];
        self.0[9] = a.0[9] - b.0[9];
    }

    fn fe_square_internal(&self) -> (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) {
        let f0 = self.0[0] as i64;
        let f1 = self.0[1] as i64;
        let f2 = self.0[2] as i64;
        let f3 = self.0[3] as i64;
        let f4 = self.0[4] as i64;
        let f5 = self.0[5] as i64;
        let f6 = self.0[6] as i64;
        let f7 = self.0[7] as i64;
        let f8 = self.0[8] as i64;
        let f9 = self.0[9] as i64;
        let f0_2 = (2 * self.0[0]) as i64;
        let f1_2 = (2 * self.0[1]) as i64;
        let f2_2 = (2 * self.0[2]) as i64;
        let f3_2 = (2 * self.0[3]) as i64;
        let f4_2 = (2 * self.0[4]) as i64;
        let f5_2 = (2 * self.0[5]) as i64;
        let f6_2 = (2 * self.0[6]) as i64;
        let f7_2 = (2 * self.0[7]) as i64;
        let f5_38 = 38 * f5; // 1.31*2^30
        let f6_19 = 19 * f6; // 1.31*2^30
        let f7_38 = 38 * f7; // 1.31*2^30
        let f8_19 = 19 * f8; // 1.31*2^30
        let f9_38 = 38 * f9; // 1.31*2^30

        let h0 = f0 * f0 + f1_2 * f9_38 + f2_2 * f8_19 + f3_2 * f7_38 + f4_2 * f6_19 + f5 * f5_38;
        let h1 = f0_2 * f1 + f2 * f9_38 + f3_2 * f8_19 + f4 * f7_38 + f5_2 * f6_19;
        let h2 = f0_2 * f2 + f1_2 * f1 + f3_2 * f9_38 + f4_2 * f8_19 + f5_2 * f7_38 + f6 * f6_19;
        let h3 = f0_2 * f3 + f1_2 * f2 + f4 * f9_38 + f5_2 * f8_19 + f6 * f7_38;
        let h4 = f0_2 * f4 + f1_2 * f3_2 + f2 * f2 + f5_2 * f9_38 + f6_2 * f8_19 + f7 * f7_38;
        let h5 = f0_2 * f5 + f1_2 * f4 + f2_2 * f3 + f6 * f9_38 + f7_2 * f8_19;
        let h6 = f0_2 * f6 + f1_2 * f5_2 + f2_2 * f4 + f3_2 * f3 + f7_2 * f9_38 + f8 * f8_19;
        let h7 = f0_2 * f7 + f1_2 * f6 + f2_2 * f5 + f3_2 * f4 + f8 * f9_38;
        let h8 = f0_2 * f8 + f1_2 * f7_2 + f2_2 * f6 + f3_2 * f5_2 + f4 * f4 + f9 * f9_38;
        let h9 = f0_2 * f9 + f1_2 * f8 + f2_2 * f7 + f3_2 * f6 + f4_2 * f5;

        (h0, h1, h2, h3, h4, h5, h6, h7, h8, h9)
    }

    // fe_square sets h = 2 * f * f
    //
    // Can overlap h with f.
    //
    // Preconditions:
    //    |f| bounded by 1.65*2^26,1.65*2^25,1.65*2^26,1.65*2^25,etc.
    //
    // Postconditions:
    //    |h| bounded by 1.01*2^25,1.01*2^24,1.01*2^25,1.01*2^24,etc.
    // See fe_mul.c for discussion of implementation strategy.
    pub fn fe_square(&mut self, f: &FieldElement) {
        let (h0, h1, h2, h3, h4, h5, h6, h7, h8, h9) = f.fe_square_internal();
        self.fe_combine(h0, h1, h2, h3, h4, h5, h6, h7, h8, h9);
    }

    pub fn fe_square_2(&mut self, f: &FieldElement) {
        let (mut h0, mut h1, mut h2, mut h3, mut h4, mut h5, mut h6, mut h7, mut h8, mut h9) =
            f.fe_square_internal();

        h0 += h0;
        h1 += h1;
        h2 += h2;
        h3 += h3;
        h4 += h4;
        h5 += h5;
        h6 += h6;
        h7 += h7;
        h8 += h8;
        h9 += h9;

        self.fe_combine(h0, h1, h2, h3, h4, h5, h6, h7, h8, h9);
    }

    pub fn fe_copy(&mut self, src: &FieldElement) {
        self.0.copy_from_slice(&src.0);
    }

    pub fn fe_cmove(&mut self, g: &FieldElement, b: i32) {
        let b = -b;
        self.0[0] ^= b & (self.0[0] ^ g.0[0]);
        self.0[1] ^= b & (self.0[1] ^ g.0[1]);
        self.0[2] ^= b & (self.0[2] ^ g.0[2]);
        self.0[3] ^= b & (self.0[3] ^ g.0[3]);
        self.0[4] ^= b & (self.0[4] ^ g.0[4]);
        self.0[5] ^= b & (self.0[5] ^ g.0[5]);
        self.0[6] ^= b & (self.0[6] ^ g.0[6]);
        self.0[7] ^= b & (self.0[7] ^ g.0[7]);
        self.0[8] ^= b & (self.0[8] ^ g.0[8]);
        self.0[9] ^= b & (self.0[9] ^ g.0[9]);
    }

    // FeNeg sets h = -f
    //
    // Preconditions:
    //    |f| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
    //
    // Postconditions:
    //    |h| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
    pub fn fe_neg(&mut self, f: &FieldElement) {
        self.0[0] = -f.0[0];
        self.0[1] = -f.0[1];
        self.0[2] = -f.0[2];
        self.0[3] = -f.0[3];
        self.0[4] = -f.0[4];
        self.0[5] = -f.0[5];
        self.0[6] = -f.0[6];
        self.0[7] = -f.0[7];
        self.0[8] = -f.0[8];
        self.0[9] = -f.0[9];
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fe_combine(
        &mut self,
        h0: i64,
        h1: i64,
        h2: i64,
        h3: i64,
        h4: i64,
        h5: i64,
        h6: i64,
        h7: i64,
        h8: i64,
        h9: i64,
    ) {
        let mut h0 = h0;
        let mut h1 = h1;
        let mut h2 = h2;
        let mut h3 = h3;
        let mut h4 = h4;
        let mut h5 = h5;
        let mut h6 = h6;
        let mut h7 = h7;
        let mut h8 = h8;
        let mut h9 = h9;

        /*
          |h0| <= (1.1*1.1*2^52*(1+19+19+19+19)+1.1*1.1*2^50*(38+38+38+38+38))
            i.e. |h0| <= 1.2*2^59; narrower ranges for h2, h4, h6, h8
          |h1| <= (1.1*1.1*2^51*(1+1+19+19+19+19+19+19+19+19))
            i.e. |h1| <= 1.5*2^58; narrower ranges for h3, h5, h7, h9
        */

        let mut c0 = (h0 + (1 << 25)) >> 26;
        h1 += c0;
        h0 -= c0 << 26;
        let c4 = (h4 + (1 << 25)) >> 26;
        h5 += c4;
        h4 -= c4 << 26;
        /* |h0| <= 2^25 */
        /* |h4| <= 2^25 */
        /* |h1| <= 1.51*2^58 */
        /* |h5| <= 1.51*2^58 */

        let c1 = (h1 + (1 << 24)) >> 25;
        h2 += c1;
        h1 -= c1 << 25;
        let c5 = (h5 + (1 << 24)) >> 25;
        h6 += c5;
        h5 -= c5 << 25;
        /* |h1| <= 2^24; from now on fits into int32 */
        /* |h5| <= 2^24; from now on fits into int32 */
        /* |h2| <= 1.21*2^59 */
        /* |h6| <= 1.21*2^59 */

        let c2 = (h2 + (1 << 25)) >> 26;
        h3 += c2;
        h2 -= c2 << 26;
        let c6 = (h6 + (1 << 25)) >> 26;
        h7 += c6;
        h6 -= c6 << 26;
        /* |h2| <= 2^25; from now on fits into int32 unchanged */
        /* |h6| <= 2^25; from now on fits into int32 unchanged */
        /* |h3| <= 1.51*2^58 */
        /* |h7| <= 1.51*2^58 */

        let c3 = (h3 + (1 << 24)) >> 25;
        h4 += c3;
        h3 -= c3 << 25;
        let c7 = (h7 + (1 << 24)) >> 25;
        h8 += c7;
        h7 -= c7 << 25;
        /* |h3| <= 2^24; from now on fits into int32 unchanged */
        /* |h7| <= 2^24; from now on fits into int32 unchanged */
        /* |h4| <= 1.52*2^33 */
        /* |h8| <= 1.52*2^33 */

        let c4 = (h4 + (1 << 25)) >> 26;
        h5 += c4;
        h4 -= c4 << 26;
        let c8 = (h8 + (1 << 25)) >> 26;
        h9 += c8;
        h8 -= c8 << 26;
        /* |h4| <= 2^25; from now on fits into int32 unchanged */
        /* |h8| <= 2^25; from now on fits into int32 unchanged */
        /* |h5| <= 1.01*2^24 */
        /* |h9| <= 1.51*2^58 */

        let c9 = (h9 + (1 << 24)) >> 25;
        h0 += c9 * 19;
        h9 -= c9 << 25;
        /* |h9| <= 2^24; from now on fits into int32 unchanged */
        /* |h0| <= 1.8*2^37 */

        c0 = (h0 + (1 << 25)) >> 26;
        h1 += c0;
        h0 -= c0 << 26;
        /* |h0| <= 2^25; from now on fits into int32 unchanged */
        /* |h1| <= 1.01*2^24 */

        self.0[0] = h0 as i32;
        self.0[1] = h1 as i32;
        self.0[2] = h2 as i32;
        self.0[3] = h3 as i32;
        self.0[4] = h4 as i32;
        self.0[5] = h5 as i32;
        self.0[6] = h6 as i32;
        self.0[7] = h7 as i32;
        self.0[8] = h8 as i32;
        self.0[9] = h9 as i32;
    }

    // FeMul calculates h = f * g
    // Can overlap h with f or g.
    //
    // Preconditions:
    //    |f| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.
    //    |g| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.
    //
    // Postconditions:
    //    |h| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
    //
    // Notes on implementation strategy:
    //
    // Using schoolbook multiplication.
    // Karatsuba would save a little in some cost models.
    //
    // Most multiplications by 2 and 19 are 32-bit precomputations;
    // cheaper than 64-bit postcomputations.
    //
    // There is one remaining multiplication by 19 in the carry chain;
    // one *19 precomputation can be merged into this,
    // but the resulting data flow is considerably less clean.
    //
    // There are 12 carries below.
    // 10 of them are 2-way parallelizable and vectorizable.
    // Can get away with 11 carries, but then data flow is much deeper.
    //
    // With tighter constraints on inputs, can squeeze carries into int32.
    pub fn fe_mul(&mut self, f: &FieldElement, g: &FieldElement) {
        let f0 = f.0[0] as i64;
        let f1 = f.0[1] as i64;
        let f2 = f.0[2] as i64;
        let f3 = f.0[3] as i64;
        let f4 = f.0[4] as i64;
        let f5 = f.0[5] as i64;
        let f6 = f.0[6] as i64;
        let f7 = f.0[7] as i64;
        let f8 = f.0[8] as i64;
        let f9 = f.0[9] as i64;

        let f1_2 = (2 * f.0[1]) as i64;
        let f3_2 = (2 * f.0[3]) as i64;
        let f5_2 = (2 * f.0[5]) as i64;
        let f7_2 = (2 * f.0[7]) as i64;
        let f9_2 = (2 * f.0[9]) as i64;

        let g0 = g.0[0] as i64;
        let g1 = g.0[1] as i64;
        let g2 = g.0[2] as i64;
        let g3 = g.0[3] as i64;
        let g4 = g.0[4] as i64;
        let g5 = g.0[5] as i64;
        let g6 = g.0[6] as i64;
        let g7 = g.0[7] as i64;
        let g8 = g.0[8] as i64;
        let g9 = g.0[9] as i64;

        let g1_19 = (19 * g.0[1]) as i64; /* 1.4*2^29 */
        let g2_19 = (19 * g.0[2]) as i64; /* 1.4*2^30; still ok */
        let g3_19 = (19 * g.0[3]) as i64;
        let g4_19 = (19 * g.0[4]) as i64;
        let g5_19 = (19 * g.0[5]) as i64;
        let g6_19 = (19 * g.0[6]) as i64;
        let g7_19 = (19 * g.0[7]) as i64;
        let g8_19 = (19 * g.0[8]) as i64;
        let g9_19 = (19 * g.0[9]) as i64;

        let h0 = f0 * g0
            + f1_2 * g9_19
            + f2 * g8_19
            + f3_2 * g7_19
            + f4 * g6_19
            + f5_2 * g5_19
            + f6 * g4_19
            + f7_2 * g3_19
            + f8 * g2_19
            + f9_2 * g1_19;
        let h1 = f0 * g1
            + f1 * g0
            + f2 * g9_19
            + f3 * g8_19
            + f4 * g7_19
            + f5 * g6_19
            + f6 * g5_19
            + f7 * g4_19
            + f8 * g3_19
            + f9 * g2_19;
        let h2 = f0 * g2
            + f1_2 * g1
            + f2 * g0
            + f3_2 * g9_19
            + f4 * g8_19
            + f5_2 * g7_19
            + f6 * g6_19
            + f7_2 * g5_19
            + f8 * g4_19
            + f9_2 * g3_19;
        let h3 = f0 * g3
            + f1 * g2
            + f2 * g1
            + f3 * g0
            + f4 * g9_19
            + f5 * g8_19
            + f6 * g7_19
            + f7 * g6_19
            + f8 * g5_19
            + f9 * g4_19;
        let h4 = f0 * g4
            + f1_2 * g3
            + f2 * g2
            + f3_2 * g1
            + f4 * g0
            + f5_2 * g9_19
            + f6 * g8_19
            + f7_2 * g7_19
            + f8 * g6_19
            + f9_2 * g5_19;
        let h5 = f0 * g5
            + f1 * g4
            + f2 * g3
            + f3 * g2
            + f4 * g1
            + f5 * g0
            + f6 * g9_19
            + f7 * g8_19
            + f8 * g7_19
            + f9 * g6_19;
        let h6 = f0 * g6
            + f1_2 * g5
            + f2 * g4
            + f3_2 * g3
            + f4 * g2
            + f5_2 * g1
            + f6 * g0
            + f7_2 * g9_19
            + f8 * g8_19
            + f9_2 * g7_19;
        let h7 = f0 * g7
            + f1 * g6
            + f2 * g5
            + f3 * g4
            + f4 * g3
            + f5 * g2
            + f6 * g1
            + f7 * g0
            + f8 * g9_19
            + f9 * g8_19;
        let h8 = f0 * g8
            + f1_2 * g7
            + f2 * g6
            + f3_2 * g5
            + f4 * g4
            + f5_2 * g3
            + f6 * g2
            + f7_2 * g1
            + f8 * g0
            + f9_2 * g9_19;
        let h9 = f0 * g9
            + f1 * g8
            + f2 * g7
            + f3 * g6
            + f4 * g5
            + f5 * g4
            + f6 * g3
            + f7 * g2
            + f8 * g1
            + f9 * g0;

        self.fe_combine(h0, h1, h2, h3, h4, h5, h6, h7, h8, h9);
    }

    pub fn fe_invert(&mut self, z: &FieldElement) {
        let mut t0 = FieldElement::default();
        let mut t1 = FieldElement::default();
        let mut t2 = FieldElement::default();
        let mut t3 = FieldElement::default();

        t0.fe_square(z); // 2^1
        t1.fe_square(&t0); // 2^2

        for _ in 1..2 {
            // 2^3
            t1.fe_square(&t1.clone());
        }

        t1.fe_mul(z, &t1.clone()); // 2^3 + 2^0
        t0.fe_mul(&t0.clone(), &t1); // 2^3 + 2^1 + 2^0
        t2.fe_square(&t0); // 2^4 + 2^2 + 2^1
        t1.fe_mul(&t1.clone(), &t2); // 2^4 + 2^3 + 2^2 + 2^1 + 2^0
        t2.fe_square(&t1); // 5,4,3,2,1

        for _ in 1..5 {
            // 9,8,7,6,5
            t2.fe_square(&t2.clone());
        }

        t1.fe_mul(&t2, &t1.clone()); // 9,8,7,6,5,4,3,2,1,0
        t2.fe_square(&t1); // 10..1

        for _ in 1..10 {
            // 19..10
            t2.fe_square(&t2.clone());
        }

        t2.fe_mul(&t2.clone(), &t1); // 19..0
        t3.fe_square(&t2); // 20..1

        for _ in 1..20 {
            // 39..20
            t3.fe_square(&t3.clone());
        }

        t2.fe_mul(&t3, &t2.clone()); // 39..0
        t2.fe_square(&t2.clone()); // 40..1

        for _ in 1..10 {
            // 49..10
            t2.fe_square(&t2.clone());
        }

        t1.fe_mul(&t2, &t1.clone()); // 49..0
        t2.fe_square(&t1); // 50..1

        for _ in 1..50 {
            // 99..50
            t2.fe_square(&t2.clone());
        }

        t2.fe_mul(&t2.clone(), &t1); // 99..0
        t3.fe_square(&t2); // 100..1

        for _ in 1..100 {
            // 199..100
            t3.fe_square(&t3.clone());
        }

        t2.fe_mul(&t3, &t2.clone()); // 199..0
        t2.fe_square(&t2.clone()); // 200..1

        for _ in 1..50 {
            // 249..50
            t2.fe_square(&t2.clone());
        }

        t1.fe_mul(&t2, &t1.clone()); // 249..0
        t1.fe_square(&t1.clone()); // 250..1

        for _ in 1..5 {
            // 254..5
            t1.fe_square(&t1.clone());
        }

        self.fe_mul(&t1, &t0); // 254..5,3,1,0
    }

    // to_bytes marshals h to s.
    // Preconditions:
    //   |h| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
    //
    // Write p=2^255-19; q=floor(h/p).
    // Basic claim: q = floor(2^(-255)(h + 19 2^(-25)h9 + 2^(-1))).
    //
    // Proof:
    //   Have |h|<=p so |q|<=1 so |19^2 2^(-255) q|<1/4.
    //   Also have |h-2^230 h9|<2^230 so |19 2^(-255)(h-2^230 h9)|<1/4.
    //
    //   Write y=2^(-1)-19^2 2^(-255)q-19 2^(-255)(h-2^230 h9).
    //   Then 0<y<1.
    //
    //   Write r=h-pq.
    //   Have 0<=r<=p-1=2^255-20.
    //   Thus 0<=r+19(2^-255)r<r+19(2^-255)2^255<=2^255-1.
    //
    //   Write x=r+19(2^-255)r+y.
    //   Then 0<x<2^255 so floor(2^(-255)x) = 0 so floor(q+2^(-255)x) = q.
    //
    //   Have q+2^(-255)x = 2^(-255)(h + 19 2^(-25) h9 + 2^(-1))
    //   so floor(2^(-255)(h + 19 2^(-25) h9 + 2^(-1))) = q.
    pub fn to_bytes(mut self) -> [u8; 32] {
        let mut carry = [0i32; 10];

        let mut q = (19 * self.0[9] + (1 << 24)) >> 25;
        q = (self.0[0] + q) >> 26;
        q = (self.0[1] + q) >> 25;
        q = (self.0[2] + q) >> 26;
        q = (self.0[3] + q) >> 25;
        q = (self.0[4] + q) >> 26;
        q = (self.0[5] + q) >> 25;
        q = (self.0[6] + q) >> 26;
        q = (self.0[7] + q) >> 25;
        q = (self.0[8] + q) >> 26;
        q = (self.0[9] + q) >> 25;

        // Goal: Output h-(2^255-19)q, which is between 0 and 2^255-20.
        self.0[0] += 19 * q;
        // Goal: Output h-2^255 q, which is between 0 and 2^255-20.

        carry[0] = self.0[0] >> 26;
        self.0[1] += carry[0];
        self.0[0] -= carry[0] << 26;
        carry[1] = self.0[1] >> 25;
        self.0[2] += carry[1];
        self.0[1] -= carry[1] << 25;
        carry[2] = self.0[2] >> 26;
        self.0[3] += carry[2];
        self.0[2] -= carry[2] << 26;
        carry[3] = self.0[3] >> 25;
        self.0[4] += carry[3];
        self.0[3] -= carry[3] << 25;
        carry[4] = self.0[4] >> 26;
        self.0[5] += carry[4];
        self.0[4] -= carry[4] << 26;
        carry[5] = self.0[5] >> 25;
        self.0[6] += carry[5];
        self.0[5] -= carry[5] << 25;
        carry[6] = self.0[6] >> 26;
        self.0[7] += carry[6];
        self.0[6] -= carry[6] << 26;
        carry[7] = self.0[7] >> 25;
        self.0[8] += carry[7];
        self.0[7] -= carry[7] << 25;
        carry[8] = self.0[8] >> 26;
        self.0[9] += carry[8];
        self.0[8] -= carry[8] << 26;
        carry[9] = self.0[9] >> 25;
        self.0[9] -= carry[9] << 25;
        // h10 = carry9

        // Goal: Output h[0]+...+2^255 h10-2^255 q, which is between 0 and 2^255-20.
        // Have h[0]+...+2^230 h[9] between 0 and 2^255-1;
        // evidently 2^255 h10-2^255 q = 0.
        // Goal: Output h[0]+...+2^230 h[9].

        [
            (self.0[0] >> 0) as u8,
            (self.0[0] >> 8) as u8,
            (self.0[0] >> 16) as u8,
            ((self.0[0] >> 24) | (self.0[1] << 2)) as u8,
            (self.0[1] >> 6) as u8,
            (self.0[1] >> 14) as u8,
            ((self.0[1] >> 22) | (self.0[2] << 3)) as u8,
            (self.0[2] >> 5) as u8,
            (self.0[2] >> 13) as u8,
            ((self.0[2] >> 21) | (self.0[3] << 5)) as u8,
            (self.0[3] >> 3) as u8,
            (self.0[3] >> 11) as u8,
            ((self.0[3] >> 19) | (self.0[4] << 6)) as u8,
            (self.0[4] >> 2) as u8,
            (self.0[4] >> 10) as u8,
            (self.0[4] >> 18) as u8,
            (self.0[5] >> 0) as u8,
            (self.0[5] >> 8) as u8,
            (self.0[5] >> 16) as u8,
            ((self.0[5] >> 24) | (self.0[6] << 1)) as u8,
            (self.0[6] >> 7) as u8,
            (self.0[6] >> 15) as u8,
            ((self.0[6] >> 23) | (self.0[7] << 3)) as u8,
            (self.0[7] >> 5) as u8,
            (self.0[7] >> 13) as u8,
            ((self.0[7] >> 21) | (self.0[8] << 4)) as u8,
            (self.0[8] >> 4) as u8,
            (self.0[8] >> 12) as u8,
            ((self.0[8] >> 20) | (self.0[9] << 6)) as u8,
            (self.0[9] >> 2) as u8,
            (self.0[9] >> 10) as u8,
            (self.0[9] >> 18) as u8,
        ]
    }

    pub fn fe_is_negative(&mut self) -> u8 {
        let s = self.to_bytes();
        s[0] & 1
    }
}

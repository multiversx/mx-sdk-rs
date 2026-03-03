#![allow(clippy::identity_op)]

pub mod completed_group_element;
pub mod constant;
pub mod extended_group_element;
pub mod field_element;
pub mod pre_computed_group_element;
pub mod projective_group_element;

fn load3(input: Vec<u8>) -> i64 {
    let mut r = input[0] as i64;
    r |= (input[1] as i64) << 8;
    r |= (input[2] as i64) << 16;
    r
}

fn load4(input: Vec<u8>) -> i64 {
    let mut r = input[0] as i64;
    r |= (input[1] as i64) << 8;
    r |= (input[2] as i64) << 16;
    r |= (input[3] as i64) << 24;
    r
}

// The scalars are GF(2^252 + 27742317777372353535851937790883648493).

// Input:
//   a[0]+256*a[1]+...+256^31*a[31] = a
//   b[0]+256*b[1]+...+256^31*b[31] = b
//   c[0]+256*c[1]+...+256^31*c[31] = c
//
// Output:
//   s[0]+256*s[1]+...+256^31*s[31] = (ab+c) mod l
//   where l = 2^252 + 27742317777372353535851937790883648493.
pub fn sc_mul_add(a: [u8; 32], b: [u8; 32], c: [u8; 32]) -> [u8; 32] {
    let a0 = 2097151 & load3(a[..].to_vec());
    let a1 = 2097151 & (load4(a[2..].to_vec()) >> 5);
    let a2 = 2097151 & (load3(a[5..].to_vec()) >> 2);
    let a3 = 2097151 & (load4(a[7..].to_vec()) >> 7);
    let a4 = 2097151 & (load4(a[10..].to_vec()) >> 4);
    let a5 = 2097151 & (load3(a[13..].to_vec()) >> 1);
    let a6 = 2097151 & (load4(a[15..].to_vec()) >> 6);
    let a7 = 2097151 & (load3(a[18..].to_vec()) >> 3);
    let a8 = 2097151 & load3(a[21..].to_vec());
    let a9 = 2097151 & (load4(a[23..].to_vec()) >> 5);
    let a10 = 2097151 & (load3(a[26..].to_vec()) >> 2);
    let a11 = load4(a[28..].to_vec()) >> 7;
    let b0 = 2097151 & load3(b[..].to_vec());
    let b1 = 2097151 & (load4(b[2..].to_vec()) >> 5);
    let b2 = 2097151 & (load3(b[5..].to_vec()) >> 2);
    let b3 = 2097151 & (load4(b[7..].to_vec()) >> 7);
    let b4 = 2097151 & (load4(b[10..].to_vec()) >> 4);
    let b5 = 2097151 & (load3(b[13..].to_vec()) >> 1);
    let b6 = 2097151 & (load4(b[15..].to_vec()) >> 6);
    let b7 = 2097151 & (load3(b[18..].to_vec()) >> 3);
    let b8 = 2097151 & load3(b[21..].to_vec());
    let b9 = 2097151 & (load4(b[23..].to_vec()) >> 5);
    let b10 = 2097151 & (load3(b[26..].to_vec()) >> 2);
    let b11 = load4(b[28..].to_vec()) >> 7;
    let c0 = 2097151 & load3(c[..].to_vec());
    let c1 = 2097151 & (load4(c[2..].to_vec()) >> 5);
    let c2 = 2097151 & (load3(c[5..].to_vec()) >> 2);
    let c3 = 2097151 & (load4(c[7..].to_vec()) >> 7);
    let c4 = 2097151 & (load4(c[10..].to_vec()) >> 4);
    let c5 = 2097151 & (load3(c[13..].to_vec()) >> 1);
    let c6 = 2097151 & (load4(c[15..].to_vec()) >> 6);
    let c7 = 2097151 & (load3(c[18..].to_vec()) >> 3);
    let c8 = 2097151 & load3(c[21..].to_vec());
    let c9 = 2097151 & (load4(c[23..].to_vec()) >> 5);
    let c10 = 2097151 & (load3(c[26..].to_vec()) >> 2);
    let c11 = load4(c[28..].to_vec()) >> 7;
    let mut carry = [0i64; 23];

    let mut s0 = c0 + a0 * b0;
    let mut s1 = c1 + a0 * b1 + a1 * b0;
    let mut s2 = c2 + a0 * b2 + a1 * b1 + a2 * b0;
    let mut s3 = c3 + a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0;
    let mut s4 = c4 + a0 * b4 + a1 * b3 + a2 * b2 + a3 * b1 + a4 * b0;
    let mut s5 = c5 + a0 * b5 + a1 * b4 + a2 * b3 + a3 * b2 + a4 * b1 + a5 * b0;
    let mut s6 = c6 + a0 * b6 + a1 * b5 + a2 * b4 + a3 * b3 + a4 * b2 + a5 * b1 + a6 * b0;
    let mut s7 = c7 + a0 * b7 + a1 * b6 + a2 * b5 + a3 * b4 + a4 * b3 + a5 * b2 + a6 * b1 + a7 * b0;
    let mut s8 = c8
        + a0 * b8
        + a1 * b7
        + a2 * b6
        + a3 * b5
        + a4 * b4
        + a5 * b3
        + a6 * b2
        + a7 * b1
        + a8 * b0;
    let mut s9 = c9
        + a0 * b9
        + a1 * b8
        + a2 * b7
        + a3 * b6
        + a4 * b5
        + a5 * b4
        + a6 * b3
        + a7 * b2
        + a8 * b1
        + a9 * b0;
    let mut s10 = c10
        + a0 * b10
        + a1 * b9
        + a2 * b8
        + a3 * b7
        + a4 * b6
        + a5 * b5
        + a6 * b4
        + a7 * b3
        + a8 * b2
        + a9 * b1
        + a10 * b0;
    let mut s11 = c11
        + a0 * b11
        + a1 * b10
        + a2 * b9
        + a3 * b8
        + a4 * b7
        + a5 * b6
        + a6 * b5
        + a7 * b4
        + a8 * b3
        + a9 * b2
        + a10 * b1
        + a11 * b0;
    let mut s12 = a1 * b11
        + a2 * b10
        + a3 * b9
        + a4 * b8
        + a5 * b7
        + a6 * b6
        + a7 * b5
        + a8 * b4
        + a9 * b3
        + a10 * b2
        + a11 * b1;
    let mut s13 = a2 * b11
        + a3 * b10
        + a4 * b9
        + a5 * b8
        + a6 * b7
        + a7 * b6
        + a8 * b5
        + a9 * b4
        + a10 * b3
        + a11 * b2;
    let mut s14 =
        a3 * b11 + a4 * b10 + a5 * b9 + a6 * b8 + a7 * b7 + a8 * b6 + a9 * b5 + a10 * b4 + a11 * b3;
    let mut s15 = a4 * b11 + a5 * b10 + a6 * b9 + a7 * b8 + a8 * b7 + a9 * b6 + a10 * b5 + a11 * b4;
    let mut s16 = a5 * b11 + a6 * b10 + a7 * b9 + a8 * b8 + a9 * b7 + a10 * b6 + a11 * b5;
    let mut s17 = a6 * b11 + a7 * b10 + a8 * b9 + a9 * b8 + a10 * b7 + a11 * b6;
    let mut s18 = a7 * b11 + a8 * b10 + a9 * b9 + a10 * b8 + a11 * b7;
    let mut s19 = a8 * b11 + a9 * b10 + a10 * b9 + a11 * b8;
    let mut s20 = a9 * b11 + a10 * b10 + a11 * b9;
    let mut s21 = a10 * b11 + a11 * b10;
    let mut s22 = a11 * b11;
    let mut s23 = 0i64;

    carry[0] = (s0 + (1 << 20)) >> 21;
    s1 += carry[0];
    s0 -= carry[0] << 21;
    carry[2] = (s2 + (1 << 20)) >> 21;
    s3 += carry[2];
    s2 -= carry[2] << 21;
    carry[4] = (s4 + (1 << 20)) >> 21;
    s5 += carry[4];
    s4 -= carry[4] << 21;
    carry[6] = (s6 + (1 << 20)) >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[8] = (s8 + (1 << 20)) >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[10] = (s10 + (1 << 20)) >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;
    carry[12] = (s12 + (1 << 20)) >> 21;
    s13 += carry[12];
    s12 -= carry[12] << 21;
    carry[14] = (s14 + (1 << 20)) >> 21;
    s15 += carry[14];
    s14 -= carry[14] << 21;
    carry[16] = (s16 + (1 << 20)) >> 21;
    s17 += carry[16];
    s16 -= carry[16] << 21;
    carry[18] = (s18 + (1 << 20)) >> 21;
    s19 += carry[18];
    s18 -= carry[18] << 21;
    carry[20] = (s20 + (1 << 20)) >> 21;
    s21 += carry[20];
    s20 -= carry[20] << 21;
    carry[22] = (s22 + (1 << 20)) >> 21;
    s23 += carry[22];
    s22 -= carry[22] << 21;

    carry[1] = (s1 + (1 << 20)) >> 21;
    s2 += carry[1];
    s1 -= carry[1] << 21;
    carry[3] = (s3 + (1 << 20)) >> 21;
    s4 += carry[3];
    s3 -= carry[3] << 21;
    carry[5] = (s5 + (1 << 20)) >> 21;
    s6 += carry[5];
    s5 -= carry[5] << 21;
    carry[7] = (s7 + (1 << 20)) >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[9] = (s9 + (1 << 20)) >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[11] = (s11 + (1 << 20)) >> 21;
    s12 += carry[11];
    s11 -= carry[11] << 21;
    carry[13] = (s13 + (1 << 20)) >> 21;
    s14 += carry[13];
    s13 -= carry[13] << 21;
    carry[15] = (s15 + (1 << 20)) >> 21;
    s16 += carry[15];
    s15 -= carry[15] << 21;
    carry[17] = (s17 + (1 << 20)) >> 21;
    s18 += carry[17];
    s17 -= carry[17] << 21;
    carry[19] = (s19 + (1 << 20)) >> 21;
    s20 += carry[19];
    s19 -= carry[19] << 21;
    carry[21] = (s21 + (1 << 20)) >> 21;
    s22 += carry[21];
    s21 -= carry[21] << 21;

    s11 += s23 * 666643;
    s12 += s23 * 470296;
    s13 += s23 * 654183;
    s14 -= s23 * 997805;
    s15 += s23 * 136657;
    s16 -= s23 * 683901;
    // s23 = 0;

    s10 += s22 * 666643;
    s11 += s22 * 470296;
    s12 += s22 * 654183;
    s13 -= s22 * 997805;
    s14 += s22 * 136657;
    s15 -= s22 * 683901;
    // s22 = 0;

    s9 += s21 * 666643;
    s10 += s21 * 470296;
    s11 += s21 * 654183;
    s12 -= s21 * 997805;
    s13 += s21 * 136657;
    s14 -= s21 * 683901;
    // s21 = 0;

    s8 += s20 * 666643;
    s9 += s20 * 470296;
    s10 += s20 * 654183;
    s11 -= s20 * 997805;
    s12 += s20 * 136657;
    s13 -= s20 * 683901;
    // s20 = 0;

    s7 += s19 * 666643;
    s8 += s19 * 470296;
    s9 += s19 * 654183;
    s10 -= s19 * 997805;
    s11 += s19 * 136657;
    s12 -= s19 * 683901;
    // s19 = 0;

    s6 += s18 * 666643;
    s7 += s18 * 470296;
    s8 += s18 * 654183;
    s9 -= s18 * 997805;
    s10 += s18 * 136657;
    s11 -= s18 * 683901;
    // s18 = 0;

    carry[6] = (s6 + (1 << 20)) >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[8] = (s8 + (1 << 20)) >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[10] = (s10 + (1 << 20)) >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;
    carry[12] = (s12 + (1 << 20)) >> 21;
    s13 += carry[12];
    s12 -= carry[12] << 21;
    carry[14] = (s14 + (1 << 20)) >> 21;
    s15 += carry[14];
    s14 -= carry[14] << 21;
    carry[16] = (s16 + (1 << 20)) >> 21;
    s17 += carry[16];
    s16 -= carry[16] << 21;

    carry[7] = (s7 + (1 << 20)) >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[9] = (s9 + (1 << 20)) >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[11] = (s11 + (1 << 20)) >> 21;
    s12 += carry[11];
    s11 -= carry[11] << 21;
    carry[13] = (s13 + (1 << 20)) >> 21;
    s14 += carry[13];
    s13 -= carry[13] << 21;
    carry[15] = (s15 + (1 << 20)) >> 21;
    s16 += carry[15];
    s15 -= carry[15] << 21;

    s5 += s17 * 666643;
    s6 += s17 * 470296;
    s7 += s17 * 654183;
    s8 -= s17 * 997805;
    s9 += s17 * 136657;
    s10 -= s17 * 683901;
    // s17 = 0;

    s4 += s16 * 666643;
    s5 += s16 * 470296;
    s6 += s16 * 654183;
    s7 -= s16 * 997805;
    s8 += s16 * 136657;
    s9 -= s16 * 683901;
    // s16 = 0;

    s3 += s15 * 666643;
    s4 += s15 * 470296;
    s5 += s15 * 654183;
    s6 -= s15 * 997805;
    s7 += s15 * 136657;
    s8 -= s15 * 683901;
    // s15 = 0;

    s2 += s14 * 666643;
    s3 += s14 * 470296;
    s4 += s14 * 654183;
    s5 -= s14 * 997805;
    s6 += s14 * 136657;
    s7 -= s14 * 683901;
    // s14 = 0;

    s1 += s13 * 666643;
    s2 += s13 * 470296;
    s3 += s13 * 654183;
    s4 -= s13 * 997805;
    s5 += s13 * 136657;
    s6 -= s13 * 683901;
    // s13 = 0;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    s12 = 0;

    carry[0] = (s0 + (1 << 20)) >> 21;
    s1 += carry[0];
    s0 -= carry[0] << 21;
    carry[2] = (s2 + (1 << 20)) >> 21;
    s3 += carry[2];
    s2 -= carry[2] << 21;
    carry[4] = (s4 + (1 << 20)) >> 21;
    s5 += carry[4];
    s4 -= carry[4] << 21;
    carry[6] = (s6 + (1 << 20)) >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[8] = (s8 + (1 << 20)) >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[10] = (s10 + (1 << 20)) >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;

    carry[1] = (s1 + (1 << 20)) >> 21;
    s2 += carry[1];
    s1 -= carry[1] << 21;
    carry[3] = (s3 + (1 << 20)) >> 21;
    s4 += carry[3];
    s3 -= carry[3] << 21;
    carry[5] = (s5 + (1 << 20)) >> 21;
    s6 += carry[5];
    s5 -= carry[5] << 21;
    carry[7] = (s7 + (1 << 20)) >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[9] = (s9 + (1 << 20)) >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[11] = (s11 + (1 << 20)) >> 21;
    s12 += carry[11];
    s11 -= carry[11] << 21;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    s12 = 0;

    carry[0] = s0 >> 21;
    s1 += carry[0];
    s0 -= carry[0] << 21;
    carry[1] = s1 >> 21;
    s2 += carry[1];
    s1 -= carry[1] << 21;
    carry[2] = s2 >> 21;
    s3 += carry[2];
    s2 -= carry[2] << 21;
    carry[3] = s3 >> 21;
    s4 += carry[3];
    s3 -= carry[3] << 21;
    carry[4] = s4 >> 21;
    s5 += carry[4];
    s4 -= carry[4] << 21;
    carry[5] = s5 >> 21;
    s6 += carry[5];
    s5 -= carry[5] << 21;
    carry[6] = s6 >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[7] = s7 >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[8] = s8 >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[9] = s9 >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[10] = s10 >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;
    carry[11] = s11 >> 21;
    s12 += carry[11];
    s11 -= carry[11] << 21;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    // s12 = 0;

    carry[0] = s0 >> 21;
    s1 += carry[0];
    s0 -= carry[0] << 21;
    carry[1] = s1 >> 21;
    s2 += carry[1];
    s1 -= carry[1] << 21;
    carry[2] = s2 >> 21;
    s3 += carry[2];
    s2 -= carry[2] << 21;
    carry[3] = s3 >> 21;
    s4 += carry[3];
    s3 -= carry[3] << 21;
    carry[4] = s4 >> 21;
    s5 += carry[4];
    s4 -= carry[4] << 21;
    carry[5] = s5 >> 21;
    s6 += carry[5];
    s5 -= carry[5] << 21;
    carry[6] = s6 >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[7] = s7 >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[8] = s8 >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[9] = s9 >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[10] = s10 >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;

    [
        (s0 >> 0) as u8,
        (s0 >> 8) as u8,
        ((s0 >> 16) | (s1 << 5)) as u8,
        (s1 >> 3) as u8,
        (s1 >> 11) as u8,
        ((s1 >> 19) | (s2 << 2)) as u8,
        (s2 >> 6) as u8,
        ((s2 >> 14) | (s3 << 7)) as u8,
        (s3 >> 1) as u8,
        (s3 >> 9) as u8,
        ((s3 >> 17) | (s4 << 4)) as u8,
        (s4 >> 4) as u8,
        (s4 >> 12) as u8,
        ((s4 >> 20) | (s5 << 1)) as u8,
        (s5 >> 7) as u8,
        ((s5 >> 15) | (s6 << 6)) as u8,
        (s6 >> 2) as u8,
        (s6 >> 10) as u8,
        ((s6 >> 18) | (s7 << 3)) as u8,
        (s7 >> 5) as u8,
        (s7 >> 13) as u8,
        (s8 >> 0) as u8,
        (s8 >> 8) as u8,
        ((s8 >> 16) | (s9 << 5)) as u8,
        (s9 >> 3) as u8,
        (s9 >> 11) as u8,
        ((s9 >> 19) | (s10 << 2)) as u8,
        (s10 >> 6) as u8,
        ((s10 >> 14) | (s11 << 7)) as u8,
        (s11 >> 1) as u8,
        (s11 >> 9) as u8,
        (s11 >> 17) as u8,
    ]
}

// Input:
//   s[0]+256*s[1]+...+256^63*s[63] = s
//
// Output:
//   s[0]+256*s[1]+...+256^31*s[31] = s mod l
//   where l = 2^252 + 27742317777372353535851937790883648493.
pub fn sc_reduce(s: [u8; 64]) -> [u8; 32] {
    let mut s0 = 2097151 & load3(s[..].to_vec());
    let mut s1 = 2097151 & (load4(s[2..].to_vec()) >> 5);
    let mut s2 = 2097151 & (load3(s[5..].to_vec()) >> 2);
    let mut s3 = 2097151 & (load4(s[7..].to_vec()) >> 7);
    let mut s4 = 2097151 & (load4(s[10..].to_vec()) >> 4);
    let mut s5 = 2097151 & (load3(s[13..].to_vec()) >> 1);
    let mut s6 = 2097151 & (load4(s[15..].to_vec()) >> 6);
    let mut s7 = 2097151 & (load3(s[18..].to_vec()) >> 3);
    let mut s8 = 2097151 & load3(s[21..].to_vec());
    let mut s9 = 2097151 & (load4(s[23..].to_vec()) >> 5);
    let mut s10 = 2097151 & (load3(s[26..].to_vec()) >> 2);
    let mut s11 = 2097151 & (load4(s[28..].to_vec()) >> 7);
    let mut s12 = 2097151 & (load4(s[31..].to_vec()) >> 4);
    let mut s13 = 2097151 & (load3(s[34..].to_vec()) >> 1);
    let mut s14 = 2097151 & (load4(s[36..].to_vec()) >> 6);
    let mut s15 = 2097151 & (load3(s[39..].to_vec()) >> 3);
    let mut s16 = 2097151 & load3(s[42..].to_vec());
    let mut s17 = 2097151 & (load4(s[44..].to_vec()) >> 5);
    let s18 = 2097151 & (load3(s[47..].to_vec()) >> 2);
    let s19 = 2097151 & (load4(s[49..].to_vec()) >> 7);
    let s20 = 2097151 & (load4(s[52..].to_vec()) >> 4);
    let s21 = 2097151 & (load3(s[55..].to_vec()) >> 1);
    let s22 = 2097151 & (load4(s[57..].to_vec()) >> 6);
    let s23 = load4(s[60..].to_vec()) >> 3;

    s11 += s23 * 666643;
    s12 += s23 * 470296;
    s13 += s23 * 654183;
    s14 -= s23 * 997805;
    s15 += s23 * 136657;
    s16 -= s23 * 683901;
    // s23 = 0;

    s10 += s22 * 666643;
    s11 += s22 * 470296;
    s12 += s22 * 654183;
    s13 -= s22 * 997805;
    s14 += s22 * 136657;
    s15 -= s22 * 683901;
    // s22 = 0;

    s9 += s21 * 666643;
    s10 += s21 * 470296;
    s11 += s21 * 654183;
    s12 -= s21 * 997805;
    s13 += s21 * 136657;
    s14 -= s21 * 683901;
    // s21 = 0;

    s8 += s20 * 666643;
    s9 += s20 * 470296;
    s10 += s20 * 654183;
    s11 -= s20 * 997805;
    s12 += s20 * 136657;
    s13 -= s20 * 683901;
    // s20 = 0;

    s7 += s19 * 666643;
    s8 += s19 * 470296;
    s9 += s19 * 654183;
    s10 -= s19 * 997805;
    s11 += s19 * 136657;
    s12 -= s19 * 683901;
    // s19 = 0;

    s6 += s18 * 666643;
    s7 += s18 * 470296;
    s8 += s18 * 654183;
    s9 -= s18 * 997805;
    s10 += s18 * 136657;
    s11 -= s18 * 683901;
    // s18 = 0;

    let mut carry = [0i64; 17];

    carry[6] = (s6 + (1 << 20)) >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[8] = (s8 + (1 << 20)) >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[10] = (s10 + (1 << 20)) >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;
    carry[12] = (s12 + (1 << 20)) >> 21;
    s13 += carry[12];
    s12 -= carry[12] << 21;
    carry[14] = (s14 + (1 << 20)) >> 21;
    s15 += carry[14];
    s14 -= carry[14] << 21;
    carry[16] = (s16 + (1 << 20)) >> 21;
    s17 += carry[16];
    s16 -= carry[16] << 21;

    carry[7] = (s7 + (1 << 20)) >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[9] = (s9 + (1 << 20)) >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[11] = (s11 + (1 << 20)) >> 21;
    s12 += carry[11];
    s11 -= carry[11] << 21;
    carry[13] = (s13 + (1 << 20)) >> 21;
    s14 += carry[13];
    s13 -= carry[13] << 21;
    carry[15] = (s15 + (1 << 20)) >> 21;
    s16 += carry[15];
    s15 -= carry[15] << 21;

    s5 += s17 * 666643;
    s6 += s17 * 470296;
    s7 += s17 * 654183;
    s8 -= s17 * 997805;
    s9 += s17 * 136657;
    s10 -= s17 * 683901;
    // s17 = 0;

    s4 += s16 * 666643;
    s5 += s16 * 470296;
    s6 += s16 * 654183;
    s7 -= s16 * 997805;
    s8 += s16 * 136657;
    s9 -= s16 * 683901;
    // s16 = 0;

    s3 += s15 * 666643;
    s4 += s15 * 470296;
    s5 += s15 * 654183;
    s6 -= s15 * 997805;
    s7 += s15 * 136657;
    s8 -= s15 * 683901;
    // s15 = 0;

    s2 += s14 * 666643;
    s3 += s14 * 470296;
    s4 += s14 * 654183;
    s5 -= s14 * 997805;
    s6 += s14 * 136657;
    s7 -= s14 * 683901;
    // s14 = 0;

    s1 += s13 * 666643;
    s2 += s13 * 470296;
    s3 += s13 * 654183;
    s4 -= s13 * 997805;
    s5 += s13 * 136657;
    s6 -= s13 * 683901;
    // s13 = 0;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    s12 = 0;

    carry[0] = (s0 + (1 << 20)) >> 21;
    s1 += carry[0];
    s0 -= carry[0] << 21;
    carry[2] = (s2 + (1 << 20)) >> 21;
    s3 += carry[2];
    s2 -= carry[2] << 21;
    carry[4] = (s4 + (1 << 20)) >> 21;
    s5 += carry[4];
    s4 -= carry[4] << 21;
    carry[6] = (s6 + (1 << 20)) >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[8] = (s8 + (1 << 20)) >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[10] = (s10 + (1 << 20)) >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;

    carry[1] = (s1 + (1 << 20)) >> 21;
    s2 += carry[1];
    s1 -= carry[1] << 21;
    carry[3] = (s3 + (1 << 20)) >> 21;
    s4 += carry[3];
    s3 -= carry[3] << 21;
    carry[5] = (s5 + (1 << 20)) >> 21;
    s6 += carry[5];
    s5 -= carry[5] << 21;
    carry[7] = (s7 + (1 << 20)) >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[9] = (s9 + (1 << 20)) >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[11] = (s11 + (1 << 20)) >> 21;
    s12 += carry[11];
    s11 -= carry[11] << 21;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    s12 = 0;

    carry[0] = s0 >> 21;
    s1 += carry[0];
    s0 -= carry[0] << 21;
    carry[1] = s1 >> 21;
    s2 += carry[1];
    s1 -= carry[1] << 21;
    carry[2] = s2 >> 21;
    s3 += carry[2];
    s2 -= carry[2] << 21;
    carry[3] = s3 >> 21;
    s4 += carry[3];
    s3 -= carry[3] << 21;
    carry[4] = s4 >> 21;
    s5 += carry[4];
    s4 -= carry[4] << 21;
    carry[5] = s5 >> 21;
    s6 += carry[5];
    s5 -= carry[5] << 21;
    carry[6] = s6 >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[7] = s7 >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[8] = s8 >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[9] = s9 >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[10] = s10 >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;
    carry[11] = s11 >> 21;
    s12 += carry[11];
    s11 -= carry[11] << 21;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    // s12 = 0;

    carry[0] = s0 >> 21;
    s1 += carry[0];
    s0 -= carry[0] << 21;
    carry[1] = s1 >> 21;
    s2 += carry[1];
    s1 -= carry[1] << 21;
    carry[2] = s2 >> 21;
    s3 += carry[2];
    s2 -= carry[2] << 21;
    carry[3] = s3 >> 21;
    s4 += carry[3];
    s3 -= carry[3] << 21;
    carry[4] = s4 >> 21;
    s5 += carry[4];
    s4 -= carry[4] << 21;
    carry[5] = s5 >> 21;
    s6 += carry[5];
    s5 -= carry[5] << 21;
    carry[6] = s6 >> 21;
    s7 += carry[6];
    s6 -= carry[6] << 21;
    carry[7] = s7 >> 21;
    s8 += carry[7];
    s7 -= carry[7] << 21;
    carry[8] = s8 >> 21;
    s9 += carry[8];
    s8 -= carry[8] << 21;
    carry[9] = s9 >> 21;
    s10 += carry[9];
    s9 -= carry[9] << 21;
    carry[10] = s10 >> 21;
    s11 += carry[10];
    s10 -= carry[10] << 21;

    [
        (s0 >> 0) as u8,
        (s0 >> 8) as u8,
        ((s0 >> 16) | (s1 << 5)) as u8,
        (s1 >> 3) as u8,
        (s1 >> 11) as u8,
        ((s1 >> 19) | (s2 << 2)) as u8,
        (s2 >> 6) as u8,
        ((s2 >> 14) | (s3 << 7)) as u8,
        (s3 >> 1) as u8,
        (s3 >> 9) as u8,
        ((s3 >> 17) | (s4 << 4)) as u8,
        (s4 >> 4) as u8,
        (s4 >> 12) as u8,
        ((s4 >> 20) | (s5 << 1)) as u8,
        (s5 >> 7) as u8,
        ((s5 >> 15) | (s6 << 6)) as u8,
        (s6 >> 2) as u8,
        (s6 >> 10) as u8,
        ((s6 >> 18) | (s7 << 3)) as u8,
        (s7 >> 5) as u8,
        (s7 >> 13) as u8,
        (s8 >> 0) as u8,
        (s8 >> 8) as u8,
        ((s8 >> 16) | (s9 << 5)) as u8,
        (s9 >> 3) as u8,
        (s9 >> 11) as u8,
        ((s9 >> 19) | (s10 << 2)) as u8,
        (s10 >> 6) as u8,
        ((s10 >> 14) | (s11 << 7)) as u8,
        (s11 >> 1) as u8,
        (s11 >> 9) as u8,
        (s11 >> 17) as u8,
    ]
}

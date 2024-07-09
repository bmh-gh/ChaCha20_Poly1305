struct ChaCha20 {
    state: [u32; 16],
}

impl ChaCha20 {
    fn new(state: [u32; 16]) -> Self {
        Self { state }
    }

    fn from(key: [u32; 8], nonce: [u32; 3]) -> Self {
        let state: [u32; 16] = [0; 16];

        Self { state }
    }

    // TODO: find solution to not use vec -> usable in embedded systems
    fn encrypt(self, plaintext: &[u8]) -> &[u8] {
        &[0_u8]
    }
}

fn block(state: [u32; 16]) -> [u32; 16] {
    let mut working_state = state.clone();
    (0..10).into_iter().for_each(|_| {
        // inner block
        quarter_round(&mut working_state, 0, 4, 8, 12);
        quarter_round(&mut working_state, 1, 5, 9, 13);
        quarter_round(&mut working_state, 2, 6, 10, 14);
        quarter_round(&mut working_state, 3, 7, 11, 15);

        quarter_round(&mut working_state, 0, 5, 10, 15);
        quarter_round(&mut working_state, 1, 6, 11, 12);
        quarter_round(&mut working_state, 2, 7, 8, 13);
        quarter_round(&mut working_state, 3, 4, 9, 14);
    });
    working_state
        .iter_mut()
        .zip(state)
        .for_each(|(a, b)| *a = a.wrapping_add(b));
    working_state
}

fn quarter_round(state: &mut [u32; 16], a_idx: usize, b_idx: usize, c_idx: usize, d_idx: usize) {
    let (mut a, mut b, mut c, mut d) = (state[a_idx], state[b_idx], state[c_idx], state[d_idx]);
    a = a.wrapping_add(b);
    d ^= a;
    d = d.rotate_left(16);

    c = c.wrapping_add(d);
    b ^= c;
    b = b.rotate_left(12);

    a = a.wrapping_add(b);
    d ^= a;
    d = d.rotate_left(8);

    c = c.wrapping_add(d);
    b ^= c;
    b = b.rotate_left(7);

    state[a_idx] = a;
    state[b_idx] = b;
    state[c_idx] = c;
    state[d_idx] = d;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_quarter_rounds() {
    //     let mut a = 0x11111111;
    //     let mut b = 0x01020304;
    //     let mut c = 0x9b8d6f43;
    //     let mut d = 0x01234567;

    //     quarter_round(&mut a, &mut b, &mut c, &mut d);

    //     assert_eq!(a, 0xea2a92f4);
    //     assert_eq!(b, 0xcb1cf8ce);
    //     assert_eq!(c, 0x4581472e);
    //     assert_eq!(d, 0x5881c4bb);
    // }

    #[test]
    fn test_quarter_round_state() {
        let mut chacha = ChaCha20::new([
            0x879531e0_u32,
            0xc5ecf37d_u32,
            0x516461b1_u32,
            0xc9a62f8a_u32,
            0x44c20ef3_u32,
            0x3390af7f_u32,
            0xd9fc690b_u32,
            0x2a5f714c_u32,
            0x53372767_u32,
            0xb00a5631_u32,
            0x974c541a_u32,
            0x359e9963_u32,
            0x5c971061_u32,
            0x3d631689_u32,
            0x2098d9d6_u32,
            0x91dbd320_u32,
        ]);
        quarter_round(&mut chacha.state, 2, 7, 8, 13);
        assert_eq!(
            chacha.state,
            [
                0x879531e0, 0xc5ecf37d, 0xbdb886dc, 0xc9a62f8a, 0x44c20ef3, 0x3390af7f, 0xd9fc690b,
                0xcfacafd2, 0xe46bea80, 0xb00a5631, 0x974c541a, 0x359e9963, 0x5c971061, 0xccc07c79,
                0x2098d9d6, 0x91dbd320
            ]
        )
    }

    #[test]
    fn test_block_function() {
        let mut chacha = ChaCha20::new([
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x03020100, 0x07060504, 0x0b0a0908,
            0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c, 0x00000001, 0x09000000,
            0x4a000000, 0x00000000,
        ]);
        let new_state = block(chacha.state);

        assert_eq!(
            new_state,
            [
                0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3, 0xc7f4d1c7, 0x0368c033, 0x9aaa2204,
                0x4e6cd4c3, 0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9, 0xd19c12b5, 0xb94e16de,
                0xe883d0cb, 0x4e3c50a2,
            ]
        )
    }
}

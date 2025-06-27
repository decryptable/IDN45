/// Performs the complex, irreversible permutation on the sponge's state.
///
/// The state is a 1600-bit (200-byte) array, which is treated as a 5x5 matrix
/// of 64-bit lanes for the purpose of these operations.
///
/// # Arguments
///
/// * `state` - A mutable reference to the 200-byte state array.
pub(crate) fn f_permutation(state: &mut [u8; 200]) {
    // Round constants for the Iota step.
    const ROUND_CONSTANTS: [u64; 24] = [
        0x0000000000000001, 0x0000000000008082, 0x800000000000808a, 0x8000000080008000,
        0x000000000000808b, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
        0x000000000000008a, 0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
        0x000000008000808b, 0x800000000000008b, 0x8000000000008089, 0x8000000000008003,
        0x8000000000008002, 0x8000000000000080, 0x000000000000800a, 0x800000008000000a,
        0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
    ];

    // Convert the byte state into a 5x5 matrix of 64-bit lanes for easier manipulation.
    let mut lanes = [[0u64; 5]; 5];
    for y in 0..5 {
        for x in 0..5 {
            let start = 8 * (5 * y + x);
            lanes[x][y] = u64::from_le_bytes(state[start..start + 8].try_into().unwrap());
        }
    }

    // Perform 24 rounds of permutation.
    for round_idx in 0..24 {
        // --- Theta Step ---
        let mut c = [0u64; 5];
        for x in 0..5 {
            c[x] = lanes[x][0] ^ lanes[x][1] ^ lanes[x][2] ^ lanes[x][3] ^ lanes[x][4];
        }
        let mut d = [0u64; 5];
        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }
        for y in 0..5 {
            for x in 0..5 {
                lanes[x][y] ^= d[x];
            }
        }

        // --- Rho and Pi Steps ---
        let mut temp_lanes = [[0u64; 5]; 5];
        for y in 0..5 {
            for x in 0..5 {
                let (new_x, new_y) = (y, (2 * x + 3 * y) % 5);
                let rotation = ((x as u32 * 11 + y as u32 * 23) % 64)
                             .wrapping_add(((round_idx as u32 + 1) * (round_idx as u32 + 2) / 2) % 64) % 64;
                temp_lanes[new_x][new_y] = lanes[x][y].rotate_left(rotation);
            }
        }
        lanes = temp_lanes;

        // --- Chi Step ---
        let old_lanes = lanes;
        for y in 0..5 {
            for x in 0..5 {
                lanes[x][y] = old_lanes[x][y] ^ (!old_lanes[(x + 1) % 5][y] & old_lanes[(x + 2) % 5][y]);
            }
        }

        // --- Iota Step ---
        lanes[0][0] ^= ROUND_CONSTANTS[round_idx];
    }

    // Convert the lanes back into the byte state.
    for y in 0..5 {
        for x in 0..5 {
            let start = 8 * (5 * y + x);
            state[start..start + 8].copy_from_slice(&lanes[x][y].to_le_bytes());
        }
    }
}

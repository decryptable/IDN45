use crate::permutation::f_permutation;

// --- Sponge Configuration Constants ---

/// The total width of the internal state in bits.
const STATE_WIDTH: usize = 1600;
/// The rate `r` in bits. This is the part of the state that is XORed with
/// input blocks and from which output is taken.
const RATE: usize = 1088;
/// The capacity `c` in bits. This is the part of the state that is shielded
/// from direct input/output, providing the security of the hash function.
// const CAPACITY: usize = STATE_WIDTH - RATE; // 512 bits

/// The rate in bytes.
pub(crate) const RATE_IN_BYTES: usize = RATE / 8; // 136 bytes

/// Core struct representing the sponge construction.
pub(crate) struct Sponge {
    /// The internal state of the sponge, a 200-byte (1600-bit) array.
    pub(crate) state: [u8; STATE_WIDTH / 8], // 200 bytes
    /// A buffer to hold leftover data that doesn't form a full block.
    buffer: Vec<u8>,
}

impl Sponge {
    /// Creates a new Sponge with a zero-initialized state.
    pub fn new() -> Self {
        Sponge {
            state: [0u8; STATE_WIDTH / 8],
            buffer: Vec::with_capacity(RATE_IN_BYTES),
        }
    }

    /// Absorbs input data into the sponge state.
    ///
    /// Data is processed in blocks of `RATE_IN_BYTES`. Any remaining data
    /// is kept in the internal buffer.
    pub fn absorb(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);

        while self.buffer.len() >= RATE_IN_BYTES {
            let block = &self.buffer[..RATE_IN_BYTES];
            for i in 0..RATE_IN_BYTES {
                self.state[i] ^= block[i];
            }
            f_permutation(&mut self.state);
            self.buffer.drain(..RATE_IN_BYTES);
        }
    }

    /// Finalizes the absorbing phase by padding the last block, and switches
    /// to the squeezing phase.
    pub fn finalize(&mut self) {
        // Simple Keccak-style padding: 10*1 padding scheme.
        // Append '0x06' for SHA-3 compatibility, followed by zeros, and a final '0x80'.
        self.buffer.push(0x06);
        self.buffer.resize(RATE_IN_BYTES - 1, 0);
        self.buffer.push(0x80);

        self.absorb(&self.buffer.clone()); // Absorb the final padded block
        self.buffer.clear();
    }

    /// Squeezes a specified number of bytes from the sponge.
    ///
    /// This can be called multiple times to get an output of arbitrary length.
    /// The state is permuted after each block of output is squeezed.
    pub fn squeeze(&mut self, out_len: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(out_len);
        while output.len() < out_len {
            let block = &self.state[..RATE_IN_BYTES];
            let take_len = (out_len - output.len()).min(RATE_IN_BYTES);
            output.extend_from_slice(&block[..take_len]);
            f_permutation(&mut self.state);
        }
        output
    }
}

// Default implementation to create a new Sponge easily.
impl Default for Sponge {
    fn default() -> Self {
        Self::new()
    }
}


use crate::sponge::Sponge;

/// Represents the output format for the hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashFormat {
    /// Standard 256-bit secure hash (64 hex characters).
    Standard,
    /// 96-bit hash formatted as `IDN45-xxxxxxxx-xxxxxxxx-xxxxxxxx`.
    Uuid,
    /// 44-bit hash formatted as a 12-character string with one dynamic colon.
    /// WARNING: This format is cryptographically insecure.
    Short,
}

/// The main IDN45 hasher.
///
/// It provides a simple interface to hash data using the underlying sponge construction.
pub struct IDN45 {
    sponge: Sponge,
    finalized: bool,
}

impl IDN45 {
    /// Creates a new `IDN45` hasher.
    /// Optionally accepts initial data and a salt.
    ///
    /// # Arguments
    ///
    /// * `data` - Optional initial data to hash. Can be `&str` or `&[u8]`.
    /// * `salt` - Optional salt to prepend to the data, enhancing security.
    pub fn new(data: Option<&[u8]>, salt: Option<&[u8]>) -> Self {
        let mut hasher = IDN45 {
            sponge: Sponge::new(),
            finalized: false,
        };
        if let Some(s) = salt {
            hasher.update(s);
        }
        if let Some(d) = data {
            hasher.update(d);
        }
        hasher
    }

    /// Updates the hash state with more data.
    ///
    /// Can be called multiple times. Panics if called after `hexdigest`.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to absorb.
    pub fn update(&mut self, data: &[u8]) {
        if self.finalized {
            panic!("Cannot update a finalized hasher.");
        }
        self.sponge.absorb(data);
    }

    /// Finalizes the hash computation and returns the result as a hex string.
    ///
    /// This method consumes the hasher. It can only be called once.
    ///
    /// # Arguments
    ///
    /// * `format_style` - The desired output format (`HashFormat`).
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted hash.
    pub fn hexdigest(mut self, format_style: HashFormat) -> String {
        if !self.finalized {
            self.sponge.finalize();
            self.finalized = true;
        }

        let (digest_len_bits, needs_formatting) = match format_style {
            HashFormat::Standard => (256, false),
            HashFormat::Uuid => (96, true),
            HashFormat::Short => (48, true), // 44 bits for body, 4 for position
        };
        
        let digest_bytes = self.sponge.squeeze(digest_len_bits / 8);
        let raw_hex = hex::encode(digest_bytes);

        if needs_formatting {
            match format_style {
                HashFormat::Uuid => {
                    let p1 = &raw_hex[0..8];
                    let p2 = &raw_hex[8..16];
                    let p3 = &raw_hex[16..24];
                    format!("IDN45-{}-{}-{}", p1, p2, p3)
                }
                HashFormat::Short => {
                    let hash_body = &raw_hex[..11];
                    let separator_val = u8::from_str_radix(&raw_hex[11..12], 16).unwrap_or(0);
                    // Cast to usize for slicing
                    let separator_pos = 1 + (separator_val % 10) as usize;
                    format!("{}:{}", &hash_body[..separator_pos], &hash_body[separator_pos..])
                }
                _ => unreachable!(),
            }
        } else {
            raw_hex
        }
    }
}

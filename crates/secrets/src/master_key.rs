use zeroize::{Zeroize, ZeroizeOnDrop};

/// The raw bytes of the application's master key.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MasterKey([u8; 32]);

impl MasterKey {
    /// Wraps a 32-byte key in a `MasterKey`.
    pub const fn new(v: [u8; 32]) -> Self {
        Self(v)
    }

    /// Borrows the underlying 32-byte key.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for MasterKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MasterKey(*****)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_does_not_show_key() {
        let v = std::iter::repeat_n(1, 32).collect::<Vec<_>>();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&v);
        let key = MasterKey::new(arr);

        let got = format!("{key:?}");

        assert_eq!(&got, "MasterKey(*****)");
    }
}

use crate::{ExecutionContext, Hash, Hasher};
use sha3::{Digest, Sha3_384};
/// A blake3 hasher implementation for the Valence protocol.
pub struct Sha3HasherSp1;

/// A blake3 execution environment for the Valence protocol.
pub struct Sha3Context;

impl Sha3HasherSp1 {
    /// Prefix for data hash.
    pub const DATA_PREFIX: &[u8] = &[0x00];

    /// Prefix for node hash.
    pub const MERGE_PREFIX: &[u8] = &[0x01];
}

impl Hasher for Sha3HasherSp1 {
    fn key(context: &str, data: &[u8]) -> Hash {
        let mut hasher = Sha3_384::new();
        hasher.update(context);

        hasher.update(data);
        hasher.finalize().as_slice().try_into().unwrap()
    }

    fn hash(data: &[u8]) -> Hash {
        let mut hasher = Sha3_384::new();
        hasher.update(Self::DATA_PREFIX);

        hasher.update(data);
        hasher.finalize().as_slice().try_into().unwrap()
    }

    fn merge(a: &Hash, b: &Hash) -> Hash {
        let mut hasher = Sha3_384::new();
        hasher.update(a);

        hasher.update(b);
        hasher.finalize().as_slice().try_into().unwrap()
    }
}

impl ExecutionContext for Sha3Context {
    type Hasher = Sha3HasherSp1;
}

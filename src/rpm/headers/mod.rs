mod header;
mod lead;
mod types;

pub use header::{FileCategory, FileDigest, FileDigestAlgorithm, FileEntry, FileMode, FileOwnership, Header};
pub(crate) use header::{IndexData, IndexEntry};
pub use lead::Lead;
pub use types::{Dependency, RPMFileEntry, RPMFileOptions, RPMFileOptionsBuilder};

#[cfg(feature = "signature-meta")]
mod signature_builder;
#[cfg(feature = "signature-meta")]
pub use signature_builder::{Empty, SignatureHeaderBuilder, WithDigest, WithSignature};

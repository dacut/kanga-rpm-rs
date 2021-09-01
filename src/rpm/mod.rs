mod builder;
mod compressor;
mod headers;
mod package;

#[cfg(feature = "signature-meta")]
pub mod signature;
pub use builder::RPMBuilder;
pub use compressor::Compressor;
pub use headers::{
    Dependency, FileCategory, FileDigest, FileDigestAlgorithm, FileEntry, FileMode, FileOwnership, Header, Lead,
    RPMFileEntry, RPMFileOptions, RPMFileOptionsBuilder,
};
pub(crate) use headers::{IndexData, IndexEntry};
pub use package::{RPMPackage, RPMPackageMetadata};

#[cfg(feature = "signature-meta")]
pub use headers::{Empty, SignatureHeaderBuilder, WithDigest, WithSignature};

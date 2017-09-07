#![unstable(feature = "steed", issue = "0")]
// NOTE(japaric) ^ unsure why this is needed ...

// Rust 1.16.0
pub mod ffi;
// Rust 1.16.0
pub mod fs;
// Rust 1.16.0
pub mod io;
// Rust 1.16.0 (no tests)
pub mod net;

#[stable(feature = "steed", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)] #[stable(feature = "steed", since = "1.0.0")]
    pub use super::io::{RawFd, AsRawFd, FromRawFd, IntoRawFd};
    #[doc(no_inline)] #[stable(feature = "steed", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)] #[stable(feature = "steed", since = "1.0.0")]
    pub use super::fs::{PermissionsExt, OpenOptionsExt, MetadataExt, FileTypeExt};
    #[doc(no_inline)] #[stable(feature = "steed", since = "1.0.0")]
    pub use super::fs::DirEntryExt;
    #[doc(no_inline)] #[unstable(feature = "file_offset", issue = "35918")]
    pub use super::fs::FileExt;
    /*
    #[doc(no_inline)] #[stable(feature = "steed", since = "1.0.0")]
    pub use super::thread::JoinHandleExt;
    #[doc(no_inline)] #[stable(feature = "steed", since = "1.0.0")]
    pub use super::process::{CommandExt, ExitStatusExt};
    */
}

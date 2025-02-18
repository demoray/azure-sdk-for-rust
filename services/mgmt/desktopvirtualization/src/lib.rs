#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(all(feature = "package-preview-2022-04", not(feature = "no-default-tag")))]
pub use package_preview_2022_04::*;
#[cfg(feature = "package-preview-2022-02")]
pub mod package_preview_2022_02;
#[cfg(all(feature = "package-preview-2022-02", not(feature = "no-default-tag")))]
pub use package_preview_2022_02::*;
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(all(feature = "package-preview-2021-09", not(feature = "no-default-tag")))]
pub use package_preview_2021_09::*;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "no-default-tag")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2021-07")]
pub mod package_2021_07;
#[cfg(all(feature = "package-2021-07", not(feature = "no-default-tag")))]
pub use package_2021_07::*;

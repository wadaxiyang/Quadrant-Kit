// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Build-time access to the Quadrant Kit Slint source facade.
#![forbid(unsafe_code)]
use std::path::PathBuf;
/// Name used by consumers in Slint imports: `@quadrant-kit`.
pub const SLINT_LIBRARY_NAME: &str = "quadrant-kit";
/// Returns the facade in this package's Cargo checkout, for build scripts only.
#[must_use]
pub fn slint_library_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("ui")
        .join("kit.slint")
}
#[cfg(test)]
mod tests {
    #[test]
    fn public_facade_is_in_the_package() {
        assert!(super::slint_library_path().is_file());
    }
}

// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Compiles the Gallery against the public source facade.
use std::{collections::HashMap, path::PathBuf};
fn main() {
    let manifest =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("Cargo manifest directory"));
    let entry = quadrant_kit::slint_library_path();
    assert!(entry.is_file(), "Kit facade is missing");
    let libraries = HashMap::from([(quadrant_kit::SLINT_LIBRARY_NAME.to_owned(), entry)]);
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent".into())
        .with_library_paths(libraries)
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config(manifest.join("ui/gallery.slint"), config)
        .expect("failed to compile Quadrant Kit Gallery");
}

// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Shared Gallery metadata; numeric pages are explicit legacy aliases only.
use std::{collections::HashSet, sync::LazyLock};

#[derive(Clone, Debug)]
pub struct CatalogEntry {
    pub id: &'static str,
    pub title: &'static str,
    pub parent: &'static str,
    pub depth: i32,
    pub icon: usize,
    pub alias: Option<i32>,
    pub component: &'static str,
    pub exports: &'static str,
    pub keywords: &'static str,
}
impl CatalogEntry {
    pub fn is_destination(&self) -> bool {
        !self.component.is_empty()
    }
}
pub static CATALOG: LazyLock<Vec<CatalogEntry>> =
    LazyLock::new(|| parse(include_str!("../catalog.tsv")).expect("valid Gallery catalog"));

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}

fn parse(source: &'static str) -> Result<Vec<CatalogEntry>, String> {
    let mut entries: Vec<CatalogEntry> = Vec::new();
    let mut ids = HashSet::new();
    let mut aliases = HashSet::new();
    let mut exports = HashSet::new();
    let mut files = HashSet::new();
    let mut components = HashSet::new();
    let mut ancestors: Vec<&str> = Vec::new();
    for line in source
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 9 {
            return Err("Catalog requires nine fields".into());
        }
        let [
            id,
            title,
            parent,
            icon,
            alias,
            component,
            file,
            visuals,
            keywords,
        ] = <[&str; 9]>::try_from(fields).map_err(|_| "Catalog fields")?;
        if !valid_id(id) || title.is_empty() || !ids.insert(id) {
            return Err(format!("Invalid/duplicate catalog ID: {id}"));
        }
        let depth = if parent.is_empty() {
            0
        } else {
            let owner = entries
                .iter()
                .find(|e| e.id == parent && !e.is_destination())
                .ok_or("Unknown catalog parent")?;
            owner.depth + 1
        };
        let index = usize::try_from(depth).map_err(|_| "Invalid depth")?;
        ancestors.truncate(index);
        if index > 2 || ancestors.len() != index || (index > 0 && ancestors[index - 1] != parent) {
            return Err("Non-preorder catalog".into());
        }
        ancestors.push(id);
        let icon = icon.parse::<usize>().map_err(|_| "Invalid icon")?;
        if icon >= 8 {
            return Err("Invalid icon".into());
        }
        let alias = (!alias.is_empty())
            .then(|| alias.parse::<i32>().map_err(|_| "Invalid alias"))
            .transpose()?;
        if let Some(alias) = alias
            && (!(0..8).contains(&alias) || !aliases.insert(alias) || component.is_empty())
        {
            return Err("Invalid/duplicate numeric alias".into());
        }
        if component.is_empty() {
            if !file.is_empty() || !visuals.is_empty() {
                return Err("Category cannot own a page or exports".into());
            }
        } else if file.is_empty()
            || file.contains('/')
            || file.contains('\\')
            || !file.ends_with("_page.slint")
            || !files.insert(file)
            || !components.insert(component)
        {
            return Err("Invalid/duplicate page implementation".into());
        }
        for name in visuals.split(',').filter(|name| !name.is_empty()) {
            if !exports.insert(name) {
                return Err("Duplicate visual component owner".into());
            }
        }
        entries.push(CatalogEntry {
            id,
            title,
            parent,
            depth,
            icon,
            alias,
            component,
            exports: visuals,
            keywords,
        });
    }
    if aliases.len() != 8 {
        return Err("All eight numeric aliases must be explicit".into());
    }
    for (i, entry) in entries.iter().enumerate() {
        if !entry.is_destination()
            && entries
                .get(i + 1)
                .is_none_or(|next| next.parent != entry.id)
        {
            return Err("Empty category".into());
        }
    }
    Ok(entries)
}
pub fn destination(id: &str) -> Option<&'static CatalogEntry> {
    CATALOG.iter().find(|e| e.id == id && e.is_destination())
}
pub fn resolve(page: Option<i32>, id: Option<&str>) -> Result<&'static str, String> {
    match (page, id) {
        (Some(_), Some(_)) => {
            Err("QUADRANT_GALLERY_PAGE and DESTINATION cannot be supplied together".into())
        }
        (Some(page), None) => CATALOG
            .iter()
            .find(|e| e.alias == Some(page))
            .map(|e| e.id)
            .ok_or_else(|| "QUADRANT_GALLERY_PAGE must be in 0..7".into()),
        (None, id) => destination(id.unwrap_or("home"))
            .map(|e| e.id)
            .ok_or_else(|| "Unknown QUADRANT_GALLERY_DESTINATION".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aliases_and_every_destination_resolve() {
        for (alias, id) in [
            "home",
            "tokens",
            "typography",
            "icons",
            "controls",
            "surfaces",
            "feedback",
            "navigation-view",
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(
                resolve(Some(i32::try_from(alias).unwrap()), None).unwrap(),
                *id
            );
        }
        for entry in CATALOG.iter().filter(|e| e.is_destination()) {
            assert_eq!(resolve(None, Some(entry.id)).unwrap(), entry.id);
            assert!(!entry.component.is_empty());
        }
        for id in ["", "missing", "controls-group"] {
            assert!(resolve(None, Some(id)).is_err());
        }
        assert!(resolve(Some(8), None).is_err());
        assert!(resolve(Some(0), Some("home")).is_err());
    }
    #[test]
    fn malformed_metadata_is_rejected() {
        let source = include_str!("../catalog.tsv");
        for changed in [
            source.replace("home\tHome", "all-components\tHome"),
            source.replace(
                "tokens\tTheme / Colors\tdesign-guidance",
                "tokens\tTheme / Colors\tmissing",
            ),
            source.replace("\t4\tControlsPage", "\t8\tControlsPage"),
            source.replace("\tIconButton\t", "\tFluentButton\t"),
        ] {
            assert!(parse(Box::leak(changed.into_boxed_str())).is_err());
        }
    }
}

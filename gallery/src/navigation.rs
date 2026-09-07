// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Gallery-only catalog, history, search and expansion controller.

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{DesignGalleryWindow, GalleryNavigation, NavigationEntry, NavigationEntryKind};

#[derive(Clone, Copy)]
struct CatalogEntry {
    id: &'static str,
    title: &'static str,
    parent: &'static str,
    depth: i32,
    page: Option<i32>,
    icon: usize,
    keywords: &'static str,
}

// Preorder, with the original snapshot page indices preserved. Aggregate pages
// remain real destinations until the later page-splitting phase.
const CATALOG: &[CatalogEntry] = &[
    CatalogEntry {
        id: "overview",
        title: "Overview",
        parent: "",
        depth: 0,
        page: Some(0),
        icon: 0,
        keywords: "getting started introduction guide",
    },
    CatalogEntry {
        id: "foundation",
        title: "Foundation",
        parent: "",
        depth: 0,
        page: None,
        icon: 1,
        keywords: "",
    },
    CatalogEntry {
        id: "tokens",
        title: "Tokens",
        parent: "foundation",
        depth: 1,
        page: Some(1),
        icon: 1,
        keywords: "foundation theme colors spacing motion",
    },
    CatalogEntry {
        id: "typography",
        title: "Typography",
        parent: "foundation",
        depth: 1,
        page: Some(2),
        icon: 2,
        keywords: "foundation font text type",
    },
    CatalogEntry {
        id: "icons",
        title: "Icons",
        parent: "foundation",
        depth: 1,
        page: Some(3),
        icon: 3,
        keywords: "foundation fluent svg symbols",
    },
    CatalogEntry {
        id: "components",
        title: "Components",
        parent: "",
        depth: 0,
        page: None,
        icon: 4,
        keywords: "",
    },
    CatalogEntry {
        id: "controls",
        title: "Controls",
        parent: "components",
        depth: 1,
        page: Some(4),
        icon: 4,
        keywords: "components button textfield input switch badge settings",
    },
    CatalogEntry {
        id: "surfaces-feedback",
        title: "Surfaces & feedback",
        parent: "components",
        depth: 1,
        page: None,
        icon: 5,
        keywords: "",
    },
    CatalogEntry {
        id: "surfaces",
        title: "Surfaces",
        parent: "surfaces-feedback",
        depth: 2,
        page: Some(5),
        icon: 5,
        keywords: "components surface card elevation",
    },
    CatalogEntry {
        id: "feedback",
        title: "Feedback",
        parent: "surfaces-feedback",
        depth: 2,
        page: Some(6),
        icon: 6,
        keywords: "components toast modal dialog confirmation",
    },
    CatalogEntry {
        id: "navigation",
        title: "Navigation",
        parent: "components",
        depth: 1,
        page: Some(7),
        icon: 7,
        keywords: "components navigationview back pane shell window",
    },
];

fn validate_catalog(catalog: &[CatalogEntry]) -> Result<(), String> {
    let mut ids = HashSet::new();
    let mut pages = HashSet::new();
    let mut ancestors: Vec<&CatalogEntry> = Vec::new();
    for entry in catalog {
        let depth = usize::try_from(entry.depth).map_err(|_| "Negative catalog depth")?;
        if depth > 2 || entry.id.is_empty() || entry.title.is_empty() || !ids.insert(entry.id) {
            return Err(format!("Invalid catalog entry: {}", entry.id));
        }
        ancestors.truncate(depth);
        if ancestors.len() != depth
            || (depth == 0 && !entry.parent.is_empty())
            || (depth > 0
                && (ancestors[depth - 1].id != entry.parent || ancestors[depth - 1].page.is_some()))
        {
            return Err(format!("Invalid catalog parent: {}", entry.id));
        }
        if let Some(page) = entry.page
            && (!(0..8).contains(&page) || !pages.insert(page))
        {
            return Err(format!("Invalid catalog page: {page}"));
        }
        if entry.icon >= 8 {
            return Err(format!("Invalid catalog icon: {}", entry.id));
        }
        ancestors.push(entry);
    }
    if pages.len() != 8 {
        return Err("Catalog must reach exactly the eight implemented pages".into());
    }
    for (index, entry) in catalog.iter().enumerate() {
        let has_children = catalog
            .get(index + 1)
            .is_some_and(|next| next.depth > entry.depth);
        if entry.page.is_none() && !has_children {
            return Err(format!("Empty catalog group: {}", entry.id));
        }
    }
    Ok(())
}

fn destination(id: &str) -> Option<&'static CatalogEntry> {
    CATALOG
        .iter()
        .find(|entry| entry.id == id && entry.page.is_some())
}

struct NavigationState {
    selected: &'static str,
    history: Vec<&'static str>,
    expanded: HashSet<&'static str>,
    query: String,
}

impl NavigationState {
    fn new(page: i32) -> Result<Self, String> {
        let selected = CATALOG
            .iter()
            .find(|entry| entry.page == Some(page))
            .ok_or_else(|| format!("Unknown Gallery page: {page}"))?
            .id;
        Ok(Self {
            selected,
            history: Vec::new(),
            expanded: CATALOG
                .iter()
                .filter(|entry| entry.page.is_none())
                .map(|entry| entry.id)
                .collect(),
            query: String::new(),
        })
    }

    fn results(&self) -> Vec<&'static CatalogEntry> {
        let query = self.query.trim().to_lowercase();
        CATALOG
            .iter()
            .filter(|entry| {
                entry.page.is_some()
                    && (entry.title.to_lowercase().contains(&query)
                        || entry.keywords.contains(&query))
            })
            .collect()
    }

    fn searching(&self) -> bool {
        !self.query.trim().is_empty()
    }

    fn reveal(&mut self, id: &'static str) {
        let mut parent = CATALOG
            .iter()
            .find(|entry| entry.id == id)
            .map_or("", |entry| entry.parent);
        while !parent.is_empty() {
            self.expanded.insert(parent);
            parent = CATALOG
                .iter()
                .find(|entry| entry.id == parent)
                .map_or("", |entry| entry.parent);
        }
    }

    fn navigate(&mut self, id: &str) {
        let Some(entry) = destination(id) else {
            return;
        };
        if entry.id != self.selected {
            self.history.push(self.selected);
            self.selected = entry.id;
        }
        // Filtering must not overwrite the normal tree's expansion state.
        if !self.searching() {
            self.reveal(entry.id);
        }
    }

    fn back(&mut self) {
        let Some(id) = self.history.pop() else {
            return;
        };
        self.selected = id;
        if self.searching() && !self.results().iter().any(|entry| entry.id == id) {
            self.query.clear();
        }
        self.reveal(id);
    }

    fn expand(&mut self, id: &str, expanded: bool) {
        if self.searching() {
            return;
        }
        let Some(entry) = CATALOG
            .iter()
            .find(|entry| entry.id == id && entry.page.is_none())
        else {
            return;
        };
        if expanded {
            self.expanded.insert(entry.id);
        } else {
            self.expanded.remove(entry.id);
        }
    }

    fn submit(&mut self, query: String) {
        self.query = query;
        if !self.searching() {
            return;
        }
        // Enter opens the first match in catalog order; typing never navigates.
        if let Some(entry) = self.results().first() {
            self.navigate(entry.id);
        }
    }

    fn rows(&self) -> Vec<NavigationEntry> {
        let searching = self.searching();
        let entries: Vec<_> = if searching {
            self.results()
        } else {
            CATALOG.iter().collect()
        };
        entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let has_children = !searching
                    && entries
                        .get(index + 1)
                        .is_some_and(|next| next.depth > entry.depth);
                NavigationEntry {
                    id: entry.id.into(),
                    text: entry.title.into(),
                    parent_id: if searching { "" } else { entry.parent }.into(),
                    depth: if searching { 0 } else { entry.depth },
                    kind: if entry.page.is_some() {
                        NavigationEntryKind::Destination
                    } else {
                        NavigationEntryKind::Group
                    },
                    enabled: true,
                    has_children,
                    expanded: !searching && self.expanded.contains(entry.id),
                    ..NavigationEntry::default()
                }
            })
            .collect()
    }
}

fn sync(window: &DesignGalleryWindow, state: &NavigationState) {
    let ui = window.global::<GalleryNavigation>();
    let icons = ui.get_icons();
    let selected_icons = ui.get_selected_icons();
    let mut rows = state.rows();
    for row in &mut rows {
        if let Some(entry) = CATALOG.iter().find(|entry| entry.id == row.id.as_str()) {
            row.icon = icons.row_data(entry.icon).unwrap_or_default();
            row.selected_icon = selected_icons.row_data(entry.icon).unwrap_or_default();
        }
    }
    ui.set_items(ModelRc::new(VecModel::from(rows)));
    ui.set_selected_id(state.selected.into());
    ui.set_page(
        destination(state.selected)
            .and_then(|entry| entry.page)
            .unwrap_or(0),
    );
    ui.set_can_go_back(!state.history.is_empty());
    ui.set_query(state.query.clone().into());
    let status = if state.searching() {
        let count = state.results().len();
        if count == 0 {
            format!("No results for \"{}\"", state.query.trim())
        } else {
            format!("{count} search result(s) · Enter opens the first match")
        }
    } else {
        String::new()
    };
    ui.set_search_status(status.into());
}

pub fn install(window: &DesignGalleryWindow, page: i32) -> Result<(), String> {
    validate_catalog(CATALOG)?;
    let state = Rc::new(RefCell::new(NavigationState::new(page)?));
    sync(window, &state.borrow());
    let update = {
        let weak = window.as_weak();
        move |action: &dyn Fn(&mut NavigationState)| {
            let mut state = state.borrow_mut();
            action(&mut state);
            if let Some(window) = weak.upgrade() {
                sync(&window, &state);
            }
        }
    };
    let handler = update.clone();
    window
        .global::<GalleryNavigation>()
        .on_navigate(move |id| handler(&|state| state.navigate(&id)));
    let handler = update.clone();
    window
        .global::<GalleryNavigation>()
        .on_back_requested(move || handler(&NavigationState::back));
    let handler = update.clone();
    window
        .global::<GalleryNavigation>()
        .on_query_edited(move |query| handler(&|state| state.query = query.to_string()));
    let handler = update.clone();
    window
        .global::<GalleryNavigation>()
        .on_search_submitted(move |query| handler(&|state| state.submit(query.to_string())));
    window
        .global::<GalleryNavigation>()
        .on_expansion_requested(move |id, expanded| update(&|state| state.expand(&id, expanded)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_and_startup_reach_only_real_pages() {
        validate_catalog(CATALOG).unwrap();
        for page in 0..8 {
            let state = NavigationState::new(page).unwrap();
            assert_eq!(destination(state.selected).unwrap().page, Some(page));
            assert!(state.history.is_empty());
        }
        assert!(NavigationState::new(8).is_err());
        let mut broken = CATALOG.to_vec();
        broken[2].parent = "missing";
        assert!(validate_catalog(&broken).is_err());
        broken = CATALOG.to_vec();
        broken[2].id = "overview";
        assert!(validate_catalog(&broken).is_err());
        broken = CATALOG.to_vec();
        broken[2].page = Some(8);
        assert!(validate_catalog(&broken).is_err());
        broken = CATALOG.to_vec();
        broken[2].depth = 3;
        assert!(validate_catalog(&broken).is_err());
    }

    #[test]
    fn forward_repeat_invalid_and_back_history() {
        let mut state = NavigationState::new(0).unwrap();
        state.back();
        state.navigate("overview");
        state.navigate("missing");
        state.navigate("foundation");
        assert!(state.history.is_empty());
        state.navigate("tokens");
        state.navigate("tokens");
        state.navigate("feedback");
        assert_eq!(state.history, ["overview", "tokens"]);
        state.expand("foundation", false);
        state.back();
        assert_eq!(state.selected, "tokens");
        assert!(state.expanded.contains("foundation"));
        state.back();
        state.back();
        assert_eq!(state.selected, "overview");
        assert!(state.history.is_empty());
    }

    #[test]
    fn case_insensitive_search_flat_results_and_expansion_restore() {
        let mut state = NavigationState::new(0).unwrap();
        state.expand("components", false);
        state.expand("surfaces-feedback", false);
        let expanded = state.expanded.clone();
        state.query = "  MODAL  ".into();
        assert_eq!(state.results()[0].id, "feedback");
        let rows = state.rows();
        assert!(
            rows.iter()
                .all(|row| row.depth == 0 && row.parent_id.is_empty() && !row.has_children)
        );
        state.navigate("feedback");
        state.expand("components", true);
        assert_eq!(state.expanded, expanded);
        state.query.clear();
        assert_eq!(state.expanded, expanded);
        assert_eq!(state.rows().len(), CATALOG.len());
        assert_eq!(state.history, ["overview"]);
    }

    #[test]
    fn submit_no_results_and_back_clear_only_when_needed() {
        let mut state = NavigationState::new(0).unwrap();
        state.query = "absent-result".into();
        assert!(state.rows().is_empty());
        state.submit(state.query.clone());
        assert_eq!(state.selected, "overview");
        assert!(state.history.is_empty());
        state.submit("components".into());
        assert_eq!(state.selected, "controls");
        state.navigate("feedback");
        state.back();
        assert_eq!(state.query, "components");
        assert_eq!(state.selected, "controls");
        state.query = "icons".into();
        state.back();
        assert_eq!(state.selected, "overview");
        assert!(state.query.is_empty());
        assert!(state.history.is_empty());
    }
}

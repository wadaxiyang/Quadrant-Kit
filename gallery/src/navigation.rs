// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Gallery-only catalog, history, search and expansion controller.

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{DesignGalleryWindow, GalleryNavigation, NavigationEntry, NavigationEntryKind};

use crate::catalog::{CATALOG, CatalogEntry, destination};

struct NavigationState {
    selected: &'static str,
    history: Vec<&'static str>,
    expanded: HashSet<&'static str>,
    query: String,
}

impl NavigationState {
    fn new(id: &str) -> Result<Self, String> {
        let selected = destination(id)
            .ok_or_else(|| format!("Unknown Gallery destination: {id}"))?
            .id;
        Ok(Self {
            selected,
            history: Vec::new(),
            expanded: CATALOG
                .iter()
                .filter(|entry| !entry.is_destination())
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
                entry.is_destination()
                    && (entry.title.to_lowercase().contains(&query)
                        || entry.keywords.contains(&query)
                        || entry.exports.to_lowercase().contains(&query))
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
            .find(|entry| entry.id == id && !entry.is_destination())
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
                    kind: if entry.is_destination() {
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
    // Settings stays reachable in the fixed footer even while filtering pages.
    rows.retain(|row| row.id != "settings");
    let settings = destination("settings").expect("Gallery settings destination");
    ui.set_footer_items(ModelRc::new(VecModel::from(vec![NavigationEntry {
        id: settings.id.into(),
        text: settings.title.into(),
        kind: NavigationEntryKind::Destination,
        enabled: true,
        icon: icons.row_data(settings.icon).unwrap_or_default(),
        selected_icon: selected_icons.row_data(settings.icon).unwrap_or_default(),
        ..NavigationEntry::default()
    }])));
    ui.set_items(ModelRc::new(VecModel::from(rows)));
    ui.set_selected_id(state.selected.into());
    ui.set_page_title(
        destination(state.selected)
            .expect("validated selection")
            .title
            .into(),
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

pub fn install(window: &DesignGalleryWindow, id: &str) -> Result<(), String> {
    let state = Rc::new(RefCell::new(NavigationState::new(id)?));
    let ui = window.global::<GalleryNavigation>();
    let components = CATALOG
        .iter()
        .flat_map(|entry| {
            entry
                .exports
                .split(',')
                .filter(|name| !name.is_empty())
                .map(move |name| crate::GalleryComponentLink {
                    destination: entry.id.into(),
                    name: name.into(),
                    category: CATALOG
                        .iter()
                        .find(|parent| parent.id == entry.parent)
                        .map_or("Design guidance", |parent| parent.title)
                        .into(),
                })
        })
        .collect::<Vec<_>>();
    ui.set_components(ModelRc::new(VecModel::from(components)));
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
        for entry in CATALOG.iter().filter(|entry| entry.is_destination()) {
            let state = NavigationState::new(entry.id).unwrap();
            assert_eq!(state.selected, entry.id);
            assert!(state.history.is_empty());
        }
        assert!(NavigationState::new("missing").is_err());
        assert!(NavigationState::new("design-guidance").is_err());
    }

    #[test]
    fn settings_is_searchable_and_returns_to_previous_page() {
        let mut state = NavigationState::new("home").unwrap();
        state.query = "settings".into();
        assert!(state.results().iter().any(|entry| entry.id == "settings"));
        // Enter opens the first catalog match; "settings" also matches component
        // examples, while "appearance" specifically finds the Gallery settings.
        state.submit("appearance".into());
        assert_eq!(state.selected, "settings");
        assert_eq!(state.history, ["home"]);
        state.navigate("settings");
        assert_eq!(state.history, ["home"]);
        state.back();
        assert_eq!(state.selected, "home");
        assert!(state.query.is_empty());
    }

    #[test]
    fn forward_repeat_invalid_and_back_history() {
        let mut state = NavigationState::new("home").unwrap();
        state.back();
        state.navigate("home");
        state.navigate("missing");
        state.navigate("design-guidance");
        assert!(state.history.is_empty());
        state.navigate("tokens");
        state.navigate("tokens");
        state.navigate("feedback");
        assert_eq!(state.history, ["home", "tokens"]);
        state.expand("design-guidance", false);
        state.back();
        assert_eq!(state.selected, "tokens");
        assert!(state.expanded.contains("design-guidance"));
        state.back();
        state.back();
        assert_eq!(state.selected, "home");
        assert!(state.history.is_empty());
    }

    #[test]
    fn case_insensitive_search_flat_results_and_expansion_restore() {
        let mut state = NavigationState::new("home").unwrap();
        state.expand("controls-group", false);
        state.expand("feedback-group", false);
        let expanded = state.expanded.clone();
        state.query = "  MODAL  ".into();
        assert_eq!(state.results()[0].id, "modal-manager");
        let rows = state.rows();
        assert!(
            rows.iter()
                .all(|row| row.depth == 0 && row.parent_id.is_empty() && !row.has_children)
        );
        state.navigate("feedback");
        state.expand("controls-group", true);
        assert_eq!(state.expanded, expanded);
        state.query.clear();
        assert_eq!(state.expanded, expanded);
        assert_eq!(state.rows().len(), CATALOG.len());
        assert_eq!(state.history, ["home"]);
    }

    #[test]
    fn submit_no_results_and_back_clear_only_when_needed() {
        let mut state = NavigationState::new("home").unwrap();
        state.query = "absent-result".into();
        assert!(state.rows().is_empty());
        state.submit(state.query.clone());
        assert_eq!(state.selected, "home");
        assert!(state.history.is_empty());
        state.submit("controls".into());
        assert_eq!(state.selected, "controls");
        state.navigate("feedback");
        state.back();
        assert_eq!(state.query, "controls");
        assert_eq!(state.selected, "controls");
        state.query = "icons".into();
        state.back();
        assert_eq!(state.selected, "home");
        assert!(state.query.is_empty());
        assert!(state.history.is_empty());
    }
}

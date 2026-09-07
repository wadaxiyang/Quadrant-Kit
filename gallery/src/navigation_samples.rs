// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Gallery-owned adversarial fixtures; no Kit runtime dependency or model repair.
use crate::{NavigationEntry, NavigationEntryKind};
use slint::Image;

fn row(id: &str, parent: &str, depth: i32, kind: NavigationEntryKind) -> NavigationEntry {
    NavigationEntry {
        id: id.into(),
        parent_id: parent.into(),
        depth,
        kind,
        text: id.into(),
        enabled: true,
        ..NavigationEntry::default()
    }
}

#[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
pub fn entries(
    case: i32,
    footer: bool,
    controls: bool,
    inputs: bool,
    settings: bool,
    icon: &Image,
    selected_icon: &Image,
) -> Vec<NavigationEntry> {
    use NavigationEntryKind::{Destination, DestinationGroup, Group, Header, Separator};
    if case == 16 {
        return vec![];
    }
    let mut rows = if footer {
        let mut parent = row("settings", "", 0, DestinationGroup);
        parent.has_children = true;
        parent.expanded = settings;
        vec![parent, row("about", "settings", 1, Destination)]
    } else {
        let mut a = row("controls", "", 0, Group);
        a.has_children = true;
        a.expanded = controls;
        let mut b = row("inputs", "controls", 1, DestinationGroup);
        b.has_children = true;
        b.expanded = inputs;
        let mut disabled = row("locked", "", 0, Group);
        disabled.enabled = false;
        disabled.has_children = true;
        disabled.expanded = true;
        let mut leaf = row("disabled", "", 0, Destination);
        leaf.enabled = false;
        let mut long = row("long", "", 0, Destination);
        long.text =
            "A deliberately long iconless destination label that must elide before the chevron"
                .into();
        vec![
            a,
            b,
            row("button", "inputs", 2, Destination),
            row("field", "inputs", 2, Destination),
            disabled,
            row("readme", "locked", 1, Destination),
            leaf,
            row("heading", "", 0, Header),
            row("divider", "", 0, Separator),
            long,
        ]
    };
    for r in &mut rows {
        if r.id != "long" {
            r.icon = icon.clone();
            r.selected_icon = selected_icon.clone();
        }
    }
    if footer {
        if case == 7 {
            rows[0].id = "controls".into();
            rows[1].parent_id = "controls".into();
        }
        if case == 11 {
            rows[0].parent_id = "controls".into();
            rows[0].depth = 1;
            rows[1].depth = 2;
        }
        return rows;
    }
    match case {
        1 => rows = vec![row("single", "", 0, Destination)],
        2 => {
            rows.truncate(2);
            rows[1].kind = Destination;
            rows[1].has_children = false;
            rows[1].expanded = false;
        }
        3 => rows[1].parent_id = "missing".into(),
        4 => rows[0].depth = -1,
        5 => rows[2].depth = 3,
        6 => rows[1].depth = 2,
        7 => rows[3].id = "button".into(),
        8 => rows[0].has_children = false,
        9 => rows[0].kind = Destination,
        10 => rows.swap(3, 4),
        12 => rows[0].parent_id = "inputs".into(),
        13 => rows[2].id = "".into(),
        14 => rows[0].kind = Header,
        15 => {
            rows = (0..257)
                .map(|n| row(&format!("entry-{n}"), "", 0, Destination))
                .collect();
        }
        _ => {}
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    // Independent stack-based host validation documents why each fixture is
    // malformed. Native Gallery checks test the actual Slint rejection policy.
    fn valid(primary: &[NavigationEntry], footer: &[NavigationEntry]) -> bool {
        let mut ids = HashSet::new();
        for model in [primary, footer] {
            if model.len() > 256 {
                return false;
            }
            let mut stack: Vec<&NavigationEntry> = vec![];
            for (index, e) in model.iter().enumerate() {
                if !(0..=2).contains(&e.depth) || e.id.is_empty() || !ids.insert(e.id.clone()) {
                    return false;
                }
                let depth = usize::try_from(e.depth).unwrap();
                stack.truncate(depth);
                if stack.len() != depth {
                    return false;
                }
                if depth == 0 {
                    if !e.parent_id.is_empty() {
                        return false;
                    }
                } else if stack[depth - 1].id != e.parent_id {
                    return false;
                }
                let child = model
                    .get(index + 1)
                    .is_some_and(|next| next.depth > e.depth);
                if child != e.has_children
                    || (child
                        && !matches!(
                            e.kind,
                            NavigationEntryKind::Group | NavigationEntryKind::DestinationGroup
                        ))
                {
                    return false;
                }
                stack.push(e);
            }
        }
        true
    }
    fn sample(case: i32, footer: bool) -> Vec<NavigationEntry> {
        entries(
            case,
            footer,
            true,
            true,
            true,
            &Image::default(),
            &Image::default(),
        )
    }
    #[test]
    fn valid_models_and_empty_regions() {
        for case in [0, 1, 2, 16] {
            assert!(
                valid(&sample(case, false), &sample(case, true)),
                "case {case}"
            );
        }
    }
    #[test]
    fn adversarial_models_cover_each_rejection() {
        for case in 3..=15 {
            assert!(
                !valid(&sample(case, false), &sample(case, true)),
                "case {case}"
            );
        }
    }
    #[test]
    fn exact_size_boundary_and_cross_region_ids() {
        let oversized = sample(15, false);
        assert!(valid(&oversized[..256], &[]));
        assert!(!valid(&oversized, &[]));
        let primary = sample(0, false);
        let mut footer = sample(0, true);
        footer[0].id = "controls".into();
        footer[1].parent_id = "controls".into();
        assert!(!valid(&primary, &footer));
        let mut separator_parent = primary;
        separator_parent[0].kind = NavigationEntryKind::Separator;
        assert!(!valid(&separator_parent, &[]));
    }
    #[test]
    fn collapse_preserves_complete_model_and_independent_child_state() {
        let expanded = sample(0, false);
        let collapsed = entries(
            0,
            false,
            false,
            false,
            false,
            &Image::default(),
            &Image::default(),
        );
        assert_eq!(expanded.len(), collapsed.len());
        assert_eq!(
            expanded.iter().map(|r| &r.id).collect::<Vec<_>>(),
            collapsed.iter().map(|r| &r.id).collect::<Vec<_>>()
        );
        assert!(!collapsed[0].expanded && !collapsed[1].expanded);
        assert!(!collapsed[4].enabled && collapsed[5].enabled);
        assert!(valid(&collapsed, &sample(0, true)));
    }
}

// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Validated snapshot configuration, parsed before creating a native window.
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum ThemeChoice {
    Light,
    Dark,
    System,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub size: Option<(f32, f32)>,
    pub theme: ThemeChoice,
    pub page: i32,
    pub preview: i32,
    pub navigation_case: i32,
    pub navigation_compact: bool,
    pub snapshot: Option<PathBuf>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Self::parse(|key| match std::env::var(key) {
            Ok(value) => Ok(Some(value)),
            Err(std::env::VarError::NotPresent) => Ok(None),
            Err(error) => Err(format!("{key}: {error}")),
        })
    }

    fn parse(mut read: impl FnMut(&str) -> Result<Option<String>, String>) -> Result<Self, String> {
        let width = read("QUADRANT_GALLERY_WIDTH")?;
        let height = read("QUADRANT_GALLERY_HEIGHT")?;
        let size = match (width, height) {
            (None, None) => None,
            (Some(width), Some(height)) => {
                Some((dimension(&width, "WIDTH")?, dimension(&height, "HEIGHT")?))
            }
            _ => return Err("QUADRANT_GALLERY_WIDTH and HEIGHT must be supplied together".into()),
        };
        let theme = match read("QUADRANT_GALLERY_THEME")?
            .unwrap_or_else(|| "light".into())
            .to_ascii_lowercase()
            .as_str()
        {
            "light" => ThemeChoice::Light,
            "dark" => ThemeChoice::Dark,
            "system" => ThemeChoice::System,
            _ => return Err("QUADRANT_GALLERY_THEME must be light, dark, or system".into()),
        };
        let page = index(read("QUADRANT_GALLERY_PAGE")?, "PAGE", 0, 7)?;
        let preview = index(read("QUADRANT_GALLERY_PREVIEW")?, "PREVIEW", 1, 2)?;
        let navigation_case = index(read("QUADRANT_GALLERY_NAV_CASE")?, "NAV_CASE", 0, 16)?;
        let navigation_compact =
            index(read("QUADRANT_GALLERY_NAV_COMPACT")?, "NAV_COMPACT", 0, 1)? == 1;
        let snapshot = read("QUADRANT_GALLERY_SNAPSHOT")?
            .map(|value| {
                if value.trim().is_empty() {
                    Err("QUADRANT_GALLERY_SNAPSHOT must not be empty".to_owned())
                } else {
                    Ok(PathBuf::from(value))
                }
            })
            .transpose()?;
        Ok(Self {
            size,
            theme,
            page,
            preview,
            navigation_case,
            navigation_compact,
            snapshot,
        })
    }
}

fn dimension(value: &str, name: &str) -> Result<f32, String> {
    value
        .parse::<f32>()
        .ok()
        .filter(|number| number.is_finite() && *number > 0.0)
        .ok_or_else(|| format!("QUADRANT_GALLERY_{name} must be finite and positive"))
}

fn index(value: Option<String>, name: &str, default: i32, maximum: i32) -> Result<i32, String> {
    match value {
        None => Ok(default),
        Some(value) => value
            .parse::<i32>()
            .ok()
            .filter(|number| (0..=maximum).contains(number))
            .ok_or_else(|| format!("QUADRANT_GALLERY_{name} must be in 0..{maximum}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(values: &[(&str, &str)]) -> Result<Config, String> {
        Config::parse(|key| {
            Ok(values
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| (*value).into()))
        })
    }
    #[test]
    fn defaults_and_complete_scenario() {
        assert_eq!(
            parse(&[]).unwrap(),
            Config {
                size: None,
                theme: ThemeChoice::Light,
                page: 0,
                preview: 1,
                navigation_case: 0,
                navigation_compact: false,
                snapshot: None
            }
        );
        let config = parse(&[
            ("QUADRANT_GALLERY_PAGE", "7"),
            ("QUADRANT_GALLERY_PREVIEW", "2"),
            ("QUADRANT_GALLERY_THEME", "SYSTEM"),
            ("QUADRANT_GALLERY_WIDTH", "900"),
            ("QUADRANT_GALLERY_HEIGHT", "600"),
            ("QUADRANT_GALLERY_SNAPSHOT", "image.png"),
        ])
        .unwrap();
        assert_eq!(config.size, Some((900.0, 600.0)));
        assert_eq!(config.theme, ThemeChoice::System);
        assert_eq!((config.page, config.preview), (7, 2));
    }
    #[test]
    fn rejects_removed_page_and_malformed_indices() {
        for value in ["8", "-1", "abc", "", "1.5"] {
            assert!(parse(&[("QUADRANT_GALLERY_PAGE", value)]).is_err());
        }
        assert!(parse(&[("QUADRANT_GALLERY_PREVIEW", "3")]).is_err());
        assert!(parse(&[("QUADRANT_GALLERY_NAV_CASE", "17")]).is_err());
        assert!(parse(&[("QUADRANT_GALLERY_NAV_COMPACT", "2")]).is_err());
        let navigation = parse(&[
            ("QUADRANT_GALLERY_NAV_CASE", "16"),
            ("QUADRANT_GALLERY_NAV_COMPACT", "1"),
        ])
        .unwrap();
        assert_eq!(navigation.navigation_case, 16);
        assert!(navigation.navigation_compact);
    }
    #[test]
    fn rejects_invalid_dimensions_and_partial_pair() {
        for value in ["NaN", "inf", "0", "-2", "oops"] {
            assert!(
                parse(&[
                    ("QUADRANT_GALLERY_WIDTH", value),
                    ("QUADRANT_GALLERY_HEIGHT", "800")
                ])
                .is_err()
            );
            assert!(
                parse(&[
                    ("QUADRANT_GALLERY_WIDTH", "800"),
                    ("QUADRANT_GALLERY_HEIGHT", value)
                ])
                .is_err()
            );
        }
        assert!(parse(&[("QUADRANT_GALLERY_WIDTH", "800")]).is_err());
    }
    #[test]
    fn rejects_unknown_theme_and_empty_output() {
        assert!(parse(&[("QUADRANT_GALLERY_THEME", "auto")]).is_err());
        assert!(parse(&[("QUADRANT_GALLERY_SNAPSHOT", " ")]).is_err());
    }
}

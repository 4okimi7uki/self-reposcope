use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct LangColor {
    color: Option<String>,
}

pub fn load_language_colors() -> HashMap<String, String> {
    let json_str = include_str!("../src/assets/languages_colors.json");
    let raw_map: HashMap<String, LangColor> =
        serde_json::from_str(json_str).expect("Failed to parse embedded JSON");

    raw_map
        .into_iter()
        .filter_map(|(lang, obj)| obj.color.map(|c| (lang, c)))
        .collect()
}

use std::collections::HashMap;
use std::fs;


pub fn load_language_colors() -> HashMap<String, String> {
    let json = fs::read_to_string("languages_colors.json").expect("Not found such file: languages_colors.json");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    parsed.as_object().unwrap().iter().filter_map(|(lang, props)| {
        props.get("color").and_then(|color| {
            color.as_str().map(|c| (lang.clone(), c.to_string()))
        })
    }).collect()
}
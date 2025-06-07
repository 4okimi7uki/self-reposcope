use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

pub fn generate_svg(
    lang_vec: &Vec<(String, u64)>,
    color_map: &HashMap<String, String>,
    output_path: &str
) -> std::io::Result<()> {
    let svg_width = 700;
    let bar_height = 12;
    let gap = 20;
    let label_margin = 10;
    let bar_start_x = 120;
    let right_padding = 20;
    let max_bar_width = svg_width - bar_start_x - right_padding;
    let max_bytes = lang_vec.first().map(|(_, b)| *b).unwrap_or(1);
    let svg_height = (bar_height + gap) * lang_vec.len() + gap;

    let mut file = File::create(output_path)?;
    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
    writeln!(
        file,
        r#"<svg width="{0}" height="{1}" xmlns="http://www.w3.org/2000/svg">"#,
        svg_width, svg_height
    )?;

    for (i, (lang, bytes)) in lang_vec.iter().enumerate() {
        let y = gap + i * (bar_height + gap);
        let bar_width = ((*bytes as f64) / (max_bytes as f64) * max_bar_width as f64) as u64;
        let color = color_map.get(lang).map(|s| s.as_str()).unwrap_or("#cccccc");
        let text_y = y + (bar_height / 2);

        // 言語ラベル
        writeln!(
            file,
            r#"<text x="{label_margin}" y="{y}" font-size="13" font-family="system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', sans-serif" fill='#333' alignment-baseline="hanging">{}</text>"#,
            lang
        )?;

        // バー
        writeln!(
            file,
            r#"<rect x="{bar_start_x}" y="{y}" width="{bar_width}" height="{bar_height}" fill="{color}" rx="5" ry="5">
    <animate attributeName="width" from="0" to="{bar_width}" dur="0.6s" fill="freeze" />
</rect>"#
        )?;

        // バイト数（右または内部）
        writeln!(
            file,
            r#"<text x="{}" y="{}" font-size="8" font-family="system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', sans-serif" alignment-baseline="middle" fill="{}">{}</text>"#,
            bar_start_x + bar_width + 5,
            text_y,
            color,
            bytes
        )?;
    }

    writeln!(file, "</svg>")?;
    Ok(())
}


pub fn generate_compact_svg(
    lang_vec: &Vec<(String, u64)>,
    color_map: &HashMap<String, String>,
    output_path: &str
) -> std::io::Result<()> {
    let svg_width = 400;
    let bar_height = 20;
    let padding_top = 40;
    let legend_line_height = 24;
    let legend_dot_radius = 6;

    let total_bytes: u64 = lang_vec.iter().map(|(_, b)| b).sum();
    let bar_y = padding_top + 10;
    let legend_start_y = bar_y + bar_height + 20;

    let svg_height = legend_start_y + lang_vec.len() as u32 * legend_line_height / 2 ;
    let mut file = File::create(output_path)?;
    let css = include_str!("animation.css");

    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(
        file,
        r#"<svg width="{svg_width}" height="{svg_height}" xmlns="http://www.w3.org/2000/svg">"#
    )?;
    writeln!(file, r#"<style>{}</style>"#, css).expect("Failed to write CSS");
    writeln!(file, r#"<rect x="0" y="0" width="{}" height="{}" fill="none" stroke='#ccc' stroke-width="1" rx="10" ry="10" />"#, svg_width - 1,svg_height - 1)?;

    // タイトル
    writeln!(
        file,
        r#"<text id="title" x="20" y="30" font-size="18" font-weight="bold" fill='#2563eb' font-family="system-ui, -apple-system, sans-serif">Most Used Languages</text>"#
    )?;
    
    // スタックバー
    let mut current_x = 20;
    let bar_total_width = svg_width - 40;
    
    writeln!(file, r#"
<defs>
    <clipPath id="roundedClip">
        <rect id="bar_back" x="{current_x}" y="{bar_y}" width="{bar_total_width}" height="10" rx="5" ry="5">
        </rect>
    </clipPath>
</defs>"#)?;
    writeln!(file, r#"<rect width="{bar_total_width}" height="10" x="{current_x}" y="{bar_y}" rx="5" ry="5" fill='#ccc' />"#)?;
    writeln!(file, r#"<g clip-path="url(#roundedClip)">"#)?;

    for (lang, bytes) in lang_vec {
        let percent = *bytes as f64 / total_bytes as f64;
        let bar_width = (percent * bar_total_width as f64).round() as u32;
        let color = color_map.get(lang).map(|s| s.as_str()).unwrap_or("#cccccc");

        writeln!(
            file, r#"    <rect x="{current_x}" y="{bar_y}" width="{bar_width}" height="10" fill="{color}" />"#
        )?;

        current_x += bar_width;
    }
    writeln!(file, r#"</g>"#)?;
    writeln!(file, r#"<g id="lang_legend">"#)?;


    // 凡例（2カラムで並べる）
    let legend_columns = 2;
    let column_width = svg_width / legend_columns - 10;
    for (i, (lang, bytes)) in lang_vec.iter().enumerate() {
        let percent = *bytes as f64 / total_bytes as f64 * 100.0;
        let col = i % legend_columns;
        let row = i / legend_columns;
        let legend_x = 20 + col * column_width;
        let legend_y = legend_start_y + (row as u32) * legend_line_height;

        let color = color_map.get(lang).map(|s| s.as_str()).unwrap_or("#cccccc");

        writeln!(
            file,
            r#"<circle cx="{x}" cy="{y}" r="{r}" fill="{color}" />
<text x="{}" y="{}" font-size="13" font-family="system-ui, -apple-system, sans-serif" fill='#333'>{lang} {percent:.2}%</text>"#,
            legend_x + legend_dot_radius + 20,
            legend_y + 4,
            x = legend_x + 10,
            y = legend_y,
            r = legend_dot_radius,
            color = color,
            lang = lang,
            percent = percent
        )?;
    }
    writeln!(file, r#"</g>"#)?;
    writeln!(file, "</svg>")?;
    Ok(())
}

use dotenv::dotenv;
use reqwest::Client;
use std::collections::HashMap;

mod get_language_color;
use get_language_color::load_language_colors;

mod draw_svg;
// use draw_svg::generate_svg;
use draw_svg::generate_compact_svg;

mod fetch_gh_api;
use clap::Parser;
use fetch_gh_api::{fetch_all_repos, get_username};

#[derive(Parser, Debug)]
#[command(name = "self-reposcope", about = "GitHub Language Stats Generator")]
struct Args {
    /// GitHub Token
    #[clap(short, long, env = "GITHUB_TOKEN")]
    token: String,

    /// Comma-separated string-list of languages to exclude
    #[clap(short, long, env = "EXCLUDED_LANGUAGES", default_value = "")]
    excluded_languages: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();
    let token = args.token;
    let excluded_languages: Vec<String> = args
        .excluded_languages
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let client = Client::new();
    let mut total_lang_map: HashMap<String, u64> = HashMap::new();
    let user_name = get_username(&token).await?;

    match fetch_all_repos(&token).await {
        Ok(repos) => {
            println!("Found {} repositories:", repos.len());
            for repo in repos {
                if repo.fork {
                    continue;
                }

                if repo.owner.owner_type == "Organization" {
                    continue; // Organization所有のリポジトリ除外
                }

                if repo.owner.login != user_name {
                    continue;
                }

                let owner = &repo.owner.login;
                let url = format!(
                    "https://api.github.com/repos/{}/{}/languages",
                    owner, repo.name
                );
                println!(" - {}", repo.name);

                let lang_res = client
                    .get(&url)
                    .bearer_auth(&token)
                    .header("User-Agent", "self-reposcope")
                    .send()
                    .await?;

                if !lang_res.status().is_success() {
                    eprintln!(
                        "Failed to fetch languages for {}: {}",
                        repo.name,
                        lang_res.status()
                    );
                    continue;
                }

                let lang_map: HashMap<String, u64> = lang_res.json().await?;

                let filtered_langs: HashMap<String, u64> = lang_map
                    .clone()
                    .into_iter()
                    .filter(|(lang, _)| {
                        !excluded_languages.iter().any(|excl| excl.eq_ignore_ascii_case(lang))
                    })
                    .collect();

                for (lang, bytes) in filtered_langs {
                    *total_lang_map.entry(lang).or_insert(0) += bytes;
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("------");

    let mut lang_vec: Vec<(String, u64)> = total_lang_map.into_iter().collect();
    lang_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n=== Aggregated Language Usage ===");
    for (i, (lang, bytes)) in lang_vec.iter().enumerate() {
        println!(" {}. {}: {} bytes", i + 1, lang, bytes);
    }

    let color_map = load_language_colors();
    // generate_svg(&lang_vec, &color_map, "output/language_chart.svg")?;
    generate_compact_svg(&lang_vec, &color_map, "output/full_languages.svg")?;

    Ok(())
}

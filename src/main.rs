use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

mod get_language_color;
use get_language_color::load_language_colors;

mod draw_svg;
use draw_svg::generate_svg;
use draw_svg::generate_compact_svg;


#[derive(Debug, Deserialize)]
struct Owner {
    login: String,
    #[serde(rename = "type")]
    owner_type: String, // "User" or "Organization"
}

#[derive(Debug, Deserialize)]
struct Repo {
    name: String,
    private: bool,
    fork: bool,
    language: Option<String>,
    owner: Owner,
}

pub async fn fetch_all_repos(token: &str) -> Result<Vec<Repo>, reqwest::Error> {
    let client = Client::new();
    let mut repos = vec![];
    let mut page = 1;

    loop {
        let url = format!(
            "https://api.github.com/user/repos?per_page=100&page={}",
            page
        );
        let res = client
            .get(&url)
            .header("User-Agent", "self-reposcope")
            .bearer_auth(token)
            .send()
            .await?;

        let batch: Vec<Repo> = res.json().await?;
        if batch.is_empty() {
            break;
        }

        repos.extend(batch);
        page += 1;
    }

    Ok(repos)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let TOKEN = env::var("GITHUB_TOKEN_KP").expect("Missing token");
    let client = Client::new();
    let lang_map: HashMap<String, u64>;
    let mut total_lang_map: HashMap<String, u64> = HashMap::new();

    match fetch_all_repos(&TOKEN).await {
        Ok(repos) => {
            println!("Found {} repositories:", repos.len());
            for repo in repos {
                if repo.fork {
                    continue;
                }

                if repo.owner.owner_type == "Organization" {
                    continue; // Organization所有のリポジトリも除外
                }

                let owner = &repo.owner.login;
                let url = format!(
                    "https://api.github.com/repos/{}/{}/languages",
                    owner, repo.name
                );
                println!(" - {}", repo.name);

                let lang_res = client
                    .get(&url)
                    .bearer_auth(&TOKEN)
                    .header("User-Agent", "self-reposcope")
                    .send()
                    .await?;

                if !lang_res.status().is_success() {
                    eprintln!(
                        "⚠️ Failed to fetch languages for {}: {}",
                        repo.name,
                        lang_res.status()
                    );
                    continue;
                }

                let lang_map: HashMap<String, u64> = lang_res.json().await?;

                for (lang, bytes) in lang_map {
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
    generate_svg(&lang_vec, &color_map, "output/language_chart.svg")?;
    generate_compact_svg(&lang_vec, &color_map, "output/full_languages.svg")?;

    Ok(())
}

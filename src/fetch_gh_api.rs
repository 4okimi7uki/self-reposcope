use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub login: String,
    #[serde(rename = "type")]
    pub owner_type: String, // "User" or "Organization"
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub private: bool,
    pub fork: bool,
    pub language: Option<String>,
    pub owner: Owner,
}

#[derive(Deserialize)]
pub struct User {
    pub login: String,
}

pub async fn get_username(token: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/user")
        .header("User-Agent", "self-reposcope")
        .bearer_auth(token)
        .send()
        .await?;

    let user: User = res.json().await?;
    Ok(user.login)
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
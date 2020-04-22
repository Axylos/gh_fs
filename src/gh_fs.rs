use reqwest;
use serde::Deserialize;
use std::fmt;
use vfs_service::SingleService;
extern crate dotenv;

use dotenv::dotenv;

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    id: u64,
    name: String,
    url: String,
    events_url: String,
    statuses_url: String,
    git_commits_url: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Github {
    pub repos: Vec<Repo>,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}\nUrl: Description: {:?},\nEvents Url: {}\nStatuses Url: {}",
            self.name, self.description, self.events_url, self.statuses_url
        )
    }
}

impl fmt::Display for Github {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self
            .repos
            .clone()
            .into_iter()
            .map(|repo| repo.to_string())
            .collect::<Vec<String>>()
            .join("\n\n");

        write!(f, "{}", data)
    }
}
pub struct GithubService {}

impl SingleService for GithubService {
    fn get_name(&self) -> String {
        "github_users".to_string()
    }

    fn fetch_data(&self, query: Option<&str>) -> Vec<String> {
        dotenv().ok();
        let username = match query {
            Some(q) => q,
            None => "torvalds",
        };

        let url = format!("https://api.github.com/users/{}/repos", username);

        let data: Vec<Repo> = reqwest::get(&url).unwrap().json().unwrap();

        let gh = Github { repos: data };

        vec![gh.to_string()]
    }
}

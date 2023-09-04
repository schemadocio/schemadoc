use anyhow::bail;
use serde::Deserialize;
use std::time::Duration;

use crate::models::ProjectKind;
use crate::web::auth::BasicAuth;

#[derive(Debug, Deserialize)]
struct ListProjectsResponse {
    pub result: Vec<ProjectBody>,
}

#[derive(Debug, Deserialize)]
struct ProjectBody {
    pub slug: String,
    pub kind: ProjectKind,
}

pub async fn schedule(host: &str, port: u16, interval: u64, force: bool) -> anyhow::Result<()> {
    // Wait until system startup
    // TODO: add healthcheck endpoint and use it here
    tokio::time::sleep(Duration::from_secs(15)).await;

    let auth = BasicAuth::user_pass()?;

    let mut interval = tokio::time::interval(Duration::from_secs(interval * 60));
    loop {
        interval.tick().await;

        if let Err(err) = pull(host, port, &auth, force).await {
            println!("Pull end: {:?}", err);
        }
    }
}

async fn pull(host: &str, port: u16, auth: &(String, String), force: bool) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{host}:{port}/api/v1/projects"))
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("Error receiving project data")
    }

    let projects: Vec<_> = response
        .json::<ListProjectsResponse>()
        .await?
        .result
        .into_iter()
        .filter(|p| p.kind.is_server())
        .collect();

    println!("Found {} projects to try to pull data from", projects.len());

    for project in projects {
        println!("Pulling datasource: {}", project.slug);

        let url = format!("http://{host}:{port}/api/v1/projects/{}/pull", project.slug);

        let response = client
            .post(url)
            .basic_auth(&auth.0, Some(&auth.1))
            .query(&[("force", force)])
            .send()
            .await;

        println!("content {:?}", &response);
    }

    Ok(())
}

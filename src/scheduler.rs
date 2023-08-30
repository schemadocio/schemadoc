use anyhow::bail;
use serde::Deserialize;
use std::time::Duration;

use crate::models::ProjectKind;

#[derive(Debug, Deserialize)]
pub struct ProjectBody {
    pub name: String,
    pub slug: String,
    pub kind: ProjectKind,
}

pub async fn schedule(host: &str, port: u16, interval: u64, force: bool) {
    // Wait until system startup
    // TODO: add healthcheck endpoint and use it here
    tokio::time::sleep(Duration::from_secs(15)).await;

    let mut interval = tokio::time::interval(Duration::from_secs(interval * 60));
    loop {
        interval.tick().await;

        let _ = pull(host, port, force).await;
    }
}

async fn pull(host: &str, port: u16, force: bool) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{host}:{port}/api/v1/projects"))
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("Error receiving project data")
    }

    let mut projects = response.json::<Vec<ProjectBody>>().await?;

    for project in projects.iter_mut() {
        if project.kind.is_client() {
            continue;
        }

        let url = format!("http://{host}:{port}/api/v1/projects/{}/pull", project.slug);

        let response = client.post(url).query(&[("force", force)]).send().await;

        println!("content {:?}", &response);
    }

    Ok(())
}

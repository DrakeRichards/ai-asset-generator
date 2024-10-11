//! Send image generation tasks to the queue.
//! Currently not implemented due to an issue with the agent-scheduler API.
//! See <https://github.com/ArtVentureX/sd-webui-agent-scheduler/issues/259>

/// Send a POST request to `/agent-scheduler/v1/queue/txt2img` to start a new image generation task.
/// The response contains the task_id for the image generation task.
pub async fn start_image_generation_task(
    base_url: &str,
    request_body: &RequestBody,
) -> Result<String> {
    let endpoint = "/agent-scheduler/v1/queue/txt2img";
    let url = format!("{}{}", base_url, endpoint);
    let body = serde_json::to_string(request_body)?;
    let client = reqwest::Client::new();
    let response = client.post(url).body(body).send().await?;
    let response: Value = serde_json::from_str(&response.text().await?)?;
    // The task_id is in the "task_id" field of the response.
    // If the response does not contain a "task_id" field, print what the response contains.
    let task_id = response["task_id"].as_str().ok_or(anyhow::anyhow!(format!(
        "Did not receive a task_id reponse. Response received: {:?}",
        response.as_str()
    )))?;
    Ok(task_id.to_string())
}

/// Check the status of the task.
async fn get_task_status(base_url: &str, task_id: &str) -> Result<String> {
    let endpoint = format!("/agent-scheduler/v1/task/{}", task_id);
    let url = format!("{}{}", base_url, endpoint);
    let response = reqwest::get(url).await?;
    let status: Value = serde_json::from_str(&response.text().await?)?;
    // The status is in the "msg" field of the response.
    let status: String = status["msg"]
        .as_str()
        .ok_or(anyhow::anyhow!("Unable to get task status."))?
        .to_string();
    dbg!(&status);
    Ok(status)
}

/// Get the results of the task. Results are a base64-encoded image.
async fn get_task_results(base_url: &str, task_id: &str) -> Result<String> {
    let endpoint = format!("/agent-scheduler/v1/task/{}/results", task_id);
    let url = format!("{}{}", base_url, endpoint);
    let response = reqwest::get(url).await?;
    let results: Value = serde_json::from_str(&response.text().await?)?;
    let image: String = results["image"]
        .as_str()
        .ok_or(anyhow::anyhow!("Unable to get task results."))?
        .to_string();
    Ok(image)
}

/// Poll the task until it is complete.
/// When the task is complete, the results are saved to a file.
pub async fn poll_task_to_file(
    base_url: &str,
    task_id: &str,
    output_path: &Path,
) -> Result<PathBuf> {
    loop {
        let status = get_task_status(base_url, task_id).await?;
        if status == "completed" {
            let image = get_task_results(base_url, task_id).await?;
            let image = base64::prelude::BASE64_STANDARD.decode(image)?;
            let image_path = output_path.join("image.png");
            std::fs::write(&image_path, image)?;
            return Ok(image_path);
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

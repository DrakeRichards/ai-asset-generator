//! Send image generation tasks to the queue.

use super::{txt2img::Txt2ImgRequestBody, Base64Image, ImageParams, StableDiffusionXLProvider};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Debug, Deserialize)]
struct QueueTaskResponse {
    task_id: TaskId,
}

type TaskId = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TaskStatus {
    Pending,
    Running,
    Done,
    Failed,
    Interrupted,
}

#[derive(Debug, Serialize)]
struct OverrideSettings {}

/// Request body for starting a new image generation task.
#[derive(Debug, Serialize)]
struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg_scale: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_settings: Option<OverrideSettings>,
    pub script_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vae: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

impl Default for RequestBody {
    fn default() -> Self {
        Self {
            prompt: None,
            negative_prompt: None,
            seed: None,
            batch_size: None,
            steps: None,
            width: None,
            height: None,
            sampler_name: None,
            cfg_scale: None,
            override_settings: None,
            script_name: "".to_string(),
            checkpoint: None,
            vae: None,
            callback_url: None,
        }
    }
}

impl RequestBody {
    pub fn from_params(params: &ImageParams) -> Self {
        Self {
            prompt: if params.prompt.to_string().is_empty() {
                None
            } else {
                Some(params.prompt.to_string())
            },
            negative_prompt: params.prompt.negative.clone(),
            seed: None,
            batch_size: Some(Number::from(1)),
            steps: Some(Number::from(params.steps)),
            width: Some(Number::from(params.width)),
            height: Some(Number::from(params.height)),
            sampler_name: Some(params.sampler_name.clone()),
            cfg_scale: Some(Number::from(params.cfg_scale)),
            override_settings: None,
            script_name: "".to_string(),
            checkpoint: params.model.clone(),
            vae: None,
            callback_url: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskStatusParams {
    checkpoint: Option<String>,
    images: Vec<String>,
    info: String,
    parameters: Txt2ImgRequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskStatusResponse {
    success: bool,
    data: TaskStatusData,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskStatusData {
    id: String,
    api_task_id: Option<String>,
    api_task_callback: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    request_type: String,
    status: TaskStatus,
    params: Value,
    priority: Number,
    position: Option<Number>,
    result: Option<String>,
    bookmarked: Option<bool>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskResults {
    success: bool,
    data: Vec<TaskResultsData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskResultsData {
    /// The image in base64 encoding.
    image: String,
    /// The generation parameters for the image.
    infotext: String,
}

impl StableDiffusionXLProvider {
    /// Send a POST request to `/agent-scheduler/v1/queue/txt2img` to start a new image generation task.
    /// The response contains the task_id for the image generation task.
    async fn start_image_generation_task(&self, request_body: &RequestBody) -> Result<TaskId> {
        let endpoint = "/agent-scheduler/v1/queue/txt2img";
        let url = format!("{}{}", self.get_url(), endpoint);
        let body = serde_json::to_string(request_body)?;
        let client = reqwest::Client::new();
        let response = client.post(url).body(body).send().await?;
        let response_text = response.text().await?;
        let response: QueueTaskResponse = serde_json::from_str(&response_text)?;
        Ok(response.task_id)
    }

    /// Check the status of the task.
    async fn get_task_status(&self, task_id: &TaskId) -> Result<TaskStatus> {
        let endpoint = format!("/agent-scheduler/v1/task/{}", task_id);
        let url = format!("{}{}", self.get_url(), endpoint);
        let response = reqwest::get(url).await?;
        let response_text = response.text().await?;
        let response: TaskStatusResponse = serde_json::from_str(&response_text)?;
        // The status is in the "msg" field of the response.
        let status = response.data.status;
        Ok(status)
    }

    /// Get the results of the task. Results are a base64-encoded image.
    async fn get_task_results(&self, task_id: &str) -> Result<String> {
        let endpoint = format!("/agent-scheduler/v1/task/{}/results", task_id);
        let url = format!("{}{}", self.get_url(), endpoint);
        let response = reqwest::get(url).await?;
        let response_text = response.text().await?;
        let results: TaskResults = serde_json::from_str(&response_text)?;
        let image = results
            .data
            .first()
            .ok_or(anyhow!(
                "No images returned. Result response: {:?}",
                response_text
            ))?
            .image
            .clone();
        // The image string is prefixed with "data:image/png;base64," which needs to be removed.
        let image = image
            .trim_start_matches("data:image/png;base64,")
            .to_string();
        Ok(image)
    }

    /// Poll the task until it is complete, returning the base64-encoded image.
    async fn poll_task(&self, task_id: &TaskId) -> Result<Base64Image> {
        let timeout = std::time::Duration::from_secs(300);
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let status = self.get_task_status(task_id).await?;
                    match status {
                        TaskStatus::Done => {
                            let image: Base64Image = Base64Image { image: self.get_task_results(task_id).await? };
                            return Ok(image);
                        }
                        TaskStatus::Failed => {
                            return Err(anyhow!("Task failed."));
                        }
                        _ => {}
                    }
                }
                _ = tokio::time::sleep(timeout) => {
                    return Err(anyhow!("Task timed out."));
                }
            }
        }
    }

    /// Add a txt2img task to the queue and wait for it to complete.
    pub async fn queue_txt2img(&self, params: &ImageParams) -> Result<Base64Image> {
        let request_body = RequestBody::from_params(params);
        let task_id = self.start_image_generation_task(&request_body).await?;
        let image = self.poll_task(&task_id).await?;
        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial(stable_diffusion, local_server)]
    async fn test_start_image_generation_task() -> Result<()> {
        let provider = StableDiffusionXLProvider::default();
        let request_body = RequestBody::default();
        let task_id = provider.start_image_generation_task(&request_body).await?;
        // Assert that we get a task_id.
        assert!(!task_id.is_empty());
        Ok(())
    }

    #[tokio::test]
    #[serial(stable_diffusion, local_server)]
    async fn test_poll_task() -> Result<()> {
        let provider = StableDiffusionXLProvider::default();
        let params = ImageParams::default();
        let request_body = RequestBody::from_params(&params);
        let task_id = provider.start_image_generation_task(&request_body).await?;
        let image = provider.poll_task(&task_id).await?;
        assert!(!image.image.is_empty());
        Ok(())
    }
}

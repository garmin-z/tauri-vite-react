use serde::Deserialize;

const GITHUB_URL: &str = "https://api.github.com/";

#[derive(Deserialize)]
struct ApiResponse {
    // 根据你的API响应结构体定义字段
    message: String,
}

struct ApiRequestParams {
    method: String,
    url: String,
    params: String,
    headers: String,
    body: String,
    timeout: u64,
    retry: u64,
    retry_delay: u64,
    retry_status_codes: Vec<u64>,
}
async fn request(param: ApiRequestParams) -> ApiResponse {
    let url = format!("{}{}", GITHUB_URL, param.url);
    let response = reqwest::get(url).await.expect("request ing");
    let api_response: ApiResponse = response.json().await.expect("request ing");

    return api_response;
}

#[tauri::command]
async fn github_octocat() -> Result<(), ApiResponse> {
    let q = request(ApiRequestParams {
        method: "GET".to_string(),
        url: "octocat".to_string(),
        params: "".to_string(),
        headers: "".to_string(),
        body: "".to_string(),
        timeout: 0,
        retry: 0,
        retry_delay: 0,
        retry_status_codes: vec![],
    })
    .await;

    // http::get(format!({}{}, GITHUB_URL, "octocat"))
    //     .await
    //     .map(|_| ())
    //     .map_err(|e| e.to_string())

    // Ok((q))
    Ok(())
}

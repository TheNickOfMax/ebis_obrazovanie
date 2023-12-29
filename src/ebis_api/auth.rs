use std::collections::HashMap;

pub async fn gos_login_token(login: &str, password: &str) -> Result<String, reqwest::Error> {
    let req_body = format!("{{\"login\":\"{login}\",\"password\":\"{password}\"}}");

    let cli: &reqwest::Client = &reqwest::Client::new();

    let req = cli
        .post("https://esia.gosuslugi.ru/aas/oauth2/api/login")
        .body(req_body)
        .send();

    let resp = req.await?.text().await?;

    let redirect_url = json::parse(&resp).unwrap()["redirect_url"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let token = &redirect_url.as_str()[redirect_url.find("code=").unwrap_or_default()
        ..redirect_url.find("&").unwrap_or_default()];

    Ok(resp.to_string())
}

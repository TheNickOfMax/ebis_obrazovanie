pub async fn gos_login_token(login: &str, password: &str) -> Result<String, reqwest::Error> {
    let req_body = format!("{{\"login\":\"{login}\",\"password\":\"{password}\"}}");

    let cli: &reqwest::Client = &reqwest::Client::new();

    let req = cli
        .post("https://esia.gosuslugi.ru/aas/oauth2/api/login")
        .body(req_body)
        .send();

    // I have no fucking idea why this shit doesnt return the url i want

    let resp = req.await?;

    let resp_text = resp.text().await?;

    let redirect_url = json::parse(&resp_text).unwrap()["redirect_url"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let token = &redirect_url.as_str()[redirect_url.find("code=").unwrap_or_default()
        ..redirect_url.find("&").unwrap_or_default()]; // for the good case, but i cant get it for some fucking reason

    Ok(resp_text.to_string())
}

use std::collections::HashMap;

pub async fn gos_login_token(login: &str, password: &str) -> Result<String, reqwest::Error> {
    let mut req_body: HashMap<&str, &str> = HashMap::new();
    req_body.insert("login", login);
    req_body.insert("password", password);

    let cli: &reqwest::Client = &reqwest::Client::new();

    let req = cli
        .post("https://esia.gosuslugi.ru/aas/oauth2/api/login")
        .json(&req_body)
        .send();
    let resp = req.await?.text().await?;

    let redirect_url = json::parse(&resp).unwrap()["redirect_url"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let token = &redirect_url.as_str()
        [redirect_url.find("code=").unwrap()..redirect_url.find("&").unwrap()];

    Ok(token.to_string())
}

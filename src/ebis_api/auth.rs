/// Logs into the provided gosuslugi acount and gets a user's auth code
pub async fn gos_login_code(login: &str, password: &str) -> Result<String, reqwest::Error> {
    let login_body = format!("{{\"login\":\"{login}\",\"password\":\"{password}\"}}");

    let cli: reqwest::Client = reqwest::Client::builder().cookie_store(true).build()?;

    // Initial request to dnevnik's login, which redirects to gosuslugi
    let req_dnev = cli
        .get("https://dnevnik.egov66.ru/api/auth/broker/esia/login?client_id=aiss2-diary")
        .send();
    let _resp_dnev = req_dnev.await?;

    // Request to the gosuslugi's login, which gives a redirect link back to dnevnik
    let req_gos = cli
        .post("https://esia.gosuslugi.ru/aas/oauth2/api/login")
        .body(login_body)
        .send();
    let resp_gos = req_gos.await?;

    // Parse the auth finishing redirect url
    let gos_json = json::parse(&resp_gos.text().await?).unwrap();
    let redir = gos_json["redirect_url"].as_str().unwrap();

    // Auth finishing request to the dnevnik
    let last = cli.get(redir).send();
    let last_resp = last.await?;

    // Code to get dnvnik's bearer token is inside the last url
    let code_url = last_resp.url().as_str().to_string();
    let code = &code_url[code_url.rfind("code=").unwrap() + 5..]
        .to_string()
        .replace("%3d", "=");

    Ok(code.to_string())
}

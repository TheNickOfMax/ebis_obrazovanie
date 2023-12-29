// https://dnevnik.egov66.ru/api/auth/swagger/index.html
// https://dnevnik.egov66.ru/api/auth/broker/esia/login?client_id=aiss2-diary

pub async fn gos_login_token(login: &str, password: &str) -> Result<String, reqwest::Error> {
    let login_body = format!("{{\"login\":\"{login}\",\"password\":\"{password}\"}}");

    let cli: reqwest::Client = reqwest::Client::builder().cookie_store(true).build()?;

    let req_d = cli
        .get("https://dnevnik.egov66.ru/api/auth/broker/esia/login?client_id=aiss2-diary")
        .send();
    let _resp_d = req_d.await?;

    let req_g = cli
        .post("https://esia.gosuslugi.ru/aas/oauth2/api/login")
        .body(login_body)
        .send();
    let resp_g = req_g.await?;

    Ok(resp_g.text().await?.to_string())
}

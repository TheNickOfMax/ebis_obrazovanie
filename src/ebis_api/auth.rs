use crate::ebis_lib::errors::ParseOrReqError;

use reqwest::Client;

use super::requests::bear_req;

/// Logs into the provided gosuslugi acount and gets a user's bearer roken for dnevnik
pub async fn gos_login(login: &str, password: &str) -> Result<String, ParseOrReqError> {
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
    let gos_json = json::parse(&resp_gos.text().await?)?;
    let redir = gos_json["redirect_url"]
        .as_str()
        .ok_or(json::Error::wrong_type("not found"))?;

    // Auth finishing request to the dnevnik
    let last = cli.get(redir).send();
    let last_resp = last.await?;

    // Code to get dnvnik's bearer token is inside the last url
    let code_url = last_resp.url().as_str().to_string();
    let code = &code_url[code_url
        .rfind("code=")
        .ok_or(json::Error::wrong_type("not found"))?
        + 5..]
        .to_string()
        .replace("%3d", "=");

    // Finish login process
    let _fin = cli.get(code_url).send().await?;

    Ok(bearer_from_code(cli, code).await?)
}

/// Gets the bearer token with the stupid code. Don't question it
pub async fn bearer_from_code(cli: Client, auth_code: &str) -> Result<String, ParseOrReqError> {
    let req_body = format!(
        "{{\"authorizationCode\":\"{auth_code}\",\"redirectUrl\":\"https://dnevnik.egov66.ru/\"}}"
    );

    let req = cli
        .post("https://dnevnik.egov66.ru/api/auth/Token")
        .body(req_body.clone())
        .header("Content-Type", "application/json")
        .send();
    let resp = req.await?;

    let resp_text = resp.text().await?;

    let resp_json = match json::parse(&resp_text) {
        Ok(jsn) => jsn,
        Err(err) => {
            println!("Error while getting ebis token");
            return Err(err.into());
        }
    };

    let token = match resp_json["accessToken"].as_str() {
        Some(t) => t.to_string(),
        None => {
            return Err(ParseOrReqError::ParsingError(json::Error::wrong_type(
                "str",
            )))
        }
    };

    Ok(token)
}

pub async fn revoke_token(token: &str) -> Result<String, ParseOrReqError> {
    let url = "https://dnevnik.egov66.ru/api/auth/Token/Revoke";

    Ok(bear_req(url, token).await?)
}

use crate::{
    ebis_api::credentials,
    ebis_lib::{diary::Discipline, errors::ParseOrReqError},
    json_utils::conversions::api_json_to_ebis_structs,
};
use reqwest::Method;

pub async fn bear_req(url: &str, token: &str) -> Result<String, reqwest::Error> {
    let cli: reqwest::Client = reqwest::Client::new();
    Ok(cli.get(url).bearer_auth(token).send().await?.text().await?)
}

pub async fn request_lessons_table(
    year_id: &str,
    class_id: &str,
    period_id: &str,
    student_id: &str,
    token: &str,
) -> Result<Vec<Discipline>, ParseOrReqError> {
    let y = year_id;
    let c = class_id;
    let p = period_id;
    let s = student_id;

    let url = format!("https://dnevnik.egov66.ru/api/estimate?schoolYear={y}&classId={c}&periodId={p}&subjectId=00000000-0000-0000-0000-000000000000&studentId={s}");

    let resp = match bear_req(&url, token).await {
        Ok(response) => response,
        Err(err) => return Err(ParseOrReqError::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(ParseOrReqError::ParsingError(err)),
    };

    Ok(api_json_to_ebis_structs(parsed))
}

pub async fn request_current_year_id(
    student_id: &str,
    token: &str,
) -> Result<String, ParseOrReqError> {
    let s = student_id;

    let url = format!("https://dnevnik.egov66.ru/api/estimate/years?studentId={s}");

    let resp = match bear_req(&url, token).await {
        Ok(response) => response,
        Err(err) => return Err(ParseOrReqError::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(ParseOrReqError::ParsingError(err)),
    };

    Ok(parsed["currentYear"]["id"]
        .as_str()
        .unwrap_or_default()
        .to_string())
}

//returns in format Vec<(name, id)>
pub async fn request_period_ids(
    student_id: &str,
    year_id: &str,
    class_id: &str,
    token: &str,
) -> Result<Vec<(String, String)>, ParseOrReqError> {
    let s = student_id;
    let y = year_id;
    let c = class_id;

    let url = format!(
        "https://dnevnik.egov66.ru/api/estimate/periods?schoolYear={y}&classId={c}&studentId={s}"
    );

    let resp = match bear_req(&url, token).await {
        Ok(response) => response,
        Err(err) => return Err(ParseOrReqError::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(ParseOrReqError::ParsingError(err)),
    };

    Ok(parsed["periods"]
        .members()
        .map(|p| {
            (
                p["name"].as_str().unwrap_or_default().to_string(),
                p["id"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect())
}

pub async fn request_current_calss_id(
    student_id: &str,
    year_id: &str,
    token: &str,
) -> Result<String, ParseOrReqError> {
    let s = student_id;
    let y = year_id;

    let url = format!("https://dnevnik.egov66.ru/api/classes?studentId={s}&schoolYear={y}");

    let resp = match bear_req(&url, token).await {
        Ok(response) => response,
        Err(err) => return Err(ParseOrReqError::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(ParseOrReqError::ParsingError(err)),
    };

    let json_value = match parsed["currentClass"]["value"].as_str() {
        Some(parsed_value) => parsed_value,
        None => {
            return Err(ParseOrReqError::ParsingError(json::Error::WrongType(
                "None".to_string(),
            )))
        }
    };
    Ok(json_value.to_string())
}

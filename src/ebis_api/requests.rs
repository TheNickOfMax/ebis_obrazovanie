use crate::{
    ebis_api::credentials,
    ebis_lib::diary::{Discipline, Error},
    json_utils::conversions::api_json_to_ebis_structs,
};
use reqwest::Method;

pub async fn req(url: &str, cookie: &str) -> Result<String, reqwest::Error> {
    let cli: reqwest::Client = reqwest::Client::new();
    Ok(cli
        .request(Method::GET, url)
        .header("Cookie", cookie)
        .send()
        .await?
        .text()
        .await?)
}

pub async fn request_lessons_table(
    year_id: &str,
    class_id: &str,
    period_id: &str,
    student_id: &str,
) -> Result<Vec<Discipline>, Error> {
    let y = year_id;
    let c = class_id;
    let p = period_id;
    let s = student_id;

    let url = format!("https://dnevnik.egov66.ru/api/estimate?schoolYear={y}&classId={c}&periodId={p}&subjectId=00000000-0000-0000-0000-000000000000&studentId={s}");

    let resp = match req(&url, credentials::COOKIE).await {
        Ok(response) => response,
        Err(err) => return Err(Error::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(Error::ParsingError(err)),
    };

    Ok(api_json_to_ebis_structs(parsed))
}

pub async fn request_current_year_id(student_id: &str) -> Result<String, Error> {
    let s = student_id;

    let url = format!("https://dnevnik.egov66.ru/api/estimate/years?studentId={s}");

    let resp = match req(&url, credentials::COOKIE).await {
        Ok(response) => response,
        Err(err) => return Err(Error::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(Error::ParsingError(err)),
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
) -> Result<Vec<(String, String)>, Error> {
    let s = student_id;
    let y = year_id;
    let c = class_id;

    let url = format!(
        "https://dnevnik.egov66.ru/api/estimate/periods?schoolYear={y}&classId={c}&studentId={s}"
    );

    let resp = match req(&url, credentials::COOKIE).await {
        Ok(response) => response,
        Err(err) => return Err(Error::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(Error::ParsingError(err)),
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

pub async fn request_current_calss_id(student_id: &str, year_id: &str) -> Result<String, Error> {
    let s = student_id;
    let y = year_id;

    let url = format!("https://dnevnik.egov66.ru/api/classes?studentId={s}&schoolYear={y}");

    let resp = match req(&url, credentials::COOKIE).await {
        Ok(response) => response,
        Err(err) => return Err(Error::ReqError(err.without_url())),
    };

    let parsed = match json::parse(&resp) {
        Ok(parsed_json) => parsed_json,
        Err(err) => return Err(Error::ParsingError(err)),
    };

    let json_value = match parsed["currentClass"]["value"].as_str() {
        Some(parsed_value) => parsed_value,
        None => {
            return Err(Error::ParsingError(json::Error::WrongType(
                "None".to_string(),
            )))
        }
    };
    Ok(json_value.to_string())
}

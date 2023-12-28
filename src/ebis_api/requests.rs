use crate::{
    ebis_api::credentials,
    ebis_lib::diary::Discipline,
    json_utils::{conversions::api_json_to_ebis_structs, from_json_trait::FromJson},
};

use json::JsonValue;
use reqwest::Method;

use super::credentials::COOKIE;

// all of this is really fucked up and badly needs refactoring

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
) -> Result<Vec<Discipline>, reqwest::Error> {
    let y = year_id;
    let c = class_id;
    let p = period_id;
    let s = student_id;
    let url = format!("https://dnevnik.egov66.ru/api/estimate?schoolYear={y}&classId={c}&periodId={p}&subjectId=00000000-0000-0000-0000-000000000000&studentId={s}");
    Ok(api_json_to_ebis_structs(
        json::parse(req(&url, credentials::COOKIE).await?.as_str()).unwrap(),
    ))
}

pub async fn request_current_year_id(student_id: &str) -> Result<String, reqwest::Error> {
    let s = student_id;

    let url = format!("https://dnevnik.egov66.ru/api/estimate/years?studentId={s}");

    // This shit is unsafe as fuck
    Ok(
        json::parse(req(&url, credentials::COOKIE).await?.as_str()).unwrap()["currentYear"]["id"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    )
}

//returns in format Vec<(name, id)>
pub async fn request_period_ids(
    student_id: &str,
    year_id: &str,
    class_id: &str,
) -> Result<Vec<(String, String)>, reqwest::Error> {
    let s = student_id;
    let y = year_id;
    let c = class_id;
    let url = format!(
        "https://dnevnik.egov66.ru/api/estimate/periods?schoolYear={y}&classId={c}&studentId={s}"
    );
    let resp = json::parse(&req(&url, COOKIE).await?).unwrap();
    Ok(Vec::<JsonValue>::from_json_array(resp["periods"].clone())
        .iter()
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
) -> Result<String, reqwest::Error> {
    let s = student_id;
    let y = year_id;

    let url = format!("https://dnevnik.egov66.ru/api/classes?studentId={s}&schoolYear={y}");

    // This shit is unsafe as fuck
    Ok(
        json::parse(req(&url, credentials::COOKIE).await?.as_str()).unwrap()["currentClass"]
            ["value"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    )
}

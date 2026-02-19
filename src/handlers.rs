use crate::math::parse_numbers;
use anyhow::Result;
use axum::extract::RawQuery;
use axum::http::StatusCode;

pub(crate) async fn add(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let result: i64 = parse_numbers(query)?.into_iter().sum();

    Ok(result.to_string())
}

pub(crate) async fn subtract(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let result: i64 = parse_numbers(query)?
        .into_iter()
        .reduce(|acc, x| acc - x)
        .ok_or(StatusCode::BAD_REQUEST)?;

    Ok(result.to_string())
}

pub(crate) async fn multiply(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let result: i64 = parse_numbers(query)?
        .into_iter()
        .reduce(|acc, x| acc * x)
        .ok_or(StatusCode::BAD_REQUEST)?;

    Ok(result.to_string())
}

pub(crate) async fn divide(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let result: i64 = parse_numbers(query)?
        .into_iter()
        .reduce(|acc, x| acc / x)
        .ok_or(StatusCode::BAD_REQUEST)?;

    Ok(result.to_string())
}
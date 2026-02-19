use crate::math::parse_numbers;
use anyhow::Result;
use axum::extract::RawQuery;
use axum::http::StatusCode;

pub(crate) async fn add(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let result: i64 = parse_numbers(query)?
        .try_fold(0i64, |acc, x| acc.checked_add(x))
        .ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(result.to_string())
}

pub(crate) async fn subtract(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let result: i64 = parse_numbers(query)?
        .try_fold(0i64, |acc, x| acc.checked_sub(x))
        .ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(result.to_string())
}

pub(crate) async fn multiply(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let mut numbers = parse_numbers(query)?;
    let first = numbers.next().ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;

    let result: i64 = numbers
        .try_fold(first, |acc, x| acc.checked_mul(x))
        .ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(result.to_string())
}

pub(crate) async fn divide(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let mut numbers = parse_numbers(query)?;
    let first = numbers.next().ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;

    let result: i64 = numbers
        .try_fold(first, |acc, x| acc.checked_div(x))
        .ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(result.to_string())
}

use anyhow::Result;
use axum::http::StatusCode;
use std::vec::IntoIter;

pub(crate) fn parse_numbers(query: Option<String>) -> Result<IntoIter<i64>, StatusCode> {
    let query = query.unwrap_or_default();

    let numbers: Result<Vec<i64>, _> = query.split(',').map(|s| s.parse::<i64>()).collect();

    let nums = numbers.map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(nums.into_iter())
}

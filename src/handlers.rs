use axum::extract::RawQuery;
use axum::http::StatusCode;

pub(crate) async fn plus(RawQuery(query): RawQuery) -> anyhow::Result<String, StatusCode> {
    let query = query.unwrap_or_default();

    let numbers: anyhow::Result<Vec<i64>, _> = query
        .split(',')
        .map(|s| s.parse::<i64>())
        .collect();

    let nums = numbers.map_err(|_| StatusCode::BAD_REQUEST)?;

    let result: i64 = nums
        .into_iter()
        .sum();

    Ok(result.to_string())
}

pub(crate) async fn minus(RawQuery(query): RawQuery) -> anyhow::Result<String, StatusCode> {
    let query = query.unwrap_or_default();

    let numbers: anyhow::Result<Vec<i64>, _> = query
        .split(',')
        .map(|s| s.parse::<i64>())
        .collect();

    let nums = numbers.map_err(|_| StatusCode::BAD_REQUEST)?;

    let result: i64 = nums
        .into_iter()
        .reduce(|acc, x| acc - x)
        .ok_or(StatusCode::BAD_REQUEST)?;

    Ok(result.to_string())
}
use std::fmt;

use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::{Response, Status};
use tracing::info;

use crate::{
    pb::{QueryRequest, RawQueryRequest, User},
    ResponseStream, ServiceResult, UserStatsService,
};

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        let sql = query.to_string();
        self.raw_query(RawQueryRequest { query: sql }).await
    }

    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        let Ok(ret) = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.inner.pool)
            .await
        else {
            return Err(Status::internal(format!("{}", req.query)));
        };

        let stream = futures::stream::iter(ret.into_iter().map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }
}

impl fmt::Display for QueryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sql = "SELECT email, name FROM user_stats WHERE ".to_string();

        let timeconditions = self
            .timestamps
            .iter()
            .map(|(k, v)| timestamp_query(k, v.lower.as_ref(), v.upper.as_ref()))
            .join(" AND ");

        sql.push_str(&timeconditions);

        let ids_cond = self
            .ids
            .iter()
            .map(|(k, v)| ids_query(k, &v.ids))
            .join(" AND ");

        if !ids_cond.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&ids_cond);
        }

        info!("generated SQL: {} for: {:?}", sql, self);
        write!(f, "{}", sql)
    }
}

fn timestamp_query(name: &str, lower: Option<&Timestamp>, upper: Option<&Timestamp>) -> String {
    if lower.is_none() && upper.is_none() {
        return "TRUE".to_string();
    }

    if lower.is_none() {
        return format!(
            "{} <= {}",
            name,
            timestamp_to_utc(upper.unwrap()).to_rfc3339()
        );
    }

    if upper.is_none() {
        return format!(
            "{} >= {}",
            name,
            timestamp_to_utc(lower.unwrap()).to_rfc3339()
        );
    }

    format!(
        "{} BETWEEN '{}' AND '{}'",
        name,
        timestamp_to_utc(lower.unwrap()).to_rfc3339(),
        timestamp_to_utc(upper.unwrap()).to_rfc3339(),
    )
}

fn timestamp_to_utc(ts: &Timestamp) -> DateTime<Utc> {
    let secs = ts.seconds;
    let nanos = ts.nanos as u32;
    chrono::Utc.timestamp_opt(secs, nanos).unwrap()
}

fn ids_query(name: &str, ids: &[u32]) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }

    format!("array{:?} <@ {}", ids, name)
}

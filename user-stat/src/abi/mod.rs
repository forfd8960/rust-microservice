use std::fmt;

// use itertools::Itertools;
use tonic::{Response, Status};

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
        write!(f, "SELECT email, name FROM user_stats WHERE ")
    }
}

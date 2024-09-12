pub mod abi;
pub mod pb;

use futures::Stream;
use pb::{user_stats_server::UserStats, QueryRequest, RawQueryRequest, User};
use sqlx::PgPool;
use std::{ops::Deref, pin::Pin, sync::Arc};
use tonic::{async_trait, Request, Response, Status};

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[derive(Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}

pub struct UserStatsServiceInner {
    pool: PgPool,
}

#[async_trait]
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    type RawQueryStream = ResponseStream;

    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        todo!()
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        todo!()
    }
}

impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

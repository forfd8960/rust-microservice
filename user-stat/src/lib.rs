pub mod abi;
pub mod pb;

use futures::Stream;
use pb::{
    user_stats_server::{UserStats, UserStatsServer},
    GreetRequest, GreetResponse, QueryRequest, RawQueryRequest, User,
};
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
        let q = request.into_inner();
        self.query(q).await
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        let q = request.into_inner();
        self.raw_query(q).await
    }

    async fn greet(&self, req: Request<GreetRequest>) -> Result<Response<GreetResponse>, Status> {
        Ok(Response::new(GreetResponse {
            msg: format!("Hello {}", req.into_inner().msg),
        }))
    }
}

impl UserStatsService {
    pub async fn new() -> Self {
        let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/user_stats")
            .await
            .expect("Failed to connect to db");
        Self {
            inner: Arc::new(UserStatsServiceInner { pool }),
        }
    }

    pub fn into_server(self) -> UserStatsServer<Self> {
        UserStatsServer::new(self)
    }
}

impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

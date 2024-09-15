use std::{sync::Arc, time::Duration};

use chrono::Utc;
use futures::{Stream, StreamExt};
use prost_types::Timestamp;
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    pb::{notification_server::NotificationServer, send_request::Msg, SendRequest, SendResponse},
    NotificationService, NotificationServiceInner, ResponseStream, ServiceResult,
};

pub trait Sender {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status>;
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(NotificationServiceInner {
                sender: dummy_send(),
            }),
        }
    }

    pub fn innto_server(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }

    pub async fn send(
        &self,
        mut stream: impl Stream<Item = Result<SendRequest, Status>> + Send + 'static + Unpin,
    ) -> ServiceResult<ResponseStream> {
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let res = match req.msg {
                    Some(v) => {
                        info!("received msg: {:?}", v);

                        Ok(SendResponse {
                            message_id: Uuid::new_v4().to_string(),
                            timestamp: Some(to_ts()),
                        })
                    }
                    None => {
                        warn!("invalid request");
                        Err(Status::invalid_argument("invalid request"))
                    }
                };

                tx.send(res).await.unwrap();
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}

fn dummy_send() -> mpsc::Sender<Msg> {
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            info!("sending msg: {:?}", msg);
            sleep(Duration::from_millis(100)).await;
        }
    });

    tx
}

fn to_ts() -> Timestamp {
    let now = Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

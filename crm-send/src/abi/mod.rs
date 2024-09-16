pub mod email;
pub mod in_app;
pub mod sms;

use crate::{
    pb::{notification_server::NotificationServer, send_request::Msg, SendRequest, SendResponse},
    NotificationService, NotificationServiceInner, ResponseStream, ServiceResult,
};
use chrono::Utc;
use futures::{Stream, StreamExt};
use prost_types::Timestamp;
use std::{sync::Arc, time::Duration};
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::{info, warn};

const CHANNEL_SIZE: usize = 1000;

pub trait Sender {
    fn send(
        self,
        svc: NotificationService,
    ) -> impl std::future::Future<Output = Result<SendResponse, Status>> + Send;
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(NotificationServiceInner {
                sender: dummy_send(),
            }),
        }
    }

    pub fn into_server(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }

    pub async fn send(
        &self,
        mut stream: impl Stream<Item = Result<SendRequest, Status>> + Send + 'static + Unpin,
    ) -> ServiceResult<ResponseStream> {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        let notify_service = self.clone();

        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let ns = notify_service.clone();

                let res = match req.msg {
                    Some(Msg::Email(email)) => email.send(ns).await,
                    Some(Msg::Sms(sms)) => sms.send(ns).await,
                    Some(Msg::InApp(in_app)) => in_app.send(ns).await,
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

use std::{sync::Arc, time::Duration};

use futures::Stream;
use tokio::{sync::mpsc, time::sleep};
use tonic::Status;
use tracing::info;

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
        todo!()
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

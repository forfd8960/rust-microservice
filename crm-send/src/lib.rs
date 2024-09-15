pub mod abi;
pub mod pb;

use futures::Stream;
use pb::{notification_server::Notification, send_request::Msg, SendRequest, SendResponse};
use std::{pin::Pin, sync::Arc};
use tokio::sync::mpsc;
use tonic::{async_trait, Request, Response, Status, Streaming};

#[derive(Clone)]
pub struct NotificationService {
    inner: Arc<NotificationServiceInner>,
}

pub struct NotificationServiceInner {
    sender: mpsc::Sender<Msg>,
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[async_trait]
impl Notification for NotificationService {
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> Result<Response<Self::SendStream>, Status> {
        todo!()
    }
}

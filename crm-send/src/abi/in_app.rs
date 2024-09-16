use tonic::Status;
use tracing::warn;

use crate::{
    pb::{send_request::Msg, InAppMessage, SendRequest, SendResponse},
    NotificationService,
};

use super::{to_ts, Sender};

impl Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let msg_id = self.message_id.clone();

        svc.sender.send(Msg::InApp(self)).await.map_err(|e| {
            warn!("failed to send in-app msg: {:?}", e);
            Status::internal("failed to send msg")
        })?;

        Ok(SendResponse {
            message_id: msg_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<InAppMessage> for Msg {
    fn from(in_app: InAppMessage) -> Self {
        Msg::InApp(in_app)
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(in_app_msg: InAppMessage) -> Self {
        let msg = in_app_msg.into();
        SendRequest { msg: Some(msg) }
    }
}

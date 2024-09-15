use tonic::Status;

use crate::{pb::SendResponse, NotificationService};

pub trait Sender {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status>;
}

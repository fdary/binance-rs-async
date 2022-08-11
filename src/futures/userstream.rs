use serde_json::from_str;

use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;

static USER_DATA_STREAM: &str = "/fapi/v1/listenKey";

#[derive(Clone)]
pub struct FuturesUserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl FuturesUserStream {
    pub async fn start(&self) -> Result<UserDataStream> {
        let data = self.client.post(USER_DATA_STREAM, None).await?;
        let user_data_stream: UserDataStream = from_str(data.as_str())?;

        Ok(user_data_stream)
    }

    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.put(USER_DATA_STREAM, listen_key, None).await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }

    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.delete(USER_DATA_STREAM, listen_key, None).await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }
}

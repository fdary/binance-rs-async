use futures::StreamExt;
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use url::Url;

use crate::config::Config;
use crate::errors::*;
use crate::futures::ws_model::WebsocketEvent;

pub static STREAM_ENDPOINT: &str = "stream";
pub static WS_ENDPOINT: &str = "ws";
pub static OUTBOUND_ACCOUNT_INFO: &str = "outboundAccountInfo";
pub static OUTBOUND_ACCOUNT_POSITION: &str = "outboundAccountPosition";
pub static EXECUTION_REPORT: &str = "executionReport";
pub static KLINE: &str = "kline";
pub static AGGREGATED_TRADE: &str = "aggTrade";
pub static DEPTH_ORDERBOOK: &str = "depthUpdate";
pub static PARTIAL_ORDERBOOK: &str = "lastUpdateId";
pub static DAYTICKER: &str = "24hrTicker";

pub fn all_ticker_stream() -> &'static str {
    "!ticker@arr"
}

pub fn ticker_stream(symbol: &str) -> String {
    format!("{}@ticker", symbol)
}

pub fn agg_trade_stream(symbol: &str) -> String {
    format!("{}@aggTrade", symbol)
}

pub fn trade_stream(symbol: &str) -> String {
    format!("{}@trade", symbol)
}

pub fn kline_stream(symbol: &str, interval: &str) -> String {
    format!("{}@kline_{}", symbol, interval)
}

pub fn book_ticker_stream(symbol: &str) -> String {
    format!("{}@bookTicker", symbol)
}

pub fn all_book_ticker_stream() -> &'static str {
    "!bookTicker"
}

pub fn all_mini_ticker_stream() -> &'static str {
    "!miniTicker@arr"
}

pub fn mini_ticker_stream(symbol: &str) -> String {
    format!("{}@miniTicker", symbol)
}

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `levels`: 5, 10 or 20
/// * `update_speed`: 1000 or 100
pub fn partial_book_depth_stream(symbol: &str, levels: u16, update_speed: u16) -> String {
    format!("{}@depth{}@{}ms", symbol, levels, update_speed)
}

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `update_speed`: 1000 or 100
pub fn diff_book_depth_stream(symbol: &str, update_speed: u16) -> String {
    format!("{}@depth@{}ms", symbol, update_speed)
}

fn combined_stream(streams: Vec<String>) -> String {
    streams.join("/")
}

pub struct WebSockets {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    conf: Config,
}

impl WebSockets {
    /// New websocket holder with default configuration
    pub fn new() -> WebSockets
    {
        Self::new_with_options(Config::default())
    }

    /// New websocket holder with provided configuration
    pub fn new_with_options(conf: Config) -> WebSockets
    {
        WebSockets {
            socket: None,
            conf,
        }
    }

    /// Connect to multiple websocket endpoints
    /// N.B: WE has to be CombinedStreamEvent
    pub async fn connect_multiple(&mut self, endpoints: Vec<String>) -> Result<()> {
        let mut url = Url::parse(&self.conf.futures_ws_endpoint)?;
        url.path_segments_mut()
            .map_err(|_| Error::UrlParserError(url::ParseError::RelativeUrlWithoutBase))?
            .push(STREAM_ENDPOINT);
        url.set_query(Some(&format!("streams={}", combined_stream(endpoints))));

        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {}", e))),
        }
    }

    /// Connect to a websocket endpoint
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/{}/{}", self.conf.futures_ws_endpoint, WS_ENDPOINT, endpoint);

        let url = Url::parse(&wss)?;

        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {}", e))),
        }
    }

    /// Disconnect from the endpoint
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub fn socket(&self) -> &Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)> {
        &self.socket
    }

    pub async fn next_event(&mut self) -> Result<WebsocketEvent> {
        loop {
            if let Some((ref mut socket, _)) = self.socket {
                let message = socket.next().await.unwrap()?;

                match message {
                    Message::Text(msg) => {
                        if !msg.is_empty() {
                            let event: WebsocketEvent = from_str(msg.as_str())?;
                            return Ok(event);
                        }
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => {}
                    Message::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {:?}", e)));
                    }
                }
            }
        }
    }

}

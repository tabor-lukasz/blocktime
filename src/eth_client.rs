use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Deserialize, Debug, Clone)]
struct BlockNrResponse {
    pub jsonrpc: String,
    pub id: i32,
    pub result: String,
}

#[derive(Deserialize, Debug, Clone)]
struct BlockDetailsResult {
    pub timestamp: String,
}
#[derive(Deserialize, Debug, Clone)]
struct BlockDetailsResponse {
    pub jsonrpc: String,
    pub id: i32,
    pub result: BlockDetailsResult,
}

pub struct EthClient {
    last_response: Arc<RwLock<Option<String>>>,
}

impl EthClient {
    pub async fn new() -> Self {
        let rval = EthClient {
            last_response: Arc::new(RwLock::new(None)),
        };
        let resp = rval.last_response.clone();

        let mut last_block_nr = "".to_string();

        tokio::spawn(async move {
            let client = reqwest::Client::new();

            let nr_request_url = "https://api.etherscan.io/api?module=proxy&action=eth_blockNumber";

            loop {
                let nr_response = match client.get(nr_request_url).send().await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }
                };

                let block_nr_resp = match nr_response.json::<BlockNrResponse>().await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }
                };

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                if last_block_nr.eq(&block_nr_resp.result) {
                    continue;
                } else {
                    last_block_nr = block_nr_resp.result.clone();
                }

                let block_details_request_url = format!("https://api.etherscan.io/api?module=proxy&action=eth_getBlockByNumber&tag={}&boolean=true",block_nr_resp.result);
                let details_response = match client.get(block_details_request_url).send().await {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }
                };

                let block_details_resp = match details_response.json::<BlockDetailsResponse>().await
                {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }
                };

                let ts = i64::from_str_radix(
                    block_details_resp.result.timestamp.trim_start_matches("0x"),
                    16,
                )
                .unwrap();
                let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc)
                    .to_rfc2822();
                println!("{}", &dt);
                *resp.write().await = Some(dt);

                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });

        rval
    }

    pub async fn get_eth_ts(&self) -> Option<String> {
        self.last_response.read().await.clone()
    }
}

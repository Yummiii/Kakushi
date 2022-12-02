use crate::cf_structs::CfResponse;
use cf_structs::Record;
use configs::Configs;
use log::{error, info};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde_json::Value;
use std::{thread, time::Duration};
use tokio::{task, time::sleep};

mod cf_structs;
mod configs;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let cfgs = Configs::get();

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", cfgs.cf_api_key)).unwrap(),
    );
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    task::spawn(async move {
        loop {
            let mut records = get_records(&cfgs, &client).await;
            while records.is_err() {
                error!("Error fetching records, trying again in 5 seconds...");
                sleep(Duration::from_secs(5)).await;
                records = get_records(&cfgs, &client).await;
            }

            let ip = get_ip()
                .await
                .map_err(|_| error!("Could not get external ip address"))
                .unwrap();

            for record in records.unwrap() {
                if record.content != ip {
                    let updated = update_record(
                        Record {
                            content: ip.clone(),
                            name: record.name.clone(),
                            ..record
                        },
                        &cfgs,
                        &client,
                    )
                    .await;

                    if updated.is_ok() {
                        info!("Updated {} from {} to {}", record.name, record.content, ip);
                    } else {                        
                        error!("Could not update {}", record.name);
                    }
                }
            }

            sleep(Duration::from_secs(300)).await;
        }
    });

    thread::park();
}

async fn get_records(cfgs: &Configs, client: &Client) -> Result<Vec<Record>, ()> {
    let res = client.get(cfgs.url("dns_records")).send().await;

    match res {
        Ok(res) => {
            let res: CfResponse<Option<Vec<Record>>> = res.json().await.unwrap();
            if res.success {
                let records: Vec<Record> = res
                    .result
                    .unwrap()
                    .into_iter()
                    .filter(|x| cfgs.cf_records.contains(&x.name))
                    .collect();
                Ok(records)
            } else {
                error!(
                    "{}",
                    res.errors
                        .into_iter()
                        .map(|x| x.message)
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                Err(())
            }
        }
        Err(e) => {
            error!("{}", e.to_string());
            Err(())
        }
    }
}

async fn update_record(record: Record, cfgs: &Configs, client: &Client) -> Result<(), ()> {
    let res = client
        .put(cfgs.url(&format!("dns_records/{}", record.id)))
        .json(&record)
        .send()
        .await;

    if res.is_ok() {
        let res: CfResponse<Value> = res.unwrap().json().await.unwrap();
        if res.success {
            Ok(())
        } else {
            error!(
                "{}",
                res.errors
                    .into_iter()
                    .map(|x| x.message)
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            Err(())
        }
    } else {
        error!("{}", res.unwrap_err().to_string());
        Err(())
    }
}

async fn get_ip() -> Result<String, ()> {
    let res = reqwest::get("https://api.ipify.org/").await;
    if res.is_ok() {
        Ok(res.unwrap().text().await.unwrap())
    } else {
        error!("{}", res.unwrap_err().to_string());
        Err(())
    }
}

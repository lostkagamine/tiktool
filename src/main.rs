use std::io::Write;

use clap::StructOpt;
use reqwest::Client;

const TIKTOK_API_BASE: &'static str =
    "https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/?text_speaker=en_us_002";

#[derive(clap::Parser)]
struct CommandLine {
    #[clap(short='o', long="--out")]
    pub output: Option<String>,
    #[clap(short='t', long="--text")]
    pub text: String,
}

#[derive(serde::Deserialize)]
struct ApiResp {
    pub data: ApiRespInner,
}

#[derive(serde::Deserialize)]
struct ApiRespInner {
    pub v_str: String,
}

#[tokio::main]
async fn main() {
    let cli = CommandLine::parse();
    let client = Client::new();
    let rq = client.post(TIKTOK_API_BASE);
    let rq = rq.query(&[("req_text", &cli.text)]);
    let res = rq.send().await.unwrap();
    let res_text = res.text().await.unwrap();
    let f = serde_json::from_str::<ApiResp>(&res_text).unwrap();
    let bytes = data_encoding::BASE64.decode(f.data.v_str.as_bytes()).unwrap();
    if let None = cli.output {
        std::io::stdout().lock().write_all(&bytes).unwrap();
    } else if let Some(x) = cli.output {
        std::fs::write(x, &bytes).unwrap();
    }
}

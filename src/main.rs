mod param;
mod url;

use param::*;
use reqwest::header::{self, COOKIE};
use reqwest::Client;
use tokio::time;
use url::*;

type RespErr = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), RespErr> {
    let params = Post {
        aid: AID,
        uuid: UUID,
        _signature: SIGNATURE,
        cookie: _COOKIE,
    };
    let client = init(&params).unwrap();

    let sign_resp = is_sign_in(client.clone()).await?;
    if let Some(false) = sign_resp.data {
        sign_in(client.clone(), &params).await?;

        time::sleep(time::Duration::from_secs(5)).await;
    };

    let draw_resp = is_draw(client.clone()).await?;
    if draw_resp.data.free_count != 0 {
        draw(client.clone(), &params).await?;
    }
    Ok(())
}

//初始化reqwest客户端
fn init(params: &Post) -> Result<Client, RespErr> {
    let mut headers = header::HeaderMap::new();
    headers.insert(COOKIE, params.cookie.parse().unwrap());
    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

//是否已签到
async fn is_sign_in(client: Client) -> Result<SignResp, RespErr> {
    let resp = client
        .get(BASE_URL.to_string() + ISSIGNINURL)
        .send()
        .await?
        .json::<SignResp>()
        .await?;
    println!("是否已签到：{:#?}", resp.data.unwrap());
    Ok(resp)
}

//签到
async fn sign_in(client: Client, new_post: &Post<'_>) -> Result<SignResp, RespErr> {
    let resp = client
        .post(BASE_URL.to_string() + SIGNINURL)
        .json(new_post)
        .send()
        .await?
        .json::<SignResp>()
        .await?;
    println!("签到：{:#?}", resp);
    Ok(resp)
}

//是否已抽奖
async fn is_draw(client: Client) -> Result<DrawResp, RespErr> {
    let resp = client
        .get(BASE_URL.to_string() + ISDRAW)
        .send()
        .await?
        .json::<DrawResp>()
        .await?;
    println!("未抽奖次数还有{:#?}次", resp.data.free_count);
    Ok(resp)
}

//抽奖
async fn draw(client: Client, new_post: &Post<'_>) -> Result<String, RespErr> {
    let resp = client
        .post(BASE_URL.to_string() + DRAWURL)
        .json(new_post)
        .send()
        .await?
        .text()
        .await?;
    println!("抽奖：{:#?}", resp);
    Ok(resp)
}

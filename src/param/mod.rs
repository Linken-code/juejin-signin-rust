use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DrawData {
    pub lottery: Vec<Lottery>,
    pub free_count: i8,
    pub point_cost: i32,
}

#[derive(Debug, Deserialize)]
pub struct Lottery {
    pub lottery_id: String,
    pub lottery_name: String,
    pub lottery_type: i8,
    pub lottery_image: String,
    pub unlock_count: i8,
}

#[derive(Debug, Deserialize)]
pub struct SignResp {
    pub err_no: i32,
    pub err_msg: String,
    pub data: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DrawResp {
    pub err_no: i8,
    pub err_msg: String,
    pub data: DrawData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post<'a> {
    pub aid: &'a str,
    pub uuid: &'a str,
    pub _signature: &'a str,
    pub cookie: &'a str,
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub pilots: Vec<Pilot>,
    pub competitions: Vec<Competition>,
    pub rankings: Vec<Ranking>,
    pub admin_users: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pilot {
    pub pin: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub location: String,
    pub overseas: bool,
    pub exchange_rate: f64,
    pub comp_date: String,
    pub num_tasks: i64,
    pub ave_num_participants: f64,
    pub placings: Vec<Placing>,
    pub comp_value: f64,
    pub pq: Value,
    pub pn: f64,
    pub ta: f64,
    pub td: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Placing {
    pub id: i64,
    pub pilot: Pilot2,
    pub place: i64,
    pub points: f64,
    pub fai_points: f64,
    pub pp: f64,
    pub pplacing: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pilot2 {
    pub pin: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ranking {
    pub id: String,
    pub date: String,
    pub ranking_points: Vec<RankingPoint>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingPoint {
    pub pilot_first_name: String,
    pub pilot_last_name: String,
    pub pilot_pin: String,
    pub pilot_gender: Option<String>,
    pub results: Vec<CompResult>,
    pub total_points: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompResult {
    pub place: i64,
    pub comp_name: String,
    #[serde(rename = "compID")]
    pub comp_id: String,
    pub points: f64,
    pub overseas: bool,
}

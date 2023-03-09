use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighCloudRoot {
    pub compinfo: Compinfo,
    pub data: Vec<Vec<Value>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compinfo {
    pub com_pk: String,
    pub com_name: String,
    pub com_location: String,
    pub com_date_from: String,
    pub com_date_to: String,
    pub com_meet_dir_name: String,
    pub com_sanction: Value,
    pub com_type: String,
    pub com_code: String,
    pub for_pk: String,
    pub com_overall_score: String,
    pub com_overall_param: String,
    pub com_time_offset: String,
    pub com_class: String,
    pub com_style_sheet: Value,
    pub com_team_size: String,
    pub com_team_scoring: String,
    pub com_team_over: String,
    pub com_contact: Value,
    pub com_locked: String,
    pub com_entry_restrict: String,
    pub reg_pk: String,
    pub for_class: String,
    pub for_version: String,
    #[serde(rename = "forGoalSSpenalty")]
    pub for_goal_sspenalty: String,
    pub for_nom_goal: String,
    pub for_min_distance: String,
    pub for_nom_distance: String,
    pub for_nom_time: String,
    pub for_arrival: String,
    pub for_departure: String,
    pub for_linear_dist: String,
    pub for_diff_dist: String,
    pub for_diff_ramp: String,
    pub for_diff_calc: String,
    #[serde(rename = "forOLCPoints")]
    pub for_olcpoints: String,
    #[serde(rename = "forOLCBase")]
    pub for_olcbase: String,
    #[serde(rename = "forHBESS")]
    pub for_hbess: String,
    pub for_dist_measure: String,
    pub for_weight_start: String,
    pub for_weight_arrival: String,
    pub for_weight_speed: String,
    pub for_stopped_glide_bonus: String,
    pub for_weight_dist: String,
    pub for_scale_to_validity: String,
    pub for_discrete_classes: String,
    pub for_nom_launch: String,
    pub for_error_margin: String,
    #[serde(rename = "TotalValidity")]
    pub total_validity: i64,
}

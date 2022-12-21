use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub username: String,
}

pub struct BaseGameModeStats {
    pub days_number_one: i32,
    pub longest_number_one: i32,
    pub percentile: f32,
    pub percentile_peak: f32,
    pub tier: Tier,
    pub tier_peak: Tier,
    pub rank: i32,
    pub rank_peak: i32,
}

pub enum Tag {
    SupporterTier1,
    SupporterTier2,
    SupporterTier3,
    Moderator,
    POMP,
    Contributor,
    ClanLeader,
    ClanCoLeader,
}

pub enum GameMode {
    BattleRoyale,
    CapturePoints,
    Custom,
    PropHunt,
    SoloDeathMatch,
    Speedrun,
    TeamDeathMatch,
    Tutorial,
    Vehicles,
}

pub enum Tier {
    Unranked,
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Pro,
    Master,
}

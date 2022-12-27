use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contains all information about a Player
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// A unique and unchanging identifier of the player
    pub id: String,
    /// A unique but variable identifier of the player
    pub username: String,
    pub tags: Vec<Tag>,
}

/// Common statistics across all game modes
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Tag {
    Supporter,
    Moderator,
    POMP,
    Contributor,
    ClanLeader,
    ClanCoLeader,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
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

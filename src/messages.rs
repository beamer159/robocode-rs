use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Message {
    ServerHandshake(ServerHandshake),
    BotHandshake(BotHandshake),
    GameStartedEventForBot(GameStarted),
    BotReady
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerHandshake {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(default)]
    name: String,
    variant: String,
    version: String,
    #[serde(rename = "gameTypes")]
    game_types: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BotHandshake {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "countryCodes", skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "programmingLang", skip_serializing_if = "Option::is_none")]
    pub programming_lang: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<u8>,
    #[serde(rename = "teamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(rename = "teamVersion", skip_serializing_if = "Option::is_none")]
    pub team_version: Option<String>,
    #[serde(rename = "isDroid", skip_serializing_if = "Option::is_none")]
    pub is_droid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameStarted {
    #[serde(rename = "myId")]
    my_id: u32,
    #[serde(rename = "startX", skip_serializing_if = "Option::is_none")]
    start_x: Option<f64>,
    #[serde(rename = "startY", skip_serializing_if = "Option::is_none")]
    start_y: Option<f64>,
    #[serde(rename = "startDirection", skip_serializing_if = "Option::is_none")]
    start_direction: Option<f64>,
    #[serde(rename = "teammateIds", skip_serializing_if = "Option::is_none")]
    teammate_ids: Option<Vec<u32>>,
    #[serde(rename = "gameSetup")]
    game_setup: GameSetup
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameSetup {
    #[serde(rename = "gameType")]
    game_type: String,
    #[serde(rename = "arenaWidth")]
    arena_width: u32,
    #[serde(rename = "isArenaWidthLocked")]
    is_arena_width_locked: bool,
    #[serde(rename = "arenaHeight")]
    arena_height: u32,
    #[serde(rename = "isArenaHeightLocked")]
    is_arena_height_locked: bool,
    #[serde(rename = "minNumberOfParticipants")]
    min_number_of_participants: u32,
    #[serde(rename = "isMinNumberOfParticipantsLocked")]
    is_min_number_of_participants_locked: bool,
    #[serde(rename = "maxNumberOfParticipants", skip_serializing_if = "Option::is_none")]
    max_number_of_participants: Option<u32>,
    #[serde(rename = "isMaxNumberOfParticipantsLocked")]
    is_max_number_of_participants_locked: bool,
    #[serde(rename = "numberOfRounds")]
    number_of_rounds: u32,
    #[serde(rename = "isNumberOfRoundsLocked")]
    is_number_of_rounds_locked: bool,
    #[serde(rename = "gunCoolingRate")]
    gun_cooling_rate: f64,
    #[serde(rename = "isGunCoolingRateLocked")]
    is_gun_cooling_rate_locked: bool,
    #[serde(rename = "maxInactivityTurns")]
    max_inactivity_turns: u32,
    #[serde(rename = "isMaxInactivityTurnsLocked")]
    is_max_inactivity_turns_locked: bool,
    #[serde(rename = "turnTimeout")]
    turn_timeout: u32,
    #[serde(rename = "isTurnTimeoutLocked")]
    is_turn_timeout_locked: bool,
    #[serde(rename = "readyTimeout")]
    ready_timeout: u32,
    #[serde(rename = "isReadyTimeoutLocked")]
    is_ready_timeout_locked: bool,
    #[serde(rename = "defaultTurnsPerSecond")]
    default_turns_per_second: u32
}
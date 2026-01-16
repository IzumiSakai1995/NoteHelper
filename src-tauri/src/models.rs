use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Novel {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Player {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub level: i32,
    pub exp: i64,
    pub hp: i32,
    pub shield: i32,
    pub attack: i32,
    pub phys_defense: i32,
    pub mag_defense: i32,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub vitality: i32,
    pub spirit: i32,
    pub crit_chance: i32,
    pub crit_dmg: i32,
    pub deadly_chance: i32,
    pub deadly_dmg: i32,
    pub equipment: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Monster {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub level: i32,
    pub race: Option<String>,
    pub rarity: Option<String>,
    pub prefix: Option<String>,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub damage_reduction: i32,
    pub drops: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Buff {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub buff_type: Option<String>,
    pub duration: Option<i32>,
    pub effect: Option<String>, // Deprecated, use attributes
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlayerBuff {
    pub id: i64,
    pub player_id: i64,
    pub buff_id: i64,
    pub applied_at: Option<NaiveDateTime>,
    // We will join Buff table to get details
    #[sqlx(default)] 
    pub name: String, 
    #[sqlx(default)]
    pub attributes: Option<serde_json::Value>,
    #[sqlx(default)]
    pub duration: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub category: Option<String>,
    pub item_type: Option<String>,
    pub attributes: Option<serde_json::Value>,
    pub description: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub novel_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Dungeon {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub monsters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub monsters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Map {
    pub id: i64,
    pub novel_id: i64,
    pub name: String,
    pub order_num: i32,
    pub monsters: Option<serde_json::Value>,
}

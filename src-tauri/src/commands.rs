use crate::db::DbState;
use crate::models::*;
use tauri::State;
use sqlx::Row;

// Novel Commands
#[tauri::command]
pub async fn create_novel(state: State<'_, DbState>, name: String, description: Option<String>) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO novels (name, description) VALUES ($1, $2) RETURNING id"
    )
    .bind(name)
    .bind(description)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_novels(state: State<'_, DbState>) -> Result<Vec<Novel>, String> {
    sqlx::query_as::<_, Novel>("SELECT * FROM novels ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_novel(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM novels WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Player Commands
#[tauri::command]
pub async fn create_player(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO players (novel_id, name) VALUES ($1, $2) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_players(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Player>, String> {
    let mut players = sqlx::query_as::<_, Player>("SELECT * FROM players WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Also fetch items to calculate stats if equipment exists? 
    // Actually, equipment is stored as JSON in player table. 
    // We need to parse it and add stats.
    // The format of equipment JSON is { "Main Hand": itemId, "Chest": itemId, ... }
    
    // We need to fetch all items to lookup stats
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
        
    let item_map: HashMap<i64, &Item> = items.iter().map(|i| (i.id, i)).collect();

    for p in players.iter_mut() {
        if let Some(eq) = &p.equipment {
            if let Some(map) = eq.as_object() {
                for (_, item_id_val) in map {
                    if let Some(item_id) = item_id_val.as_i64() {
                        if let Some(item) = item_map.get(&item_id) {
                            if let Some(attrs) = &item.attributes {
                                if let Some(attr_map) = attrs.as_object() {
                                    // Add item stats to player stats
                                    if let Some(val) = attr_map.get("phys_attack").and_then(|v| v.as_i64()) { p.attack += val as i32; }
                                    // Note: Player struct has 'attack' but item has 'phys_attack'/'mag_attack'.
                                    // For simplicity let's map phys_attack to attack for now, or sum them?
                                    // The prompt says "calculate character's equipment stats".
                                    
                                    if let Some(val) = attr_map.get("hp").and_then(|v| v.as_i64()) { p.hp += val as i32; }
                                    if let Some(val) = attr_map.get("mp").and_then(|v| v.as_i64()) { /* p.mp += val as i32; */ } // Player doesn't have MP yet
                                    if let Some(val) = attr_map.get("phys_defense").and_then(|v| v.as_i64()) { p.phys_defense += val as i32; }
                                    if let Some(val) = attr_map.get("mag_defense").and_then(|v| v.as_i64()) { p.mag_defense += val as i32; }
                                    
                                    if let Some(val) = attr_map.get("strength").and_then(|v| v.as_i64()) { p.strength += val as i32; }
                                    if let Some(val) = attr_map.get("agility").and_then(|v| v.as_i64()) { p.agility += val as i32; }
                                    if let Some(val) = attr_map.get("intelligence").and_then(|v| v.as_i64()) { p.intelligence += val as i32; }
                                    if let Some(val) = attr_map.get("vitality").and_then(|v| v.as_i64()) { p.vitality += val as i32; }
                                    if let Some(val) = attr_map.get("spirit").and_then(|v| v.as_i64()) { p.spirit += val as i32; }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(players)
}

#[tauri::command]
pub async fn update_player_stats(
    state: State<'_, DbState>,
    id: i64,
    hp: i32,
    max_hp: i32,
    shield: i32,
    max_shield: i32,
    attack: i32,
    phys_defense: i32,
    mag_defense: i32,
    strength: i32,
    agility: i32,
    intelligence: i32,
    vitality: i32,
    spirit: i32,
    crit_chance: i32,
    crit_dmg: i32,
    deadly_chance: i32,
    deadly_dmg: i32
) -> Result<(), String> {
    sqlx::query(
        "UPDATE players SET hp=$1, max_hp=$2, shield=$3, max_shield=$4, attack=$5, phys_defense=$6, mag_defense=$7, strength=$8, agility=$9, intelligence=$10, vitality=$11, spirit=$12, crit_chance=$13, crit_dmg=$14, deadly_chance=$15, deadly_dmg=$16 WHERE id=$17"
    )
    .bind(hp).bind(max_hp).bind(shield).bind(max_shield).bind(attack).bind(phys_defense).bind(mag_defense)
    .bind(strength).bind(agility).bind(intelligence).bind(vitality).bind(spirit)
    .bind(crit_chance).bind(crit_dmg).bind(deadly_chance).bind(deadly_dmg)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn update_player_equipment(
    state: State<'_, DbState>,
    id: i64,
    equipment: Option<serde_json::Value>
) -> Result<(), String> {
    sqlx::query("UPDATE players SET equipment=$1 WHERE id=$2")
        .bind(equipment)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Monster Commands
#[tauri::command]
pub async fn create_monster(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String,
    level: Option<i32>,
    race: Option<String>,
    rarity: Option<String>,
    prefix: Option<String>,
    base_hp: Option<i32>,
    base_attack: Option<i32>,
    base_defense: Option<i32>,
    damage_reduction: Option<i32>,
    fixed_damage_reduction: Option<i32>,
    description: Option<String>,
    prefix_id: Option<i64>,
    rank_id: Option<i64>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO monsters (
            novel_id, name, level, race, rarity, prefix, 
            base_hp, base_attack, base_defense, 
            hp, attack, defense, 
            damage_reduction, fixed_damage_reduction, description, 
            prefix_id, rank_id
        ) 
        VALUES (
            $1, $2, $3, $4, $5, $6, 
            $7, $8, $9, 
            $7, $8, $9, 
            $10, $11, $12, 
            $13, $14
        ) 
        RETURNING id
        "#
    )
    .bind(novel_id)
    .bind(name)
    .bind(level.unwrap_or(1))
    .bind(race)
    .bind(rarity)
    .bind(prefix)
    .bind(base_hp.unwrap_or(100))
    .bind(base_attack.unwrap_or(10))
    .bind(base_defense.unwrap_or(0))
    .bind(damage_reduction.unwrap_or(0))
    .bind(fixed_damage_reduction.unwrap_or(0))
    .bind(description)
    .bind(prefix_id)
    .bind(rank_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

use std::collections::HashMap;

#[tauri::command]
pub async fn get_monsters(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Monster>, String> {
    let mut monsters = sqlx::query_as::<_, Monster>("SELECT * FROM monsters WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Fetch related data for calculation
    let prefixes = sqlx::query_as::<_, MonsterPrefix>("SELECT * FROM monster_prefixes WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let ranks = sqlx::query_as::<_, MonsterRank>("SELECT * FROM monster_ranks WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let settings = sqlx::query_as::<_, GameSetting>("SELECT * FROM game_settings WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let level_coeff = settings.map(|s| s.level_coefficient).unwrap_or(1.0);

    let prefix_map: HashMap<i64, &MonsterPrefix> = prefixes.iter().map(|p| (p.id, p)).collect();
    let rank_map: HashMap<i64, &MonsterRank> = ranks.iter().map(|r| (r.id, r)).collect();

    for m in monsters.iter_mut() {
        let p_hp = m.prefix_id.and_then(|id| prefix_map.get(&id)).map(|p| p.hp_modifier).unwrap_or(1.0);
        let p_atk = m.prefix_id.and_then(|id| prefix_map.get(&id)).map(|p| p.attack_modifier).unwrap_or(1.0);
        let p_def = m.prefix_id.and_then(|id| prefix_map.get(&id)).map(|p| p.defense_modifier).unwrap_or(1.0);

        let r_hp = m.rank_id.and_then(|id| rank_map.get(&id)).map(|r| r.hp_modifier).unwrap_or(1.0);
        let r_atk = m.rank_id.and_then(|id| rank_map.get(&id)).map(|r| r.attack_modifier).unwrap_or(1.0);
        let r_def = m.rank_id.and_then(|id| rank_map.get(&id)).map(|r| r.defense_modifier).unwrap_or(1.0);

        // Formula: Base * (1 + (Level - 1) * Coeff) * Prefix * Rank
        // If Level < 1, treat as 1
        let level = if m.level < 1 { 1 } else { m.level };
        let level_mult = 1.0 + (level as f64 - 1.0) * level_coeff;

        m.hp = (m.base_hp as f64 * level_mult * p_hp * r_hp) as i32;
        m.attack = (m.base_attack as f64 * level_mult * p_atk * r_atk) as i32;
        m.defense = (m.base_defense as f64 * level_mult * p_def * r_def) as i32;
    }

    Ok(monsters)
}

#[tauri::command]
pub async fn update_monster(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    level: i32,
    race: Option<String>,
    rarity: Option<String>,
    prefix: Option<String>,
    base_hp: i32,
    base_attack: i32,
    base_defense: i32,
    damage_reduction: i32,
    fixed_damage_reduction: i32,
    drops: Option<serde_json::Value>,
    description: Option<String>,
    prefix_id: Option<i64>,
    rank_id: Option<i64>
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE monsters 
        SET name=$1, level=$2, race=$3, rarity=$4, prefix=$5, base_hp=$6, base_attack=$7, base_defense=$8, damage_reduction=$9, drops=$10, description=$11, prefix_id=$12, rank_id=$13, fixed_damage_reduction=$15
        WHERE id=$14
        "#
    )
    .bind(name)
    .bind(level)
    .bind(race)
    .bind(rarity)
    .bind(prefix)
    .bind(base_hp)
    .bind(base_attack)
    .bind(base_defense)
    .bind(damage_reduction)
    .bind(drops)
    .bind(description)
    .bind(prefix_id)
    .bind(rank_id)
    .bind(id)
    .bind(fixed_damage_reduction)
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

// Item Commands
#[tauri::command]
pub async fn create_item(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String,
    category: Option<String>,
    item_type: Option<String>,
    description: Option<String>,
    category_id: Option<i64>,
    level: Option<i32>,
    rarity: Option<String>,
    attributes: Option<serde_json::Value>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO items (novel_id, name, category, item_type, description, category_id, level, rarity, attributes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(category)
    .bind(item_type)
    .bind(description)
    .bind(category_id)
    .bind(level.unwrap_or(1))
    .bind(rarity)
    .bind(attributes)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_items(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Item>, String> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE novel_id = $1 ORDER BY id ASC")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_item(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    category: Option<String>,
    item_type: Option<String>,
    attributes: Option<serde_json::Value>,
    description: Option<String>,
    category_id: Option<i64>,
    level: i32,
    rarity: Option<String>
) -> Result<(), String> {
    sqlx::query("UPDATE items SET name=$1, category=$2, item_type=$3, attributes=$4, description=$5, category_id=$6, level=$7, rarity=$8 WHERE id=$9")
        .bind(name)
        .bind(category)
        .bind(item_type)
        .bind(attributes)
        .bind(description)
        .bind(category_id)
        .bind(level)
        .bind(rarity)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Skill Commands
#[tauri::command]
pub async fn create_skill(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String,
    description: Option<String>,
    level: Option<i32>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO skills (novel_id, name, description, level) VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(description)
    .bind(level.unwrap_or(1))
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_skills(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Skill>, String> {
    sqlx::query_as::<_, Skill>("SELECT * FROM skills WHERE novel_id = $1 ORDER BY id ASC")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_skill(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    description: Option<String>,
    level: i32
) -> Result<(), String> {
    sqlx::query("UPDATE skills SET name=$1, description=$2, level=$3 WHERE id=$4")
        .bind(name)
        .bind(description)
        .bind(level)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_skill(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM skills WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_item(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM items WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Map Commands
#[tauri::command]
pub async fn create_map(
    state: State<'_, DbState>,
    novel_id: i64,
    parent_id: Option<i64>,
    name: String,
    order_num: Option<i32>,
    description: Option<String>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO maps (novel_id, parent_id, name, order_num, description) VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(novel_id)
    .bind(parent_id)
    .bind(name)
    .bind(order_num.unwrap_or(0))
    .bind(description)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_maps(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Map>, String> {
    sqlx::query_as::<_, Map>("SELECT * FROM maps WHERE novel_id = $1 ORDER BY order_num ASC, id ASC")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_map(
    state: State<'_, DbState>,
    id: i64,
    parent_id: Option<i64>,
    name: String,
    order_num: i32,
    description: Option<String>
) -> Result<(), String> {
    sqlx::query("UPDATE maps SET name=$1, order_num=$2, parent_id=$3, description=$4 WHERE id=$5")
        .bind(name)
        .bind(order_num)
        .bind(parent_id)
        .bind(description)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn update_map_monsters(
    state: State<'_, DbState>,
    id: i64,
    monsters: Option<serde_json::Value>
) -> Result<(), String> {
    sqlx::query("UPDATE maps SET monsters=$1 WHERE id=$2")
        .bind(monsters)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_map(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM maps WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Buff Commands
#[tauri::command]
pub async fn create_buff(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String,
    buff_type: Option<String>,
    duration: Option<i32>,
    attributes: Option<serde_json::Value>,
    effects: Option<serde_json::Value>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO buffs (novel_id, name, buff_type, duration, attributes, effects) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(buff_type)
    .bind(duration)
    .bind(attributes)
    .bind(effects)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_buffs(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Buff>, String> {
    sqlx::query_as::<_, Buff>("SELECT * FROM buffs WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_buff(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    buff_type: Option<String>,
    duration: Option<i32>,
    attributes: Option<serde_json::Value>,
    effects: Option<serde_json::Value>
) -> Result<(), String> {
    sqlx::query("UPDATE buffs SET name=$1, buff_type=$2, duration=$3, attributes=$4, effects=$5 WHERE id=$6")
        .bind(name)
        .bind(buff_type)
        .bind(duration)
        .bind(attributes)
        .bind(effects)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_buff(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM buffs WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Player Buff Commands
#[tauri::command]
pub async fn apply_buff(
    state: State<'_, DbState>,
    player_id: i64,
    buff_id: i64
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO player_buffs (player_id, buff_id) VALUES ($1, $2) RETURNING id"
    )
    .bind(player_id)
    .bind(buff_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn remove_buff(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM player_buffs WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_player_buffs(state: State<'_, DbState>, player_id: i64) -> Result<Vec<PlayerBuff>, String> {
    // Join buffs table to get buff details
    sqlx::query_as::<_, PlayerBuff>(
        r#"
        SELECT pb.id, pb.player_id, pb.buff_id, pb.applied_at, 
               b.name, b.attributes, b.duration
        FROM player_buffs pb
        JOIN buffs b ON pb.buff_id = b.id
        WHERE pb.player_id = $1
          AND (b.duration IS NULL OR pb.applied_at + (b.duration * interval '1 second') > LOCALTIMESTAMP)
        "#
    )
    .bind(player_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())
}

// Category Commands
#[tauri::command]
pub async fn create_category(
    state: State<'_, DbState>,
    novel_id: i64,
    parent_id: Option<i64>,
    name: String,
    description: Option<String>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO categories (novel_id, parent_id, name, description) VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(novel_id)
    .bind(parent_id)
    .bind(name)
    .bind(description)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_categories(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Category>, String> {
    sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE novel_id = $1 ORDER BY id ASC")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_category(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    description: Option<String>
) -> Result<(), String> {
    sqlx::query("UPDATE categories SET name=$1, description=$2 WHERE id=$3")
        .bind(name)
        .bind(description)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_category(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Monster Prefix Commands
#[tauri::command]
pub async fn create_monster_prefix(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String,
    hp_modifier: f64,
    attack_modifier: f64,
    defense_modifier: f64
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO monster_prefixes (novel_id, name, hp_modifier, attack_modifier, defense_modifier) VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(hp_modifier)
    .bind(attack_modifier)
    .bind(defense_modifier)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_monster_prefixes(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<MonsterPrefix>, String> {
    sqlx::query_as::<_, MonsterPrefix>("SELECT * FROM monster_prefixes WHERE novel_id = $1 ORDER BY id ASC")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_monster_prefix(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    hp_modifier: f64,
    attack_modifier: f64,
    defense_modifier: f64
) -> Result<(), String> {
    sqlx::query("UPDATE monster_prefixes SET name=$1, hp_modifier=$2, attack_modifier=$3, defense_modifier=$4 WHERE id=$5")
        .bind(name)
        .bind(hp_modifier)
        .bind(attack_modifier)
        .bind(defense_modifier)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_monster_prefix(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM monster_prefixes WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Monster Rank Commands
#[tauri::command]
pub async fn create_monster_rank(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String,
    hp_modifier: f64,
    attack_modifier: f64,
    defense_modifier: f64,
    color: Option<String>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO monster_ranks (novel_id, name, hp_modifier, attack_modifier, defense_modifier, color) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(hp_modifier)
    .bind(attack_modifier)
    .bind(defense_modifier)
    .bind(color)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_monster_ranks(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<MonsterRank>, String> {
    sqlx::query_as::<_, MonsterRank>("SELECT * FROM monster_ranks WHERE novel_id = $1 ORDER BY id ASC")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_monster_rank(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    hp_modifier: f64,
    attack_modifier: f64,
    defense_modifier: f64,
    color: Option<String>
) -> Result<(), String> {
    sqlx::query("UPDATE monster_ranks SET name=$1, hp_modifier=$2, attack_modifier=$3, defense_modifier=$4, color=$5 WHERE id=$6")
        .bind(name)
        .bind(hp_modifier)
        .bind(attack_modifier)
        .bind(defense_modifier)
        .bind(color)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_monster_rank(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM monster_ranks WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Game Settings Commands
#[tauri::command]
pub async fn get_game_settings(state: State<'_, DbState>, novel_id: i64) -> Result<GameSetting, String> {
    let row = sqlx::query_as::<_, GameSetting>("SELECT * FROM game_settings WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    if let Some(settings) = row {
        Ok(settings)
    } else {
        // Create default settings if not exists
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO game_settings (novel_id, level_coefficient) VALUES ($1, 1.0) RETURNING novel_id"
        )
        .bind(novel_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(GameSetting {
            novel_id: row.0,
            level_coefficient: 1.0,
        })
    }
}

#[tauri::command]
pub async fn update_game_settings(
    state: State<'_, DbState>,
    novel_id: i64,
    level_coefficient: f64
) -> Result<(), String> {
    sqlx::query("INSERT INTO game_settings (novel_id, level_coefficient) VALUES ($1, $2) ON CONFLICT (novel_id) DO UPDATE SET level_coefficient = $2")
        .bind(novel_id)
        .bind(level_coefficient)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Player Template Commands
#[tauri::command]
pub async fn create_player_template(
    state: State<'_, DbState>,
    novel_id: i64,
    name: String,
    level: Option<i32>,
    exp: Option<i64>,
    strength: Option<i32>,
    agility: Option<i32>,
    intelligence: Option<i32>,
    vitality: Option<i32>,
    spirit: Option<i32>,
    crit_chance: Option<i32>,
    crit_dmg: Option<i32>,
    deadly_chance: Option<i32>,
    deadly_dmg: Option<i32>,
    strength_to_phys_attack: Option<f64>,
    strength_to_max_hp: Option<f64>,
    agility_to_phys_attack: Option<f64>,
    intelligence_to_mag_attack: Option<f64>,
    intelligence_to_mag_defense: Option<f64>,
    vitality_to_phys_defense: Option<f64>,
    vitality_to_mag_defense: Option<f64>,
    vitality_to_max_hp: Option<f64>,
    spirit_to_mana_regen: Option<f64>,
    spirit_to_mag_defense: Option<f64>
) -> Result<i64, String> {
    let lvl = level.unwrap_or(1);
    let xp = exp.unwrap_or(0);
    let strv = strength.unwrap_or(10) as f64;
    let agiv = agility.unwrap_or(10) as f64;
    let intv = intelligence.unwrap_or(10) as f64;
    let vitv = vitality.unwrap_or(10) as f64;
    let spiv = spirit.unwrap_or(10) as f64;

    let s_pa = strength_to_phys_attack.unwrap_or(1.0);
    let s_hp = strength_to_max_hp.unwrap_or(0.0);
    let a_pa = agility_to_phys_attack.unwrap_or(1.0);
    let i_ma = intelligence_to_mag_attack.unwrap_or(1.0);
    let i_md = intelligence_to_mag_defense.unwrap_or(0.0);
    let v_pd = vitality_to_phys_defense.unwrap_or(1.0);
    let v_md = vitality_to_mag_defense.unwrap_or(0.5);
    let v_hp = vitality_to_max_hp.unwrap_or(10.0);
    let s_mr = spirit_to_mana_regen.unwrap_or(0.0);
    let s_mg = spirit_to_mag_defense.unwrap_or(0.5);

    let c_ch = crit_chance.unwrap_or(0);
    let c_dm = crit_dmg.unwrap_or(0);
    let d_ch = deadly_chance.unwrap_or(0);
    let d_dm = deadly_dmg.unwrap_or(0);

    let max_hp = (strv * s_hp + vitv * v_hp).round() as i32;
    let attack = (strv * s_pa + agiv * a_pa + intv * i_ma).round() as i32;
    let phys_defense = (vitv * v_pd).round() as i32;
    let mag_defense = (intv * i_md + spiv * s_mg).round() as i32;

    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO player_templates (
            novel_id, name, level, exp, strength, agility, intelligence, vitality, spirit,
            max_hp, attack, phys_defense, mag_defense,
            crit_chance, crit_dmg, deadly_chance, deadly_dmg,
            strength_to_phys_attack, strength_to_max_hp, agility_to_phys_attack,
            intelligence_to_mag_attack, intelligence_to_mag_defense,
            vitality_to_phys_defense, vitality_to_mag_defense, vitality_to_max_hp,
            spirit_to_mana_regen, spirit_to_mag_defense
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9,
            $10, $11, $12, $13,
            $14, $15, $16, $17,
            $18, $19, $20,
            $21, $22,
            $23, $24, $25,
            $26, $27
        )
        RETURNING id
        "#
    )
    .bind(novel_id)
    .bind(name)
    .bind(lvl)
    .bind(xp)
    .bind(strv as i32)
    .bind(agiv as i32)
    .bind(intv as i32)
    .bind(vitv as i32)
    .bind(spiv as i32)
    .bind(max_hp)
    .bind(attack)
    .bind(phys_defense)
    .bind(mag_defense)
    .bind(c_ch)
    .bind(c_dm)
    .bind(d_ch)
    .bind(d_dm)
    .bind(s_pa)
    .bind(s_hp)
    .bind(a_pa)
    .bind(i_ma)
    .bind(i_md)
    .bind(v_pd)
    .bind(v_md)
    .bind(v_hp)
    .bind(s_mr)
    .bind(s_mg)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_player_templates(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<PlayerTemplate>, String> {
    sqlx::query_as::<_, PlayerTemplate>("SELECT * FROM player_templates WHERE novel_id = $1 ORDER BY id ASC")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_player_template(
    state: State<'_, DbState>,
    id: i64,
    name: String,
    level: i32,
    exp: i64,
    strength: i32,
    agility: i32,
    intelligence: i32,
    vitality: i32,
    spirit: i32,
    crit_chance: i32,
    crit_dmg: i32,
    deadly_chance: i32,
    deadly_dmg: i32,
    strength_to_phys_attack: f64,
    strength_to_max_hp: f64,
    agility_to_phys_attack: f64,
    intelligence_to_mag_attack: f64,
    intelligence_to_mag_defense: f64,
    vitality_to_phys_defense: f64,
    vitality_to_mag_defense: f64,
    vitality_to_max_hp: f64,
    spirit_to_mana_regen: f64,
    spirit_to_mag_defense: f64
) -> Result<(), String> {
    let strv = strength as f64;
    let agiv = agility as f64;
    let intv = intelligence as f64;
    let vitv = vitality as f64;
    let spiv = spirit as f64;

    let max_hp = (strv * strength_to_max_hp + vitv * vitality_to_max_hp).round() as i32;
    let attack = (strv * strength_to_phys_attack + agiv * agility_to_phys_attack + intv * intelligence_to_mag_attack).round() as i32;
    let phys_defense = (vitv * vitality_to_phys_defense).round() as i32;
    let mag_defense = (intv * intelligence_to_mag_defense + spiv * spirit_to_mag_defense).round() as i32;

    sqlx::query(
        r#"
        UPDATE player_templates 
        SET name=$1, level=$2, exp=$3, strength=$4, agility=$5, intelligence=$6, vitality=$7, spirit=$8,
            max_hp=$9, attack=$10, phys_defense=$11, mag_defense=$12,
            crit_chance=$13, crit_dmg=$14, deadly_chance=$15, deadly_dmg=$16,
            strength_to_phys_attack=$13, strength_to_max_hp=$14, agility_to_phys_attack=$15,
            intelligence_to_mag_attack=$17, intelligence_to_mag_defense=$18,
            vitality_to_phys_defense=$19, vitality_to_mag_defense=$20, vitality_to_max_hp=$21,
            spirit_to_mana_regen=$22, spirit_to_mag_defense=$23
        WHERE id=$24
        "#
    )
    .bind(name)
    .bind(level)
    .bind(exp)
    .bind(strength)
    .bind(agility)
    .bind(intelligence)
    .bind(vitality)
    .bind(spirit)
    .bind(max_hp)
    .bind(attack)
    .bind(phys_defense)
    .bind(mag_defense)
    .bind(crit_chance)
    .bind(crit_dmg)
    .bind(deadly_chance)
    .bind(deadly_dmg)
    .bind(strength_to_phys_attack)
    .bind(strength_to_max_hp)
    .bind(agility_to_phys_attack)
    .bind(intelligence_to_mag_attack)
    .bind(intelligence_to_mag_defense)
    .bind(vitality_to_phys_defense)
    .bind(vitality_to_mag_defense)
    .bind(vitality_to_max_hp)
    .bind(spirit_to_mana_regen)
    .bind(spirit_to_mag_defense)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_player_template(state: State<'_, DbState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM player_templates WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn create_player_from_template(
    state: State<'_, DbState>,
    novel_id: i64,
    template_id: i64,
    name: String
) -> Result<i64, String> {
    let tpl = sqlx::query_as::<_, PlayerTemplate>("SELECT * FROM player_templates WHERE id = $1 AND novel_id = $2")
        .bind(template_id)
        .bind(novel_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO players (
            novel_id, name, level, exp, 
            hp, max_hp, shield, max_shield, 
            attack, phys_defense, mag_defense,
            strength, agility, intelligence, vitality, spirit,
            crit_chance, crit_dmg, deadly_chance, deadly_dmg,
            equipment
        ) VALUES (
            $1, $2, $3, $4,
            $5, $6, 0, 0,
            $7, $8, $9,
            $10, $11, $12, $13, $14,
            $15, $16, $17, $18,
            NULL
        )
        RETURNING id
        "#
    )
    .bind(novel_id)
    .bind(name)
    .bind(tpl.level)
    .bind(tpl.exp)
    .bind(tpl.max_hp)
    .bind(tpl.max_hp)
    .bind(tpl.attack)
    .bind(tpl.phys_defense)
    .bind(tpl.mag_defense)
    .bind(tpl.strength)
    .bind(tpl.agility)
    .bind(tpl.intelligence)
    .bind(tpl.vitality)
    .bind(tpl.spirit)
    .bind(tpl.crit_chance)
    .bind(tpl.crit_dmg)
    .bind(tpl.deadly_chance)
    .bind(tpl.deadly_dmg)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

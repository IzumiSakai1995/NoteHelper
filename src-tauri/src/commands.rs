use crate::db::DbState;
use crate::models::*;
use tauri::State;

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
    sqlx::query_as::<_, Player>("SELECT * FROM players WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_player_stats(
    state: State<'_, DbState>,
    id: i64,
    hp: i32,
    shield: i32,
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
        "UPDATE players SET hp=$1, shield=$2, attack=$3, phys_defense=$4, mag_defense=$5, strength=$6, agility=$7, intelligence=$8, vitality=$9, spirit=$10, crit_chance=$11, crit_dmg=$12, deadly_chance=$13, deadly_dmg=$14 WHERE id=$15"
    )
    .bind(hp).bind(shield).bind(attack).bind(phys_defense).bind(mag_defense)
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
    name: String
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO monsters (novel_id, name) VALUES ($1, $2) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_monsters(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Monster>, String> {
    sqlx::query_as::<_, Monster>("SELECT * FROM monsters WHERE novel_id = $1")
        .bind(novel_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
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
    hp: i32,
    attack: i32,
    defense: i32,
    damage_reduction: i32,
    drops: Option<serde_json::Value>,
    description: Option<String>
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE monsters 
        SET name=$1, level=$2, race=$3, rarity=$4, prefix=$5, hp=$6, attack=$7, defense=$8, damage_reduction=$9, drops=$10, description=$11
        WHERE id=$12
        "#
    )
    .bind(name)
    .bind(level)
    .bind(race)
    .bind(rarity)
    .bind(prefix)
    .bind(hp)
    .bind(attack)
    .bind(defense)
    .bind(damage_reduction)
    .bind(drops)
    .bind(description)
    .bind(id)
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
    category_id: Option<i64>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO items (novel_id, name, category, item_type, description, category_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(category)
    .bind(item_type)
    .bind(description)
    .bind(category_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(row.0)
}

#[tauri::command]
pub async fn get_items(state: State<'_, DbState>, novel_id: i64) -> Result<Vec<Item>, String> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE novel_id = $1")
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
    category_id: Option<i64>
) -> Result<(), String> {
    sqlx::query("UPDATE items SET name=$1, category=$2, item_type=$3, attributes=$4, description=$5, category_id=$6 WHERE id=$7")
        .bind(name)
        .bind(category)
        .bind(item_type)
        .bind(attributes)
        .bind(description)
        .bind(category_id)
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
    name: String,
    order_num: Option<i32>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO maps (novel_id, name, order_num) VALUES ($1, $2, $3) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(order_num.unwrap_or(0))
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
    name: String,
    order_num: i32
) -> Result<(), String> {
    sqlx::query("UPDATE maps SET name=$1, order_num=$2 WHERE id=$3")
        .bind(name)
        .bind(order_num)
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
    attributes: Option<serde_json::Value>
) -> Result<i64, String> {
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO buffs (novel_id, name, buff_type, duration, attributes) VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(novel_id)
    .bind(name)
    .bind(buff_type)
    .bind(duration)
    .bind(attributes)
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
    attributes: Option<serde_json::Value>
) -> Result<(), String> {
    sqlx::query("UPDATE buffs SET name=$1, buff_type=$2, duration=$3, attributes=$4 WHERE id=$5")
        .bind(name)
        .bind(buff_type)
        .bind(duration)
        .bind(attributes)
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

use sqlx::postgres::{PgPoolOptions, PgPool};

pub struct DbState {
    pub pool: PgPool,
}

pub async fn create_database_if_not_exists(base_url: &str, db_name: &str) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(base_url)
        .await?;
    
    let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
        .bind(db_name)
        .fetch_one(&pool)
        .await?;

    if !row.0 {
        sqlx::query(&format!("CREATE DATABASE \"{}\"", db_name))
            .execute(&pool)
            .await?;
    }
    
    pool.close().await;
    Ok(())
}

pub async fn init_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Execute each table creation separately to avoid "cannot insert multiple commands into a prepared statement" error
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS novels (
            id BIGSERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS player_templates (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            level INT DEFAULT 1,
            exp BIGINT DEFAULT 0,
            strength INT DEFAULT 10,
            agility INT DEFAULT 10,
            intelligence INT DEFAULT 10,
            vitality INT DEFAULT 10,
            spirit INT DEFAULT 10,
            max_hp INT DEFAULT 100,
            attack INT DEFAULT 10,
            phys_defense INT DEFAULT 0,
            mag_defense INT DEFAULT 0,
            strength_to_phys_attack FLOAT8 DEFAULT 1.0,
            strength_to_max_hp FLOAT8 DEFAULT 0.0,
            agility_to_phys_attack FLOAT8 DEFAULT 1.0,
            intelligence_to_mag_attack FLOAT8 DEFAULT 1.0,
            intelligence_to_mag_defense FLOAT8 DEFAULT 0.0,
            vitality_to_phys_defense FLOAT8 DEFAULT 1.0,
            vitality_to_mag_defense FLOAT8 DEFAULT 0.5,
            vitality_to_max_hp FLOAT8 DEFAULT 10.0,
            spirit_to_mana_regen FLOAT8 DEFAULT 0.0,
            spirit_to_mag_defense FLOAT8 DEFAULT 0.5,
            crit_chance INT DEFAULT 0,
            crit_dmg INT DEFAULT 0,
            deadly_chance INT DEFAULT 0,
            deadly_dmg INT DEFAULT 0
        );
        "#
    ).execute(&pool).await?;

    // Migration: Add crit/deadly fields to player_templates
    sqlx::query(
        r#"
        ALTER TABLE player_templates ADD COLUMN IF NOT EXISTS crit_chance INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    sqlx::query(
        r#"
        ALTER TABLE player_templates ADD COLUMN IF NOT EXISTS crit_dmg INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    sqlx::query(
        r#"
        ALTER TABLE player_templates ADD COLUMN IF NOT EXISTS deadly_chance INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    sqlx::query(
        r#"
        ALTER TABLE player_templates ADD COLUMN IF NOT EXISTS deadly_dmg INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            level INT DEFAULT 1,
            exp BIGINT DEFAULT 0,
            hp INT DEFAULT 100,
            shield INT DEFAULT 0,
            attack INT DEFAULT 10,
            phys_defense INT DEFAULT 0,
            mag_defense INT DEFAULT 0,
            strength INT DEFAULT 10,
            agility INT DEFAULT 10,
            intelligence INT DEFAULT 10,
            vitality INT DEFAULT 10,
            spirit INT DEFAULT 10,
            crit_chance INT DEFAULT 0,
            crit_dmg INT DEFAULT 0,
            deadly_chance INT DEFAULT 0,
            deadly_dmg INT DEFAULT 0,
            equipment JSONB
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS monsters (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            level INT DEFAULT 1,
            race TEXT,
            rarity TEXT,
            prefix TEXT,
            hp INT DEFAULT 100,
            attack INT DEFAULT 10,
            defense INT DEFAULT 0,
            damage_reduction INT DEFAULT 0,
            drops JSONB,
            description TEXT
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            category TEXT,
            item_type TEXT,
            attributes JSONB,
            description TEXT
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS skills (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            description TEXT,
            level INT DEFAULT 1
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS buffs (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            buff_type TEXT,
            duration INT,
            effect TEXT
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS dungeons (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            monsters JSONB
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS events (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            monsters JSONB
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS maps (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            order_num INT DEFAULT 0,
            monsters JSONB
        );
        "#
    ).execute(&pool).await?;

    // Create player_buffs table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS player_buffs (
            id BIGSERIAL PRIMARY KEY,
            player_id BIGINT REFERENCES players(id) ON DELETE CASCADE,
            buff_id BIGINT REFERENCES buffs(id) ON DELETE CASCADE,
            applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(&pool).await?;

    // Migration: Add monsters column to maps if it doesn't exist
    sqlx::query(
        r#"
        ALTER TABLE maps ADD COLUMN IF NOT EXISTS monsters JSONB;
        "#
    ).execute(&pool).await?;

    // Migration: Add category column to items if it doesn't exist
    sqlx::query(
        r#"
        ALTER TABLE items ADD COLUMN IF NOT EXISTS category TEXT;
        "#
    ).execute(&pool).await?;

    // Migration: Add phys_defense and mag_defense columns to players if they don't exist
    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS phys_defense INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS mag_defense INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    
    // Migration: Add new crit and deadly stats
    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS crit_chance INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS crit_dmg INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS deadly_chance INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS deadly_dmg INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    // Migration: Add attributes to buffs
    sqlx::query(
        r#"
        ALTER TABLE buffs ADD COLUMN IF NOT EXISTS attributes JSONB;
        "#
    ).execute(&pool).await?;

    // Migration: Add shield to players
    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS shield INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    // Migration: Add description to items
    sqlx::query(
        r#"
        ALTER TABLE items ADD COLUMN IF NOT EXISTS description TEXT;
        "#
    ).execute(&pool).await?;

    // Migration: Create categories table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            parent_id BIGINT REFERENCES categories(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            description TEXT
        );
        "#
    ).execute(&pool).await?;

    // Migration: Add category_id to items
    sqlx::query(
        r#"
        ALTER TABLE items ADD COLUMN IF NOT EXISTS category_id BIGINT REFERENCES categories(id) ON DELETE SET NULL;
        "#
    ).execute(&pool).await?;

    // Migration: Add order_num to maps
    sqlx::query(
        r#"
        ALTER TABLE maps ADD COLUMN IF NOT EXISTS order_num INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    // Migration: Create monster_prefixes table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS monster_prefixes (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            hp_modifier FLOAT8 DEFAULT 1.0,
            attack_modifier FLOAT8 DEFAULT 1.0,
            defense_modifier FLOAT8 DEFAULT 1.0
        );
        "#
    ).execute(&pool).await?;

    // Migration: Create monster_ranks table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS monster_ranks (
            id BIGSERIAL PRIMARY KEY,
            novel_id BIGINT REFERENCES novels(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            hp_modifier FLOAT8 DEFAULT 1.0,
            attack_modifier FLOAT8 DEFAULT 1.0,
            defense_modifier FLOAT8 DEFAULT 1.0
        );
        "#
    ).execute(&pool).await?;

    // Migration: Create game_settings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS game_settings (
            novel_id BIGINT PRIMARY KEY REFERENCES novels(id) ON DELETE CASCADE,
            level_coefficient FLOAT8 DEFAULT 1.0
        );
        "#
    ).execute(&pool).await?;

    // Migration: Add prefix_id and rank_id to monsters
    sqlx::query(
        r#"
        ALTER TABLE monsters ADD COLUMN IF NOT EXISTS prefix_id BIGINT REFERENCES monster_prefixes(id) ON DELETE SET NULL;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        ALTER TABLE monsters ADD COLUMN IF NOT EXISTS rank_id BIGINT REFERENCES monster_ranks(id) ON DELETE SET NULL;
        "#
    ).execute(&pool).await?;

    // Migration: Add base_hp, base_attack, base_defense to monsters
    // Initialize with existing values if they are 0 (which implies new column)
    sqlx::query(
        r#"
        ALTER TABLE monsters ADD COLUMN IF NOT EXISTS base_hp INT DEFAULT 0;
        "#
    ).execute(&pool).await?;
    
    // Copy existing hp to base_hp if base_hp is 0 (newly added)
    sqlx::query(
        r#"
        UPDATE monsters SET base_hp = hp WHERE base_hp = 0 AND hp > 0;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        ALTER TABLE monsters ADD COLUMN IF NOT EXISTS base_attack INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        UPDATE monsters SET base_attack = attack WHERE base_attack = 0 AND attack > 0;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        ALTER TABLE monsters ADD COLUMN IF NOT EXISTS base_defense INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        UPDATE monsters SET base_defense = defense WHERE base_defense = 0 AND defense > 0;
        "#
    ).execute(&pool).await?;

    // Migration: Add fixed_damage_reduction to monsters
    sqlx::query(
        r#"
        ALTER TABLE monsters ADD COLUMN IF NOT EXISTS fixed_damage_reduction INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    // Migration: Add parent_id to maps
    sqlx::query(
        r#"
        ALTER TABLE maps ADD COLUMN IF NOT EXISTS parent_id BIGINT REFERENCES maps(id) ON DELETE SET NULL;
        "#
    ).execute(&pool).await?;

    // Migration: Add color to monster_ranks
    sqlx::query(
        r#"
        ALTER TABLE monster_ranks ADD COLUMN IF NOT EXISTS color TEXT;
        "#
    ).execute(&pool).await?;

    // Migration: Add description to maps
    sqlx::query(
        r#"
        ALTER TABLE maps ADD COLUMN IF NOT EXISTS description TEXT;
        "#
    ).execute(&pool).await?;

    // Migration: Add level and rarity to items
    sqlx::query(
        r#"
        ALTER TABLE items ADD COLUMN IF NOT EXISTS level INT DEFAULT 1;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        ALTER TABLE items ADD COLUMN IF NOT EXISTS rarity TEXT;
        "#
    ).execute(&pool).await?;

    // Migration: Add effects to buffs (for dynamic formulas)
    sqlx::query(
        r#"
        ALTER TABLE buffs ADD COLUMN IF NOT EXISTS effects JSONB;
        "#
    ).execute(&pool).await?;

    // Migration: Add max_hp and max_shield to players
    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS max_hp INT DEFAULT 100;
        "#
    ).execute(&pool).await?;
    
    // Set max_hp to hp (current) if it was just added
    sqlx::query(
        r#"
        UPDATE players SET max_hp = hp WHERE max_hp = 100 AND hp > 100;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        ALTER TABLE players ADD COLUMN IF NOT EXISTS max_shield INT DEFAULT 0;
        "#
    ).execute(&pool).await?;

    Ok(pool)
}

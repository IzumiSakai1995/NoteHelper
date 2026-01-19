mod db;
mod models;
mod commands;

use db::{init_db, create_database_if_not_exists, DbState};
use commands::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let base_db_url = "postgres://liuzheng:950519Lz!@192.168.1.24:5433/postgres";
                let target_db_name = "farm_game";
                let target_db_url = "postgres://liuzheng:950519Lz!@192.168.1.24:5433/farm_game";
                
                // Ensure database exists
                create_database_if_not_exists(base_db_url, target_db_name)
                    .await
                    .expect("Failed to create database");

                let pool = init_db(target_db_url).await.expect("Failed to initialize database");
                app.manage(DbState { pool });
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_novel, get_novels, delete_novel,
            create_player, get_players, update_player_stats, update_player_equipment,
            create_monster, get_monsters, update_monster,
            create_item, get_items, update_item, delete_item,
            create_skill, get_skills, update_skill, delete_skill,
            create_map, get_maps, update_map, update_map_monsters, delete_map,
            create_buff, get_buffs, update_buff, delete_buff,
            apply_buff, remove_buff, get_player_buffs,
            create_category, get_categories, update_category, delete_category,
            create_monster_prefix, get_monster_prefixes, update_monster_prefix, delete_monster_prefix,
            create_monster_rank, get_monster_ranks, update_monster_rank, delete_monster_rank,
            get_game_settings, update_game_settings,
            create_player_template, get_player_templates, update_player_template, delete_player_template,
            create_player_from_template
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

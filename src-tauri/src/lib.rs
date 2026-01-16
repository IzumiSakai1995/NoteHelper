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
            create_map, get_maps, update_map, update_map_monsters, delete_map,
            create_buff, get_buffs, update_buff, delete_buff,
            apply_buff, remove_buff, get_player_buffs,
            create_category, get_categories, update_category, delete_category
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

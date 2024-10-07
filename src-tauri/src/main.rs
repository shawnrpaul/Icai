// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_must_use, dead_code)]

mod db;
mod playlist;
mod song;
mod watcher;
use std::sync::{Arc, Mutex};
use tauri::{api::path::audio_dir, App, Manager};
use watcher::DirWatcher;

pub type Watcher = Arc<Mutex<DirWatcher>>;

fn setup(app: &mut App) -> DirWatcher {
    let path = audio_dir().expect("wtf").join("icai.db");
    if !path.exists() {
        db::create_db_file(path)
    }
    let conn = db::create_connection();
    let mut paths = db::get_paths(&conn).unwrap();
    paths.insert(0, audio_dir().expect("wtf"));
    let dir_watcher = watcher::DirWatcher::new(&paths);
    match app.windows().get("main") {
        Some(window) => {
            let fs = window.fs_scope();
            for path in paths.iter() {
                fs.allow_directory(path, true);
            }
        }
        _ => (),
    };
    db::refresh_db(&conn, paths);
    dir_watcher
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let dir_watcher = setup(app);
            app.manage(Arc::new(Mutex::new(dir_watcher)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::add_dir,
            db::remove_dir,
            db::get_all_songs,
            db::get_all_playlists,
            db::get_playlist,
            db::create_playlist,
            db::delete_playlist,
            db::add_playlist_song,
            db::remove_playlist_song
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

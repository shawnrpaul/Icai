use crate::{playlist::Playlist, song::Song, Watcher};
use rusqlite::Connection;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use tauri::{api::path::audio_dir, Manager, State};
use walkdir::WalkDir;

pub fn create_db_file(path: PathBuf) {
    File::create(path).unwrap();
    let conn = create_connection();
    conn.execute(
        "CREATE TABLE if NOT EXISTS songs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title VARCHAR NOT NULL,
            path VARCHAR NOT NULL UNIQUE,
            count INTEGER NOT NULL
        )",
        (),
    );
    conn.execute(
        "CREATE TABLE if NOT EXISTS paths (
            path VARCHAR PRIMARY KEY
        )",
        (),
    );
    conn.execute(
        "CREATE TABLE if NOT EXISTS playlists (
            name VARCHAR PRIMARY KEY
        )",
        (),
    );
}

pub fn create_connection() -> Connection {
    let path = audio_dir().expect("wtf").join("icai.db");
    let conn = Connection::open(path).expect("Connection to the database couldn't be made");
    conn
}

pub fn _add_song(path: &PathBuf) {
    let path2 = path.as_path();
    if !["mp3", "wav", "ogg"].contains(&path2.extension().unwrap().to_str().unwrap()) {
        return;
    }
    if !path2.exists() {
        return;
    }
    let conn = create_connection();
    conn.execute(
        "INSERT INTO songs (title, path, count) VALUES (?1, ?2, ?3)",
        &[
            path.file_stem().unwrap().to_str().unwrap(),
            &path.display().to_string(),
            "0",
        ],
    );
}

pub fn _remove_song(path: &PathBuf) {
    let path2 = path.as_path();
    if !["mp3", "wav", "ogg"].contains(&path2.extension().unwrap().to_str().unwrap()) {
        return;
    }
    if path2.exists() {
        return;
    }
    let conn = create_connection();
    conn.execute(
        "DELETE FROM songs WHERE path = ?1",
        &[&path.display().to_string()],
    );
}

fn check_for_new_files(conn: &Connection, dir: String) {
    for dir_entry in WalkDir::new(dir) {
        match dir_entry {
            Err(_) => (),
            Ok(file) => {
                let data = file.metadata();
                match data {
                    Err(_) => (),
                    Ok(metadata) => {
                        if !metadata.is_file() {
                            continue;
                        }
                        let path = file.path();
                        match &path.extension() {
                            Some(ext) => {
                                if !["mp3", "wav", "ogg"].contains(&ext.to_str().unwrap()) {
                                    continue;
                                }
                            }
                            _ => {
                                continue;
                            }
                        };
                        match conn.prepare("SELECT * FROM songs WHERE path = ?1") {
                            Err(_) => (),
                            Ok(mut stmt) => match stmt.exists(&[&path.display().to_string()]) {
                                Err(_) => (),
                                Ok(success) => {
                                    if success {
                                        continue;
                                    }
                                    let _ = conn.execute(
                                        "INSERT INTO songs (title, path, count) VALUES (?1, ?2, ?3)",
                                        &[path.file_stem().unwrap().to_str().unwrap(), &path.display().to_string(), "0"],
                                        );
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

fn check_for_removed_files(conn: &Connection) {
    match conn.prepare("SELECT * FROM songs") {
        Err(_) => (),
        Ok(mut stmt) => {
            let paths: Vec<String> = stmt
                .query_map([], |row| Ok(row.get(2).unwrap()))
                .unwrap()
                .filter_map(|path| path.ok())
                .collect();
            for path in paths.iter() {
                if !Path::new(path).exists() {
                    conn.execute("DELETE FROM songs WHERE path = ?1", &[path]);
                }
            }
        }
    }
}

pub fn get_paths(conn: &Connection) -> Result<Vec<PathBuf>, String> {
    let prepare = conn.prepare("SELECT * FROM paths");
    match prepare {
        Err(e) => Err(e.to_string()),
        Ok(mut stmt) => {
            let paths: Vec<PathBuf> = stmt
                .query_map([], |row| {
                    let path: String = row.get(0).unwrap();
                    Ok(PathBuf::from(path))
                })
                .unwrap()
                .filter_map(|song| song.ok())
                .collect();
            Ok(paths)
        }
    }
}

pub fn refresh_db(conn: &Connection, paths: Vec<PathBuf>) {
    for path in paths {
        check_for_new_files(conn, path.display().to_string());
    }
    check_for_removed_files(conn);
}

#[tauri::command]
pub async fn add_dir(
    window: tauri::Window,
    watcher: State<'_, Watcher>,
    path: String,
) -> Result<(), String> {
    let directory = Path::new(&path);
    match watcher.lock().expect("").add_dir(directory) {
        Ok(val) => {
            let conn = create_connection();
            match conn.execute("INSERT INTO paths (path) VALUES (?1)", &[&path]) {
                Ok(_) => {
                    window.fs_scope().allow_directory(&path, true);
                    check_for_new_files(&conn, path)
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            };
            Ok(val)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn remove_dir(
    window: tauri::Window,
    watcher: State<'_, Watcher>,
    path: String,
) -> Result<(), String> {
    let dir = Path::new(&path);
    if dir.starts_with(audio_dir().unwrap()) {
        return Err("Can't remove the audio directory".to_string());
    }
    match watcher.lock().expect("").remove_dir(dir) {
        Ok(val) => {
            let conn = create_connection();
            match conn.execute("DELETE FROM paths WHERE path = ?1", &[&path]) {
                Ok(_) => {
                    window.fs_scope().forbid_directory(&path, true);
                    let sql = format!(
                        "DELETE FROM songs WHERE path LIKE '{}'",
                        dir.join("%").display().to_string()
                    );
                    conn.execute(&sql, ());
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            };
            Ok(val)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn _get_all_songs(conn: &Connection) -> Result<Playlist, String> {
    let prepare = conn.prepare("SELECT * FROM songs");
    match prepare {
        Err(_) => Err("Couldn't get songs".to_string()),
        Ok(mut stmt) => {
            let songs: Vec<Song> = stmt
                .query_map([], |row| {
                    Ok(Song::new(
                        row.get(0).unwrap(),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                        row.get(3).unwrap(),
                    ))
                })
                .unwrap()
                .filter_map(|song| song.ok())
                .collect();
            let playlist = Playlist::from("All Songs".to_string(), songs);
            Ok(playlist)
        }
    }
}

#[tauri::command]
pub async fn get_all_songs(_: tauri::State<'_, Watcher>) -> Result<Playlist, String> {
    let conn = create_connection();
    _get_all_songs(&conn)
}

fn _get_playlist_songs(name: &String, conn: &Connection) -> Result<Vec<Song>, String> {
    let sql = &format!(
        "SELECT songs.id, title, path, count FROM songs INNER JOIN `{}` as playlist ON songs.id = playlist.song",
        name
    );
    let prepare = conn.prepare(sql);
    match prepare {
        Err(e) => Err(e.to_string()),
        Ok(mut stmt) => {
            let songs: Vec<Song> = stmt
                .query_map([], |row| {
                    Ok(Song::new(
                        row.get(0).unwrap(),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                        row.get(3).unwrap(),
                    ))
                })
                .unwrap()
                .filter_map(|song| song.ok())
                .collect();
            Ok(songs)
        }
    }
}

#[tauri::command]
pub async fn get_all_playlists(_: State<'_, Watcher>) -> Result<Vec<Playlist>, String> {
    let conn = create_connection();
    let prepare = conn.prepare("SELECT * FROM playlists");
    let mut playlists: Vec<Playlist> = match prepare {
        Err(e) => return Err(e.to_string()),
        Ok(mut stmt) => stmt
            .query_map([], |row| Ok(Playlist::new(row.get(0).unwrap())))
            .unwrap()
            .filter_map(|playlist| playlist.ok())
            .collect(),
    };
    for playlist in playlists.iter_mut() {
        let songs = _get_playlist_songs(&playlist.name(), &conn);
        match songs {
            Ok(songs) => playlist.set_songs(songs),
            _ => (),
        }
    }
    match _get_all_songs(&conn) {
        Ok(playlist) => playlists.insert(0, playlist),
        _ => (),
    };
    Ok(playlists)
}

#[tauri::command]
pub async fn get_playlist(_: State<'_, Watcher>, name: String) -> Result<Playlist, String> {
    let conn = create_connection();
    match _get_playlist_songs(&name, &conn) {
        Err(e) => Err(e.to_string()),
        Ok(songs) => Ok(Playlist::from(name, songs)),
    }
}

#[tauri::command]
pub async fn create_playlist(_: State<'_, Watcher>, name: String) -> Result<(), String> {
    let conn = create_connection();
    let sql = format!(
        "CREATE TABLE `{}` (
            song INTEGER PRIMARY KEY,
            FOREIGN KEY(song) REFERENCES songs(id) ON DELETE CASCADE
        )",
        name
    );
    match conn.execute(&sql, ()) {
        Err(e) => Err(e.to_string()),
        Ok(_) => {
            conn.execute("INSERT INTO playlists (name) VALUES(?1)", &[&name]);
            Ok(())
        }
    }
}

#[tauri::command]
pub async fn delete_playlist(_: State<'_, Watcher>, name: String) -> Result<(), String> {
    let conn = create_connection();
    match conn.execute(&format!("DROP TABLE `{}`", name), ()) {
        Err(e) => Err(e.to_string()),
        Ok(_) => {
            conn.execute("DELETE FROM playlists WHERE name = ?1", &[&name]);
            Ok(())
        }
    }
}

#[tauri::command]
pub async fn add_playlist_song(
    _: State<'_, Watcher>,
    playlist: String,
    id: u16,
) -> Result<(), String> {
    let conn = create_connection();
    let sql = format!("INSERT INTO `{}` (song) VALUES(?1)", playlist);
    match conn.execute(&sql, &[&id.to_string()]) {
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}

#[tauri::command]
pub async fn remove_playlist_song(
    _: State<'_, Watcher>,
    playlist: String,
    id: u16,
) -> Result<(), String> {
    let conn = create_connection();
    let sql = format!("DELETE FROM `{}` WHERE song = ?1", playlist);
    match conn.execute(&sql, &[&id.to_string()]) {
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}

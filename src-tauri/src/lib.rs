#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_games, replace_art])
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Clone, serde::Serialize)]
struct Game {
    id: u32,
    name: String,
    slug: String,
    lastplayed: Option<u32>,
    installed_at: Option<u32>,
    has_custom_banner: u32,
    has_custom_coverart_big: u32,
    playtime: f32,
}

#[derive(Debug, Clone, serde::Serialize)]
struct GameWithArt {
    game: Game,
    coverart_path: String,
    coverart_width: usize,
    coverart_height: usize,
    banner_path: String,
    banner_width: usize,
    banner_height: usize,
}

impl TryFrom<Game> for GameWithArt {
    type Error = String;
    fn try_from(game: Game) -> Result<Self, Self::Error> {
        println!("{}", game.name);
        let binding = home_dir().unwrap_or(std::path::PathBuf::new());
        let homedir = binding.display();
        let coverart_path = format!("{}/.local/share/lutris/coverart/", homedir);
        let banner_path = format!("{}/.local/share/lutris/banners/", homedir);
        let coverart_path = match extension_from_path(&coverart_path, &game.slug) {
            Ok(Some(value)) => value,
            Ok(None) => "".to_string(),
            Err(value) => {
                println!("{}", value);
                return Err(value);
            }
        };
        let coverart_dims = imagesize::size(&coverart_path);
        let coverart_dims: imagesize::ImageSize = match coverart_dims {
            Ok(value) => value,
            _ => imagesize::ImageSize {
                width: 0,
                height: 0,
            },
        };
        let banner_path = match extension_from_path(&banner_path, &game.slug) {
            Ok(Some(value)) => value,
            Ok(None) => "".to_string(),
            Err(value) => {
                println!("{}", value);
                return Err(value);
            }
        };
        let banner_dims = imagesize::size(&banner_path);
        let banner_dims = match banner_dims {
            Ok(value) => value,
            _ => imagesize::ImageSize {
                width: 0,
                height: 0,
            },
        };
        Ok(GameWithArt {
            coverart_path,
            banner_path,
            coverart_width: coverart_dims.width,
            coverart_height: coverart_dims.height,
            banner_width: banner_dims.width,
            banner_height: banner_dims.height,
            game,
        })
    }
}

fn extension_from_path(art_path: &String, slug: &String) -> Result<Option<String>, String> {
    let coverart_entries = fs::read_dir(art_path.clone());
    let mut coverart_entries = match coverart_entries {
        Ok(e) => e,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let entry: Option<String> = coverart_entries.find_map(|entry| {
        let entry = match entry {
            Ok(e) => e,
            _ => {
                return None;
            }
        };
        let path = entry.path();
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        match filename.contains(slug) {
            true => Some(path.to_str().unwrap().to_string()),
            _ => None,
        }
    });
    Ok(entry)
}

use anyhow_tauri::IntoTAResult;
use home::home_dir;
use rusqlite::{Connection, OpenFlags, Result};
use std::fs;
use tauri::Url;

#[tauri::command]
fn get_games() -> Result<Vec<GameWithArt>, String> {
    let binding = home_dir().unwrap_or(std::path::PathBuf::new());
    let homedir = binding.display();
    let connection = Connection::open_with_flags(
        format!("{}/.local/share/lutris/pga.db", homedir),
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    );
    let connection = match connection {
        Ok(conn) => conn,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let query = "SELECT id,name,slug,lastplayed,installed_at,has_custom_banner,has_custom_coverart_big,playtime FROM 'games'";
    let res = connection.prepare(query);
    let mut res = match res {
        Ok(r) => r,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let games = res.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            slug: row.get(2)?,
            lastplayed: Some(row.get(3).unwrap_or(0)),
            installed_at: Some(row.get(4).unwrap_or(0)),
            has_custom_banner: row.get(5).unwrap_or(0),
            has_custom_coverart_big: row.get(6).unwrap_or(0),
            playtime: row.get(7).unwrap_or(0.0),
        })
    });
    let games = match games {
        Ok(g) => g,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let games: Vec<GameWithArt> = games
        .map(|g| {
            dbg!(&g);
            g
        })
        .filter_map(Result::ok)
        .filter_map(|g| g.try_into().ok())
        .collect();
    Result::Ok(games)
}

use std::{fs::File, io::copy};

fn download_image(path: String, url: String) -> anyhow::Result<()> {
    let url = Url::parse(&url)?;
    let mut response = reqwest::blocking::get(url)?;
    let mut file = File::create(path)?;
    copy(&mut response, &mut file)?;
    Ok(())
}

#[tauri::command]
fn replace_art(path: String, url: String) -> anyhow_tauri::TAResult<()> {
    download_image(path, url).into_ta_result()
}

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod mrpack;
mod types;
use anyhow::Result;
use std::fs;
use std::io::{BufReader, Cursor, Write};
use std::path::Path;
use std::time::SystemTime;
use std::{fs::File, path::PathBuf};

use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use image::DynamicImage;
use mrpack::modrinthpack::{ModrinthPack, ModrinthFileEnvTypes};
use reqwest::Response;
use serde_json::{Map, Value};
use types::*;
use walkdir::WalkDir;

use crate::mrpack::modrinthpack::ModrinthManifest;
// use image::io::Reader as ImageReader;

const BASE_URL: &str = "https://modpack.vloedje.nl";

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_minecraft_directory,
      directory_exists,
      path_join,
      install_modpack
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_minecraft_directory() -> String {
  let home_dir = tauri::api::path::home_dir().unwrap();
  let home_dir_path = home_dir.as_path();
  match std::env::consts::OS {
    "linux" => {
      let minecraft_path = std::path::Path::new(".minecraft");
      let minecraft_dir = home_dir_path.join(minecraft_path);

      minecraft_dir.to_string_lossy().into()
    }
    "windows" => {
      let minecraft_path = std::path::Path::new("AppData\\Roaming\\.minecraft");
      let minecraft_dir = home_dir_path.join(minecraft_path);

      minecraft_dir.to_string_lossy().into()
    }
    "macos" => {
      let minecraft_path = std::path::Path::new("Library/Application Support/minecraft");
      let minecraft_dir = home_dir_path.join(minecraft_path);

      minecraft_dir.to_string_lossy().into()
    }
    _ => "".into(),
  }
}

#[tauri::command]
fn directory_exists(path: String) -> bool {
  std::path::Path::new(&path).exists().into()
}

#[tauri::command]
fn path_join(path_string: String, second_path_string: String) -> String {
  let path = std::path::Path::new(&path_string);
  let second_path = std::path::Path::new(&second_path_string);

  path.join(second_path).to_string_lossy().into()
}

async fn install_loader(
  loader_id: &String,
  minecraft_version: &String,
  loader_version: &String,
  gamepath: &std::path::Path,
) -> Result<()> {
  let version_dir = gamepath.join("versions").join(&loader_id);
  fs::create_dir_all(&version_dir).expect("Unable to create loader directory");

  let response: Response = reqwest::get(format!(
    "https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json",
    minecraft_version, loader_version
  ))
  .await
  .expect("Unable to fetch loader configuration");
  let mut loader_config: Value = response
    .json::<Value>()
    .await
    .expect("Unable to get body from loader configuration response");
  let obj = loader_config.as_object_mut().expect("");
  obj.insert("id".to_owned(), Value::String(loader_id.to_owned()));

  let file: File = File::create(version_dir.join(format!("{}.json", &loader_id)))
    .expect("Unable to create loader config");
  serde_json::to_writer_pretty(file, &loader_config).expect("Unable to write to loader config");

  Ok(())
}

async fn install_profile(
  gamepath: &std::path::Path,
  modpack: &Modpack,
  application_dir: &std::path::Path,
  icon_path: &PathBuf,
) -> Result<()> {
  let profiles_file: File = File::open(gamepath.join("launcher_profiles.json"))
    .expect("Unable to open launcher_profiles.json");
  let mut profiles: Value = serde_json::from_reader(BufReader::new(&profiles_file))
    .expect("Unable to parse launcher_profiles.json");
  let now: DateTime<Utc> = SystemTime::now().into();

  let img: DynamicImage = image::open(icon_path).expect("Unable to open profile icon");
  let mut buf: Vec<u8> = vec![];
  img
    .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
    .expect("Unable to write profile icon to buffer");
  let res_base64: String = base64::encode(&buf);

  let profile: Value = serde_json::json!({
    "created": now.to_rfc3339(),
    "gameDir": application_dir,
    "lastUsed": now.to_rfc3339(),
    "lastVersionId": &modpack.id,
    "type": "custom",
    "icon": format!("data:image/png;base64,{}", res_base64),
    "name": &modpack.name
  });
  let obj: &mut Map<String, Value> = profiles
    .get_mut("profiles")
    .expect("Unable to get profiles from json")
    .as_object_mut()
    .expect("Unable to make profile mutable");
  obj.insert(modpack.id.to_owned(), profile);

  let profiles_file_write: File = File::create(gamepath.join("launcher_profiles.json"))
    .expect("Unable to open launcher_profiles for writing");
  serde_json::to_writer_pretty(profiles_file_write, &profiles).expect("Unable to write profiles");

  Ok(())
}

async fn download_file(url: &str, path: &PathBuf, window: &tauri::Window, emit_progress: bool) {
  let resp: Response = reqwest::get(url)
    .await
    .expect(format!("Error downloading file from {}", url).as_str());
  let mut stream = resp.bytes_stream();
  let mut file: File = File::create(path)
    .expect(format!("Error creating file at {}", path.to_str().unwrap()).as_str());

  while let Some(item) = stream.next().await {
    let chunk = item.expect(format!("Error downloading file from {}", url).as_str());
    file
      .write(&chunk)
      .expect(format!("Error writing to file at {}", path.to_str().unwrap()).as_str());

    if emit_progress {
      window
        .emit(
          "install-progress",
          ProgressPayload {
            bytes: chunk.len() as u64,
          },
        )
        .expect("Error while transmitting install progress");
    }
  }
}

async fn get_mrpack(
  modpack: &Modpack,
  modpack_dir: &PathBuf,
  window: &tauri::Window,
) -> Result<File> {
  let modpack_file_name: String = format!("{}-{}.mrpack", modpack.id, modpack.version);
  let modpack_url: String = format!("{}/{}", BASE_URL, modpack_file_name);
  let modpack_zip_path: PathBuf = modpack_dir.join(&modpack_file_name);
  download_file(&modpack_url, &modpack_zip_path, &window, false).await;

  let modpack_file = File::open(&modpack_zip_path).expect("Unable to open modpack file");

  Ok(modpack_file)
}

#[tauri::command]
async fn install_modpack(
  modpack: serde_json::Value,
  gamepath: String,
  window: tauri::Window,
  handle: tauri::AppHandle,
) -> Result<(), tauri::Error> {
  let modpack: Modpack = serde_json::from_value(modpack).expect("Unable to map modpack data");
  let gamepath: &Path = Path::new(&gamepath);

  let modpack_dir: PathBuf = gamepath.join(".vloedje").join(&modpack.id);
  fs::create_dir_all(&modpack_dir).expect("Unable to create modpack folder");

  let pack_cache_dir: &PathBuf = &modpack_dir.join(".pack");

  if pack_cache_dir.is_dir() && pack_cache_dir.starts_with(gamepath) {
    fs::remove_dir_all(pack_cache_dir).expect("Unable to clear cache dir");
  }

  let modpack_file: File = get_mrpack(&modpack, &modpack_dir, &window)
    .await
    .expect("Unable to download modpack file");
  let modrinthpack: ModrinthPack =
    ModrinthPack::from_mrpack(modpack_file, &modpack_dir).expect("Unable to extract modpack");
  let manifest: ModrinthManifest = modrinthpack
    .get_manifest()
    .expect("Unable to read modpack index");
  let manifest_files = &manifest.files;

  let total_filesize = manifest_files.into_iter().map(|x| x.file_size).sum();
  window
    .emit(
      "total-filesize",
      TotalFileSizePayload {
        total_bytes: total_filesize,
      },
    )
    .unwrap();

  install_loader(
    &modpack.id,
    &manifest.dependencies.minecraft,
    &manifest.dependencies.quilt_loader,
    &gamepath,
  )
  .await
  .expect("Unable to install loader");

  let profile_icon: PathBuf = handle
    .path_resolver()
    .resolve_resource("resources/profile.png")
    .expect("Unable to resolve profile icon");

  install_profile(&gamepath, &modpack, &modpack_dir, &profile_icon)
    .await
    .expect("Unable to install profile");

  let has_prev_paths = modpack_dir.join(".changed_paths").is_file();

  if has_prev_paths {
    let prev_paths: Vec<String> = fs::read_to_string(&modpack_dir.join(".changed_paths"))
      .expect("Error reading changed paths")
      .split("\n")
      .map(|s| s.to_string())
      .collect();

    for file in WalkDir::new(&modpack_dir)
      .into_iter()
      .filter_map(|f| f.ok())
    {
      if file.metadata().unwrap().is_file() {
        let path: &Path = file
          .path()
          .strip_prefix(&modpack_dir)
          .expect("Path not in modpack directory");
        let path_str: &str = path.to_str().expect("cannot convert path to str");
        let pack_cache_dir: &PathBuf = &modpack_dir.join(".pack");

        let is_in_prev_paths: bool = prev_paths.contains(&path_str.to_owned());
        let is_in_manifest: bool = manifest_files
          .into_iter()
          .find(|&f| f.path == path_str)
          .is_some();
        let is_in_overrides: bool = pack_cache_dir.join("overrides").join(path).is_file();
        let is_in_client_overrides: bool =
          pack_cache_dir.join("client-overrides").join(path).is_file();

        if is_in_prev_paths && !(is_in_manifest || is_in_overrides || is_in_client_overrides) {
          println!("{}", &path_str);

          fs::remove_file(&modpack_dir.join(path)).expect("Unable to remove old file");
        }
      }
    }
  }

  let mut changed_paths: Vec<String> = vec![];

  for file in manifest_files.into_iter() {
    changed_paths.push(file.path.to_owned());

    let file_path = &modpack_dir.join(file.path.to_owned());
    if file_path.starts_with(&modpack_dir) && file.env.client != ModrinthFileEnvTypes::Unsupported {
      fs::create_dir_all(file_path.parent().unwrap()).expect("Error creating parent directory");
      download_file(&file.downloads[0], file_path, &window, true).await;
    }
  }

  let override_paths: [&str; 2] = ["overrides", "client-overrides"];

  for override_path in override_paths.into_iter() {
    for file in WalkDir::new(&modpack_dir.join(".pack").join(override_path))
      .into_iter()
      .filter_map(|f| f.ok())
    {
      if file.metadata().unwrap().is_file() {
        let path = file
          .path()
          .strip_prefix(&modpack_dir.join(".pack").join(override_path))
          .expect("this is awkward");
        changed_paths.push(path.to_str().unwrap().to_owned());

        fs::create_dir_all(modpack_dir.join(path).parent().unwrap())
          .expect("Error creating parent directory");

        fs::copy(file.path(), modpack_dir.join(path)).expect("Unable to copy override file");
      }
    }
  }

  fs::write(
    &modpack_dir.join(".changed_paths"),
    changed_paths.join("\n"),
  )
  .expect("Error writing changed paths");

  window
    .emit(
      "install-done",
      FinishedPayload {
        finished: true,
        errors: false,
      },
    )
    .unwrap();

  Ok(())
}

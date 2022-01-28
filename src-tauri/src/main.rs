#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use futures_util::StreamExt;
use std::io::Write;
use image::io::Reader as ImageReader;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_minecraft_directory,
      directory_exists,
      path_join,
      download_modpack
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

#[derive(serde::Serialize)]
struct ProgressPayload {
  current_mod: String,
  mod_index: usize,
  progress: u64,
}

#[derive(serde::Serialize)]
struct FinishedPayload {
  finished: bool,
  errors: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Mod {
  id: String,
  name: String,
  filename: String,
  state: String,
  url: String,
  hash: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct ModConfig {
  id: String,
  name: String,
  filename: String,
  state: String,
  url: String,
  hash: String,
}


#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Modpack {
  id: String,
  name: String,
  description: String,
  minecraftVersion: String,
  loaderVersion: String,
  mods: Vec<Mod>,
  configs: Vec<ModConfig>,
}

async fn install_loader(minecraft_version: &String, loader_version: &String, gamepath: &std::path::Path) {
  
  let version_string = format!("twilight-{}-{}", loader_version, minecraft_version);
  let version_dir = gamepath.join("versions").join(&version_string);
  std::fs::create_dir_all(&version_dir).unwrap();

  let profile_resp = reqwest::get(format!("https://modpack.vloedje.nl/profile/{}/{}.json", minecraft_version, loader_version)).await.unwrap().text().await.unwrap();
  let mut file = std::fs::File::create(version_dir.join(format!("{}.json", &version_string))).unwrap();
  std::io::copy(&mut profile_resp.as_bytes(), &mut file).unwrap();
}

async fn install_profile(gamepath: &std::path::Path, modpack: &Modpack, application_dir: &std::path::Path) {
  let profiles_file = std::fs::File::open(gamepath.join("launcher_profiles.json")).unwrap();
  let mut profiles: serde_json::Value = serde_json::from_reader(std::io::BufReader::new(profiles_file)).unwrap();
  let now: chrono::DateTime<chrono::Utc> = std::time::SystemTime::now().into();

  let context = tauri::generate_context!();
  let img_path = tauri::api::path::resolve_path(context.config(), context.package_info(), "resources/profile.png", Some(tauri::api::path::BaseDirectory::Resource)).unwrap();
  println!("{}", img_path.to_string_lossy());
  let img = ImageReader::open(img_path.to_string_lossy().to_string()).unwrap().decode().unwrap();
  let mut buf = vec![];
  img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
  let res_base64 = base64::encode(&buf);

  let profile = serde_json::json!({
    "created": now.to_rfc3339(),
    "gameDir": application_dir,
    "lastUsed": now.to_rfc3339(),
    "lastVersionId": format!("twilight-{}-{}", &modpack.loaderVersion, &modpack.minecraftVersion),
    "type": "custom",
    "icon": format!("data:image/png;base64,{}", res_base64),
    "name": &modpack.name
  });
  let obj = profiles.get_mut("profiles").unwrap().as_object_mut().unwrap();
  obj.insert(modpack.id.clone(), profile);

  serde_json::to_writer(std::fs::File::create(gamepath.join("launcher_profiles.json")).unwrap(), &profiles).unwrap();
}

async fn download_mod(mods_dir: &std::path::Path, mcmod: &Mod, window: &tauri::Window, index: usize) {
  let resp = reqwest::get(&mcmod.url).await.unwrap();
  let total_size = resp.content_length().unwrap();
  let mut stream = resp.bytes_stream();
  let mut file = std::fs::File::create(mods_dir.join(&mcmod.filename)).unwrap();
  let mut downloaded: u64 = 0;

  while let Some(item) = stream.next().await {
    let chunk = item.unwrap();
    file.write(&chunk).unwrap();
    downloaded = downloaded + (chunk.len() as u64);

    let percentage: f64 = (downloaded as f64) / (total_size as f64) * 100.0;

    window
    .emit(
      "install-progress",
      ProgressPayload {
        current_mod: mcmod.name.clone(),
        mod_index: index + 1,
        progress: percentage as u64,
      },
    )
    .unwrap();
  }

}

async fn configure_mod(config_dir: &std::path::Path, modconfig: &ModConfig) {
  let resp = reqwest::get(&modconfig.url).await.unwrap();

  let file_path = config_dir.join(std::path::Path::new(&modconfig.filename));
  std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();

  let mut file = std::fs::File::create(file_path).unwrap();

  let content = resp.text().await.unwrap();
  std::io::copy(&mut content.as_bytes(), &mut file).unwrap();
}

#[tauri::command]
async fn download_modpack(modpack: serde_json::Value, gamepath: String, window: tauri::Window) {
  let modpack: Modpack = serde_json::from_value(modpack).unwrap();
  let gamepath = std::path::Path::new(&gamepath);

  install_loader(&modpack.minecraftVersion, &modpack.loaderVersion, &gamepath).await;
  
  let application_dir = gamepath.join(".twilight").join(&modpack.id);
  std::fs::create_dir_all(&application_dir).unwrap();

  install_profile(&gamepath, &modpack, &application_dir).await;
  
  let mods_dir = application_dir.join("mods");
  std::fs::create_dir_all(&mods_dir).unwrap();

  let config_dir = application_dir.join("config");
  std::fs::create_dir_all(&config_dir).unwrap();

  tauri::async_runtime::spawn(async move {
    for (i, mcmod) in modpack.mods.iter().enumerate() {
      download_mod(&mods_dir, &mcmod, &window, i).await;
    }

    for (_i, modconfig) in modpack.configs.iter().enumerate() {
      configure_mod(&config_dir, &modconfig).await;
    }

    window
      .emit(
        "install-done",
        FinishedPayload {
          finished: true,
          errors: false,
        },
      )
      .unwrap();
  });
}

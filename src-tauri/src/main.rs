#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

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
  progress: i32,
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

#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Modpack {
  id: String,
  name: String,
  description: String,
  minecraftVersion: String,
  loaderVersion: String,
  mods: Vec<Mod>,
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
  let profile = serde_json::json!({
    "created": now.to_rfc3339(),
    "gameDir": application_dir,
    "lastUsed": now.to_rfc3339(),
    "lastVersionId": format!("twilight-{}-{}", &modpack.loaderVersion, &modpack.minecraftVersion),
    "type": "custom",
    "name": &modpack.name
  });
  let obj = profiles.get_mut("profiles").unwrap().as_object_mut().unwrap();
  obj.insert(modpack.id.clone(), profile);

  serde_json::to_writer(std::fs::File::create(gamepath.join("launcher_profiles.json")).unwrap(), &profiles).unwrap();
}

#[tauri::command]
async fn download_modpack(modpack: serde_json::Value, gamepath: String, window: tauri::Window) {
  let modpack: Modpack = serde_json::from_value(modpack).unwrap();
  let gamepath = std::path::Path::new(&gamepath);

  install_loader(&modpack.minecraftVersion, &modpack.loaderVersion, &gamepath).await;
  
  let application_dir = gamepath.join(".twilight").join(&modpack.id);
  std::fs::create_dir_all(&application_dir).unwrap();

  install_profile(&gamepath, &modpack, &application_dir).await;
  

  tauri::async_runtime::spawn(async move {
    for (i, mcmod) in modpack.mods.iter().enumerate() {
      for n in 1..=100 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        window
          .emit(
            "install-progress",
            ProgressPayload {
              current_mod: mcmod.name.clone(),
              mod_index: i + 1,
              progress: n,
            },
          )
          .unwrap();
      }
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

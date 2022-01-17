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

#[tauri::command]
async fn download_modpack(data: serde_json::Value, window: tauri::Window) {
  let modpack: Modpack = serde_json::from_value(data).unwrap();

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

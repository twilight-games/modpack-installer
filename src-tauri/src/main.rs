#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_minecraft_directory, directory_exists, path_join])
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
    },
    "windows" => {
      let minecraft_path = std::path::Path::new("AppData\\Roaming\\.minecraft");
      let minecraft_dir = home_dir_path.join(minecraft_path);

      minecraft_dir.to_string_lossy().into()
    },
    "macos" => {
      let minecraft_path = std::path::Path::new("Library/Application Support/minecraft");
      let minecraft_dir = home_dir_path.join(minecraft_path);

      minecraft_dir.to_string_lossy().into()
    },
    _ => {
      "".into()
    }
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
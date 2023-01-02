use std::{fs::{File, self}, path::{PathBuf}};
use serde::{Serialize, Deserialize};
use zip::ZipArchive;
use anyhow::{Result};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ModrinthFileHashes {
  pub sha1: String,
  pub sha512: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ModrinthFileEnvTypes {
  #[serde(alias = "required")]
  Required,
  #[serde(alias = "optional")]
  Optional,
  #[serde(alias = "unsupported")]
  Unsupported
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ModrinthFileEnv {
  pub client: ModrinthFileEnvTypes,
  pub server: ModrinthFileEnvTypes,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ModrinthFile {
  pub path: String,
  pub downloads: Vec<String>,
  #[serde(alias = "fileSize")]
  pub file_size: u64,
  pub env: ModrinthFileEnv
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ModrinthDependencies {
  pub minecraft: String,
  #[serde(alias = "quilt-loader")]
  pub quilt_loader: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ModrinthManifest {
  #[serde(alias = "formatVersion")]
  pub format_version: u8,
  pub game: String,
  #[serde(alias = "versionId")]
  pub version_id: String,
  pub name: String,
  pub files: Vec<ModrinthFile>,
  pub dependencies: ModrinthDependencies,
}
pub struct ModrinthPack {
    path: PathBuf
}

impl ModrinthPack {
    pub fn from_mrpack(file: File, path: &PathBuf) -> Result<ModrinthPack> {
        let mut archive: ZipArchive<File> = ZipArchive::new(file).expect("");
        let path: PathBuf = path.join(".pack");
        std::fs::create_dir_all(&path).expect("Unable to create modpack folder");
        archive.extract(&path).expect("Unable to extract modpack");

        Ok(ModrinthPack {
            path
        })
    }

    pub fn get_manifest(&self) -> Result<ModrinthManifest> {
      let manifest_string = fs::read_to_string(&self.path.join("modrinth.index.json")).expect("Unable to read manifset");
      let modpack_manifest: ModrinthManifest = serde_json::from_str(&manifest_string).expect("Unable to map data");

      Ok(modpack_manifest)
    }

}
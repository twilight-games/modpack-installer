import { fetch } from "@tauri-apps/api/http";

export interface Mod {
  id: string;
  name: string;
  filename: string;
  state: string;
  url: string;
  hash: string;
}

export interface ModConfig {
  id: string;
  name: string;
  filename: string;
  state: string;
  url: string;
  hash: string;
}

export interface Modpack {
  id: string;
  name: string;
  description: string;
  minecraftVersion: string;
  loaderVersion: string;
  mods: Mod[];
  configs: ModConfig[],
}

export interface ModpackRequest {
  modpacks: [];
}

export const rawDataToModpackObject = (data: any): Modpack => ({
  id: data.id,
  name: data.name,
  description: data.description,
  minecraftVersion: data.minecraft_version,
  loaderVersion: data.loader_version,
  mods: (data.mods || []).map(rawDataToModObject),
  configs: (data.configs || []).map(rawDataToModConfigObject),
});

export const rawDataToModObject = (data: any): Mod => ({
  id: data.id,
  name: data.name,
  filename: data.filename,
  state: data.state,
  url: data.url,
  hash: data.hash,
});

export const rawDataToModConfigObject = (data: any): ModConfig => ({
  id: data.id,
  name: data.name,
  filename: data.filename,
  state: data.state,
  url: data.url,
  hash: data.hash,
});

export default async (): Promise<Modpack[]> => {
  const result = await fetch<ModpackRequest>('https://modpack.vloedje.nl/test.json');
  console.log(result);
  const { modpacks } = result.data;

  return modpacks.map(rawDataToModpackObject);
}
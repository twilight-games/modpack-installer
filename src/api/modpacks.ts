import { invoke } from "@tauri-apps/api";
import { fetch } from "@tauri-apps/api/http";
import { Modpack } from "./types";

const BASE_URL = "https://modpack.vloedje.nl";

interface getModpackResponse {
    modpacks: Modpack[],
}

export const getModpacks = async (): Promise<Modpack[]> => {
    const result = await fetch<getModpackResponse>(BASE_URL + '/modpacks.json')

    if (!result.ok) {
        throw Error('Unable to reach modpacks api: error code ' + result.status)
    }

    const { modpacks } = result.data

    if (modpacks.length == 0) {
        throw Error('Unable to fetch modpacks: api returned empty array')
    }
  
    return modpacks;
}

export const downloadModpack = async (data: Modpack, path: String) => {
    await invoke('install_modpack', {
        modpack: data,
        gamepath: path
    });
}
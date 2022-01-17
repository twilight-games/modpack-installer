import { invoke } from '@tauri-apps/api';
import { Modpack } from './getModpacks';

export default async (data: Modpack, path: String) => {
    invoke('download_modpack', {
        modpack: data,
        gamepath: path
    });
}
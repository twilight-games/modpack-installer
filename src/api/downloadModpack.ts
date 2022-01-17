import { invoke } from '@tauri-apps/api';
import { Modpack } from './getModpacks';

export default async (data: Modpack) => {
    invoke('download_modpack', {
        data: data
    });
}
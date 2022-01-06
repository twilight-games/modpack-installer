import {fetch, Response, ResponseType} from '@tauri-apps/api/http'

export interface Mod {
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
});

export const rawDataToModObject = (data: any): Mod => ({
    id: data.id,
    name: data.name,
    filename: data.filename,
    state: data.state,
    url: data.url,
    hash: data.hash,
});

export default (): Promise<Modpack[]> => {
    return new Promise((resolve, reject) => {
        fetch<any>('https://modpack.vloedje.nl/test.json')
        .then(({ data }) => resolve(
            data.modpacks.map(rawDataToModpackObject),
        ))
        .catch(reject);
    });
};
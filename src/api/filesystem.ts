import { fs, invoke } from "@tauri-apps/api";

export const isDirectory = async (directory: string): Promise<boolean> => {
    try {
        await fs.readDir(directory);
        
    } catch(e) {
        console.log(e);
        return false;
    }
    return true;
    
}

export const isMinecraftDirectory = async (directory: string): Promise<boolean> => {
    try {
        const entries = await fs.readDir(directory);
        
        if (!entries.find(x => x.name == "launcher_profiles.json")) return false;
        if (!entries.find(x => x.name == "versions" && x.children)) return false;

        return true;
    } catch (e) {
        console.log(e)
        return false;
    }
    
}

export const getMinecraftDirectory = (): Promise<string> => {
    return invoke('get_minecraft_directory');
}
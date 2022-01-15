import { path } from '@tauri-apps/api';

export default async (directory: string): Promise<boolean> => {
    try {
        await path.resolve(directory);
        await path.resolve(directory, 'versions');
        await path.resolve(directory, 'launcher_profiles.json');
    } catch {
        return false;
    }
    return true;
    
}
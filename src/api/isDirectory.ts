import { path } from '@tauri-apps/api';

export default async (directory: string): Promise<boolean> => {
    try {
        await path.resolve(directory);
    } catch {
        return false;
    }
    return true;
    
}
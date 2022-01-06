import { invoke } from '@tauri-apps/api';

export default (): Promise<string> => {
    return invoke('get_minecraft_directory');
}
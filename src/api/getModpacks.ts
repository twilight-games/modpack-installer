import { fetch, Response, ResponseType } from "@tauri-apps/api/http";
import { reactive, Ref, ref, toRefs } from "vue";

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

export default (): {
  result: Ref<Modpack[]>;
  error: Ref<any>;
  isLoading: Ref<boolean>;
} => {
  const state = reactive({
    isLoading: false,
    error: null as any,
  });

  const result = ref<Modpack[]>([]);
  state.isLoading = true;
  fetch<any>("https://modpack.vloedje.nl/test.json")
    .then(({ data }) => {
      result.value = data.modpacks.map(rawDataToModpackObject);
      console.log(result.value);
    })
    .catch((error) => {
      state.error = error;
    })
    .finally(() => {
      state.isLoading = false;
    });

  return {
    result,
    ...toRefs(state),
  };
};

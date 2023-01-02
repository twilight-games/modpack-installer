<script setup lang="ts">
import { dialog, path } from "@tauri-apps/api"
import { onMounted, ref } from "vue";
import { MagnifyingGlassIcon, FolderIcon } from '@heroicons/vue/20/solid'
import { getMinecraftDirectory, isMinecraftDirectory, isDirectory } from "../api/filesystem";
import { useModpackStore } from "../state/modpackStore";
const store = useModpackStore();


async function openDialog() {
    const dir: string|string[]|null = await dialog.open({
        multiple: false,
        directory: true,
        defaultPath: (await isDirectory(store.gamePath)) ? store.gamePath : (await path.homeDir()),
    });

    if (typeof dir === "string") {
        store.gamePath = dir;
    }       
}

const isChecking = ref<boolean>(false);

onMounted(async () => {
    if (store.gamePath == '') {
        const minecraftDir = await getMinecraftDirectory();
        store.gamePath = minecraftDir;
    }
})

async function validateAndContinue() {
    isChecking.value = true;
    store.error = undefined;

    const isMinecraftDir: boolean = await isMinecraftDirectory(store.gamePath);
    if (!isMinecraftDir) {
        isChecking.value = false;
        store.error = 'This directory is not a valid Minecraft folder.'
        return;
    }

    isChecking.value = false;

    store.nextStep()
}

</script>

<template>
    <div>
        <label
            for="email"
            class="block text-sm font-medium text-gray-200"
        >Minecraft Game Directory (.minecraft)</label>
        <div class="mt-1 flex rounded-md shadow-sm">
            <div class="relative flex items-stretch flex-grow focus-within:z-10">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                    <FolderIcon class="h-5 w-5 text-gray-500" aria-hidden="true" />
                </div>
                <input
                    v-model="store.gamePath"
                    name="path"
                    id="path"
                    disabled
                    class="focus:ring-teal-500 focus:border-teal-500 block w-full rounded-none rounded-l-md pl-10 sm:text-sm text-neutral-200 border border-neutral-500 bg-neutral-700"
                />
            </div>
            <button
                @click="openDialog"
                type="button"
                class="-ml-px relative inline-flex items-center space-x-2 px-4 py-2 border border-neutral-500 text-sm font-medium rounded-r-md text-neutral-200 bg-neutral-700 hover:bg-neutral-600 focus:outline-none focus:ring-1 focus:ring-teal-500 focus:border-teal-500"
            >
                <MagnifyingGlassIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
                <span>Select</span>
            </button>
        </div>
    </div>
        <div class="flex justify-end">
                    <button
                        :disabled="store.gamePath === '' || isChecking"
                        type="button"
                        @click="validateAndContinue"
                        class="items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-teal-100 bg-teal-900 hover:bg-teal-800 disabled:text-neutral-300 disabled:bg-neutral-700 disabled:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-700"
                    >Continue</button>
                </div>
</template>
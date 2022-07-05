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
                    v-model="gamePath"
                    type="email"
                    name="email"
                    id="email"
                    class="focus:ring-teal-500 focus:border-teal-500 block w-full rounded-none rounded-l-md pl-10 sm:text-sm text-neutral-200 border border-neutral-500 bg-neutral-700"
                />
            </div>
            <button
                @click="openDialog"
                type="button"
                class="-ml-px relative inline-flex items-center space-x-2 px-4 py-2 border border-neutral-500 text-sm font-medium rounded-r-md text-neutral-200 bg-neutral-700 hover:bg-neutral-600 focus:outline-none focus:ring-1 focus:ring-teal-500 focus:border-teal-500"
            >
                <SearchIcon class="h-5 w-5 text-gray-500" aria-hidden="true" />
                <span>Select</span>
            </button>
        </div>
        <p v-if="errorMessage" v-text="errorMessage" class="text-white mt-1 font-mono"></p>
    </div>
        <div class="flex justify-end">
                    <button
                        :disabled="gamePath === '' || isChecking"
                        type="button"
                        @click="nextStep"
                        class="items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-teal-100 bg-teal-900 hover:bg-teal-800 disabled:text-neutral-300 disabled:bg-neutral-700 disabled:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-700"
                    >Continue</button>
                </div>
</template>

<script setup lang="ts">
import { dialog, fs, os, path } from "@tauri-apps/api"
import { computed, onMounted, ref } from "vue";
import { SearchIcon, FolderIcon } from '@heroicons/vue/solid'
import isMinecraftDirectory from "../api/isMinecraftDirectory";
import isDirectory from "../api/isDirectory";
import getMinecraftDirectory from "../api/getMinecraftDirectory";

const props = defineProps<{
    modelValue: string
}>()

const emit = defineEmits(['update:modelValue', 'next', 'alert'])

const gamePath = computed({
    get: () => props.modelValue,
    set: (value) => emit('update:modelValue', value)
});

async function openDialog() {
    const dir: string|string[]|null = await dialog.open({
        multiple: false,
        directory: true,
        defaultPath: (await isDirectory(gamePath.value)) ? gamePath.value : (await path.homeDir()),
    });

    if (typeof dir === "string") {
        gamePath.value = dir;
    }       
}

const errorMessage = ref<string|null>(null);
const isChecking = ref<boolean>(false);

onMounted(async () => {
    gamePath.value = (await getMinecraftDirectory()) ?? '';
})

async function nextStep() {
    isChecking.value = true;
    emit('alert', null);

    const isMinecraftDir: boolean = await isMinecraftDirectory(gamePath.value);
    if (!isMinecraftDir) {
        emit('alert', 'This directory is not a valid Minecraft folder.')
    }

    isChecking.value = false;

    emit('next');
}

</script>
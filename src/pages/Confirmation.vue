<template>
    <div class="space-y-4">
        <div class="flex justify-between bg-neutral-800 px-4 py-2 rounded-md">
            <div>
                <h3 class="text-gray-300 text-xs mb-1">Modpack</h3>
                <span class="text-gray-100" v-text="store.selectedModpack?.name" />
            </div>
            <button class="text-teal-600 hover:text-teal-500 ml-4" @click="store.step = 0">Edit</button>
        </div>
        <div class="flex justify-between bg-neutral-800 px-4 py-2 rounded-md">
            <div>
                <h3 class="text-gray-300 text-xs mb-1">Minecraft directory</h3>
                <span class="text-gray-100 font-mono text-sm" v-text="store.gamePath" />
            </div>
            <button class="text-teal-600 hover:text-teal-500 ml-4" @click="store.step = 1">Edit</button>
        </div>
        <div class="flex justify-end">
            <button
                type="button"
                @click="store.nextStep"
                class="items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-teal-100 bg-teal-900 hover:bg-teal-800 disabled:text-neutral-300 disabled:bg-neutral-700 disabled:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-700"
            >Install</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { downloadModpack } from "../api/modpacks";
import { useModpackStore } from "../state/modpackStore";

const store = useModpackStore();



</script>
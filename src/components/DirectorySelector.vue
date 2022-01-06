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
    </div>
</template>

<script setup lang="ts">
import { dialog, fs, os } from "@tauri-apps/api"
import { computed, ref } from "vue";
import { SearchIcon, FolderIcon } from '@heroicons/vue/solid'

const props = defineProps<{
    modelValue: string
}>()

const emit = defineEmits(['update:modelValue'])

const gamePath = computed({
    get: () => props.modelValue,
    set: (value) => emit('update:modelValue', value)
});

async function openDialog() {
    let defaultPath = undefined;
    try {
        const dir = await fs.readDir(gamePath.value);
        if (dir.length > 0) {
            defaultPath = gamePath.value;
        }
    } catch { }

    dialog.open({
        multiple: false,
        directory: true,
        defaultPath: defaultPath,
    }).then(path => {
        if (typeof path === "string") {
            gamePath.value = path;
        } else {
            gamePath.value = path[0];
        }
    })
}

</script>
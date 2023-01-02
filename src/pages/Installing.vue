<template>
    <div class="space-y-2">
        <h2 class="text-white text-2xl text-center">
            Installing:
            <span v-text="store.selectedModpack?.name"></span>
        </h2>
        <div class="bg-gray-200 rounded-full overflow-hidden">
            <div class="h-2 bg-teal-600 rounded-full" v-bind:style="{ width: (total_bytes ? (bytes / total_bytes * 100) : 0 )+ '%' }" />
        </div>
        <span class="text-gray-300 text-xs tracking-wider font-medium" v-text="(total_bytes ? (bytes / total_bytes * 100).toFixed(0) : 0 )+ '%'"></span>
    </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { computed, onMounted, ref } from 'vue';
import { downloadModpack } from '../api/modpacks';
import { useModpackStore } from "../state/modpackStore";

const store = useModpackStore();


const bytes = ref<number>(0);
const total_bytes = ref<number|null>(null);


interface payload {
    bytes: number,
}

interface total_filesizepayload {
    total_bytes: number,
}

interface finishedpayload {
    finished: boolean,
    errors: boolean
}

onMounted(async () => {
    if (store.selectedModpack && store.gamePath) {
        try {
            downloadModpack(store.selectedModpack, store.gamePath);
        } catch(error: unknown) {
            store.error = (error as Error).message;
        }
    }
    await listen('install-progress', event => {
        const payload = event.payload as payload;
        bytes.value = bytes.value + payload.bytes;
    })

    await listen('total-filesize', event => {
        const payload = event.payload as total_filesizepayload;
        total_bytes.value = payload.total_bytes;
    })

    await listen('install-done', event => {
        const payload = event.payload as finishedpayload;
        if (payload.finished && !payload.errors) {
            store.nextStep()
        }
    })
});

</script>
<template>
    <div class="space-y-2">
        <h2 class="text-white text-2xl text-center">
            Installing:
            <span v-text="selectedModpack.name"></span>
        </h2>
        <div class="bg-gray-200 rounded-full overflow-hidden">
            <div class="h-2 bg-teal-600 rounded-full" v-bind:style="{ width: progress + '%' }" />
        </div>
        <p class="text-sm text-center text-neutral-100" v-text="currentMod"></p>
    </div>
</template>

<script setup lang="ts">
import { Modpack } from '../api/getModpacks';
import { listen } from '@tauri-apps/api/event';
import downloadModpack from '../api/downloadModpack';
import { onMounted, ref } from 'vue';


const props = defineProps<{
    selectedModpack: Modpack,
}>()

const currentMod = ref<string>('');
const progress = ref<Number>(0);

const emit = defineEmits(['navigate', 'next'])

interface payload {
    current_mod: string,
    progress: Number,
}

interface finishedpayload {
    finished: boolean,
    errors: boolean
}

onMounted(async () => {
    await listen('install-progress', event => {
        console.log(event.payload);
        const payload = event.payload as payload;
        currentMod.value = payload.current_mod;
        progress.value = payload.progress;
    })

    await listen('install-done', event => {
        console.log(event.payload);
        const payload = event.payload as finishedpayload;
        if (payload.finished && !payload.errors) {
            emit('next');
        }
    })
});

const install = async function () {


    downloadModpack(props.selectedModpack);
}

</script>
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import ModpackSelector from './ModpackSelector.vue';
import AlertBox from '../components/AlertBox.vue';
import DirectorySelector from './DirectorySelector.vue';
import { app } from '@tauri-apps/api';
import Confirmation from './Confirmation.vue';
import Installing from './Installing.vue';
import Finished from './Finished.vue';
import { useModpackStore } from '../state/modpackStore';

const store = useModpackStore();

const step = ref(0);

const steps = [
    ModpackSelector,
    DirectorySelector,
    Confirmation,
    Installing,
    Finished
];

const versionString = ref<string>('');

onMounted(async () => {
    versionString.value = (await app.getName()) + ' v' + (await app.getVersion());
})

function nextStep() {
    step.value++;
}

</script>
<template>
    <!-- This example requires Tailwind CSS v2.0+ -->
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <!-- We've used 3xl here, but feel free to try other max-widths based on your needs -->
        <div class="max-w-3xl mx-auto flex justify-center h-screen items-center">
            <div class="flex flex-col space-y-8">
                <AlertBox :error="store.error" v-if="store.error" />
                <div class="flex items-center justify-center">
                    <img src="../assets/logo.jpg" class="w-24" />
                    <h1 class="text-white font-semibold ml-8 text-3xl">Modpack Installer</h1>
                </div>

                <component :is="steps[store.step]" />

            </div>
        </div>
    </div>
    <span
        class="absolute bottom-0 right-0 mr-4 mb-4 text-right text-gray-200 text-xs font-mono"
        v-text="versionString"
    />
</template>

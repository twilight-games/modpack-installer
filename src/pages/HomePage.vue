<script setup lang="ts">
import { onMounted, ref } from 'vue';
import getModpacks, { Modpack } from '../api/getModpacks';
import ModpackSelector from '../components/ModpackSelector.vue';
import LoadingIcon from '../components/LoadingIcon.vue';
import AlertBox from '../components/AlertBox.vue';
import DirectorySelector from '../components/DirectorySelector.vue';
import { app } from '@tauri-apps/api';
import getMinecraftDirectory from '../api/getMinecraftDirectory';


const { result: modpacks, isLoading: modpacksLoading, error: modpacksError } = getModpacks();
const selectedModpack = ref<Modpack | null>(null);
const selectedGamePath = ref<string>('');
const step = ref(0);

getMinecraftDirectory().then(path => {
    path = path ?? '';
    selectedGamePath.value = path;
})

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
                <AlertBox :error="modpacksError" v-if="modpacksError" />
                <div class="flex items-center">
                    <img src="../assets/logo.jpg" class="w-24" />
                    <h1 class="text-white font-semibold ml-8 text-3xl">Modpack Installer</h1>
                </div>

                <ModpackSelector :modpacks="modpacks" v-model="selectedModpack" @next="nextStep" v-if="step == 0" />
                <DirectorySelector v-model="selectedGamePath" @next="nextStep" v-if="step == 1" />

                <div class="justify-center flex" v-if="modpacksLoading">
                    <LoadingIcon />
                </div>

                
            </div>
        </div>
    </div>
    <span class="absolute bottom-0 right-0 mr-4 mb-4 text-right text-gray-200 text-xs font-mono" v-text="versionString" />
</template>

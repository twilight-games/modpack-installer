<script setup lang="ts">
import { ref } from 'vue';
import getModpacks, { Modpack } from '../api/getModpacks';
import ModpackSelector from './ModpackSelector.vue';
import LoadingIcon from './LoadingIcon.vue';
import AlertBox from './AlertBox.vue';


const { result: modpacks, isLoading: modpacksLoading, error: modpacksError } = getModpacks();
const selectedModpack = ref<Modpack | null>(null);
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

                <ModpackSelector :modpacks="modpacks" v-model="selectedModpack" />

                <div class="justify-center flex" v-if="modpacksLoading">
                    <LoadingIcon />
                </div>

                <div class="flex justify-end">
                    <button
                        :disabled="modpacksError || selectedModpack == null"
                        type="button"
                        class="items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-teal-100 bg-teal-900 hover:bg-teal-800 disabled:text-neutral-300 disabled:bg-neutral-700 disabled:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-700"
                    >Verder</button>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import {ref} from 'vue';
import { RadioGroup, RadioGroupDescription, RadioGroupLabel, RadioGroupOption } from '@headlessui/vue'
import getModpacks, {Modpack} from '../api/getModpacks';

const modpacks = ref<Modpack[]>([]);
const selected = ref<Modpack | null>(null);

const fetchModpacks = async() => {
    modpacks.value = await getModpacks();
    selected.value = modpacks.value[0];
}

fetchModpacks();

</script>
<template>
    <!-- This example requires Tailwind CSS v2.0+ -->
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <!-- We've used 3xl here, but feel free to try other max-widths based on your needs -->
        <div class="max-w-3xl mx-auto flex justify-center h-screen items-center">
            <div class="flex flex-col space-y-8">
                <div class="flex items-center">
                    <img src="../assets/logo.jpg" class="w-24">
                    <h1 class="text-white font-semibold ml-8 text-3xl">Modpack Installer</h1>
                </div>

                <RadioGroup v-model="selected">
                    <RadioGroupLabel class="sr-only">
                    Privacy setting
                    </RadioGroupLabel>
                    <div class="bg-neutral-800 rounded-md -space-y-px">
                    <RadioGroupOption as="template" v-for="(modpack, modpackIndex) in modpacks" :key="modpack.id" :value="modpack" v-slot="{ checked, active }">
                        <div :class="[modpackIndex === 0 ? 'rounded-tl-md rounded-tr-md' : '', modpackIndex === modpacks.length - 1 ? 'rounded-bl-md rounded-br-md' : '', checked ? 'bg-teal-900 border-teal-600 z-10' : 'border-gray-600', 'relative border p-4 flex cursor-pointer focus:outline-none']">
                        <span :class="[checked ? 'bg-teal-600 border-transparent' : 'bg-white border-gray-300', active ? 'ring-2 ring-offset-2 ring-teal-500' : '', 'h-4 w-4 mt-0.5 cursor-pointer rounded-full border flex items-center justify-center']" aria-hidden="true">
                            <span class="rounded-full bg-white w-1.5 h-1.5" />
                        </span>
                        <div class="ml-3 flex flex-col">
                            <RadioGroupLabel as="span" :class="[checked ? 'text-white' : 'text-gray-400', 'block text-sm font-medium']">
                            {{ modpack.name }}
                            </RadioGroupLabel>
                            <RadioGroupDescription as="span" :class="[checked ? 'text-gray-200' : 'text-gray-500', 'block text-sm']">
                            {{ modpack.description }}
                            </RadioGroupDescription>
                        </div>
                        </div>
                    </RadioGroupOption>
                    </div>
                </RadioGroup>
                
                <div class="flex justify-end">
                    <button type="button" class="items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-teal-100 bg-teal-900 hover:bg-teal-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-700">
    Verder
  </button>
                </div>
            </div>
            
        </div>
    </div>
</template>
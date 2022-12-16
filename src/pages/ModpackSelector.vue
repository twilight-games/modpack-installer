<script setup lang="ts">
import LoadingIcon from "../components/LoadingIcon.vue";
import { RadioGroup, RadioGroupDescription, RadioGroupLabel, RadioGroupOption } from '@headlessui/vue'
import { useModpackStore } from "../state/modpackStore";
import { onMounted } from "vue";
import { getModpacks } from "../api/modpacks";
const store = useModpackStore();

onMounted(async () => {
    try {
        store.setModpacks(await getModpacks());
    } catch (error: unknown) {
        store.error = (error as Error).message;
    }
})

</script>

<template>
    <div v-show="store.modpacks.length > 0" class="space-y-4">
        <RadioGroup v-model="store.selectedModpackId">
            <RadioGroupLabel class="sr-only">Modpack selection</RadioGroupLabel>
            <div class="bg-neutral-800 rounded-md -space-y-px">
                <RadioGroupOption
                    as="template"
                    v-for="(modpack, modpackIndex) in store.modpacks"
                    :key="modpack.id"
                    :value="modpack.id"
                    v-slot="{ checked, active }"
                >
                    <div
                        :class="[modpackIndex === 0 ? 'rounded-tl-md rounded-tr-md' : '', modpackIndex === store.modpacks.length - 1 ? 'rounded-bl-md rounded-br-md' : '', checked ? 'bg-teal-900 border-teal-600 z-10' : 'border-gray-600', 'relative border p-4 flex cursor-pointer focus:outline-none']"
                    >
                        <span
                            :class="[checked ? 'bg-teal-600 border-transparent' : 'bg-white border-gray-300', active ? 'ring-2 ring-offset-2 ring-teal-500' : '', 'h-4 w-4 mt-0.5 cursor-pointer rounded-full border flex items-center justify-center']"
                            aria-hidden="true"
                        >
                            <span class="rounded-full bg-white w-1.5 h-1.5" />
                        </span>
                        <div class="ml-3 flex flex-col">
                            <RadioGroupLabel
                                as="span"
                                :class="[checked ? 'text-white' : 'text-gray-400', 'block text-sm font-medium']"
                            >{{ modpack.name }}</RadioGroupLabel>
                            <RadioGroupDescription
                                as="span"
                                :class="[checked ? 'text-gray-200' : 'text-gray-500', 'block text-sm']"
                            >{{ modpack.description }}</RadioGroupDescription>
                        </div>
                    </div>
                </RadioGroupOption>
            </div>
        </RadioGroup>
        <div class="flex justify-end">
            <button
                :disabled="store.selectedModpackId === null"
                type="button"
                @click="store.nextStep"
                class="items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-teal-100 bg-teal-900 hover:bg-teal-800 disabled:text-neutral-300 disabled:bg-neutral-700 disabled:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-700"
            >Continue</button>
        </div>
    </div>

    <div class="justify-center flex" v-if="store.modpacks.length == 0">
        <LoadingIcon />
    </div>
</template>>
<script setup lang="ts">
import { Modpack } from "../api/getModpacks";
import { RadioGroup, RadioGroupDescription, RadioGroupLabel, RadioGroupOption } from '@headlessui/vue'
import { computed } from "vue";


const props = defineProps<{
    modpacks: Modpack[],
    modelValue: Modpack | null
}>()

const emit = defineEmits(['update:modelValue'])

const selectedModpack = computed({
    get: () => props.modelValue,
    set: (value) => emit('update:modelValue', value)
});

</script>

<template>
    <RadioGroup v-model="selectedModpack">
        <RadioGroupLabel class="sr-only">Privacy setting</RadioGroupLabel>
        <div class="bg-neutral-800 rounded-md -space-y-px">
            <RadioGroupOption
                as="template"
                v-for="(modpack, modpackIndex) in modpacks"
                :key="modpack.id"
                :value="modpack"
                v-slot="{ checked, active }"
            >
                <div
                    :class="[modpackIndex === 0 ? 'rounded-tl-md rounded-tr-md' : '', modpackIndex === modpacks.length - 1 ? 'rounded-bl-md rounded-br-md' : '', checked ? 'bg-teal-900 border-teal-600 z-10' : 'border-gray-600', 'relative border p-4 flex cursor-pointer focus:outline-none']"
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
</template>>
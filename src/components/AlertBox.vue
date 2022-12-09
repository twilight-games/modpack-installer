<template>
    <div class="rounded-md bg-red-800 p-4">
        <div class="flex">
            <div class="flex-shrink-0">
                <XCircleIcon class="h-5 w-5 text-red-200" aria-hidden="true" />
            </div>
            <div class="ml-3">
                <h3 class="text-sm font-medium text-red-100">An error occured:</h3>
                <p class="mt-2 text-sm text-white" v-text="error" />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { XCircleIcon } from '@heroicons/vue/24/solid';

const props = defineProps<{
    error: string;
}>()
</script>
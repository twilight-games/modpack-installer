<template>
    <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-teal-800">
        <CheckIcon class="h-10 w-10 text-teal-200" aria-hidden="true" />
    </div>
    <h2 class="text-center text-white text-2xl">Installation finished!</h2>

    <div class="flex justify-center">
        <button
            type="button"
            @click="close"
            class="items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-teal-100 bg-teal-900 hover:bg-teal-800 disabled:text-neutral-300 disabled:bg-neutral-700 disabled:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-700"
        >Close</button>
    </div>
</template>

<script setup lang="ts">
import { window } from '@tauri-apps/api';
import { CheckIcon } from '@heroicons/vue/24/outline'

const close = async function () {
    const w = await window.getCurrent();
    w.close();
}


</script>
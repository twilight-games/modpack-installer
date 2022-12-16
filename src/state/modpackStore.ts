import { defineStore } from "pinia";
import { computed, Ref, ref } from "vue";
import { Modpack } from "../api/types";

export const useModpackStore = defineStore('modpack', () => {
    const modpacks: Ref<Modpack[]> = ref([])
    const selectedModpackId: Ref<string | undefined> = ref()
    const selectedModpack = computed(() => modpacks.value.find(x => x.id === selectedModpackId?.value))
    const gamePath: Ref<string> = ref('')
    const step: Ref<number> = ref(0);
    const error: Ref<string | undefined> = ref();

    function setModpacks(packs: Modpack[]) {
        modpacks.value = packs;
        if(!selectedModpackId.value || !packs.find(x => x.id == selectedModpackId.value)) {
            selectedModpackId.value = packs[0].id
        }
    }

    function nextStep() {
        step.value++
    }

    return {modpacks, selectedModpack, gamePath, step, error, selectedModpackId, setModpacks, nextStep}
});
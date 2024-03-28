import { defineStore } from "pinia";

export const usePreferenciesStore = defineStore("preferencies", {
    state: () => ({ current_tab: null }),
});

import { createPinia, defineStore } from "pinia";

export const useMenuStore = defineStore('menuStore.js', {
state: () => ({
    downloadMenu: true,
    downloadProgress: 0,
    }),
actions: {
    setDownloadProgress(downloadProgress) {
        this.downloadProgress = downloadProgress;
    },
    getDownloadProgress() {
        return this.downloadProgress;
    },
    setDownloadMenu(downloadMenu) {
        this.downloadMenu = downloadMenu;
    },
    getDownloadMenu() {
        return this.downloadMenu;
    },
},});
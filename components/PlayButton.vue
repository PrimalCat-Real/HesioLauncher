<template>
    <div class="server-btn group hover:cursor-pointer" @click="execute()">
        <svg v-if="isDownloaded" class="group-hover:stroke-light-default transition-all duration-300 stroke-light-600 ml-1" width="20" height="24" viewBox="0 0 29 30" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M16.9876 22.8003L9.13878 27.1828C7.41555 28.1449 5.27895 27.954 3.75362 26.7015V26.7015C2.64345 25.79 2 24.429 2 22.9926V6.36057C2 4.7705 2.80033 3.28721 4.12931 2.4142V2.4142C5.4712 1.53272 7.16314 1.3904 8.6193 2.06663C13.1031 4.14888 22.7627 8.81803 26.0826 11.8736C26.5771 12.3287 27.0962 12.8028 27.3409 13.4287C27.8764 14.7989 27.2731 17.2054 22.1873 19.9873" class="stroke-inherit" stroke-width="3"/>
        </svg>
        <svg v-else class="group-hover:fill-light-default transition-all duration-300 fill-light-600 " width="20" height="20" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M2 16C1.45 16 0.979002 15.804 0.587002 15.412C0.195002 15.02 -0.000664969 14.5493 1.69779e-06 14V11H2V14H14V11H16V14C16 14.55 15.804 15.021 15.412 15.413C15.02 15.805 14.5493 16.0007 14 16H2ZM8 12L3 7L4.4 5.55L7 8.15V0H9V8.15L11.6 5.55L13 7L8 12Z" fill="inherit"/>
        </svg>

    </div>
</template>

<script>
// import { invoke } from '@tauri-apps/api/tauri'
import { useMenuStore } from "~/stores/MenuStore";
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { listen } from '@tauri-apps/api/event'
// import { http } from '@tauri-apps/api'
// import { download } from "tauri-plugin-upload-api";

export default {
    data(){
        return{
            progress: 0,
            menuStore: useMenuStore(),
            isDownloaded: false,
            gameFolder: null
        }
    },
    created() {  // or you can use the 'mounted' hook
        listen('download-progress', event => {
        this.progress = event.payload
        this.menuStore.setDownloadProgress(event.payload)
        // console.log('download progress', this.progress) 
        })
    },
    unmounted() { // stop listening when the component is unmounted
        stop('download-progress')
    },
    methods:{
        async execute(){
            this.menuStore.setDownloadMenu(true)
            await this.selectAndCreateDirectory()
            await invoke('create_directory', { path: this.gameFolder + "\\java" }).catch(err => console.error(err))
            await this.downloadFile('https://javadl.oracle.com/webapps/download/AutoDL?BundleId=248242_ce59cff5c23f4e2eaf4e778a117d4c5b', this.gameFolder + "\\java\\java8.exe")
            this.menuStore.setDownloadProgress(0)
            await invoke('create_directory', { path: this.gameFolder + "\\modpack" }).catch(err => console.error(err))
            // await this.selectAndCreateDirectory("modpack")
            await this.downloadFile('https://mediafilez.forgecdn.net/files/4480/750/nomi-ceu-client-1.6.zip', this.gameFolder + "\\modpack\\nomi-ceu-client-1.6.zip")
            await this.unzipFile(this.gameFolder + "\\modpack\\nomi-ceu-client-1.6.zip", this.gameFolder + "\\modpack")
            this.menuStore.setDownloadMenu(false)
        },

        async unzipFile(source, destination) {
            // const source = '/path/to/your/zip/file.zip'
            // const destination = '/path/to/extract/to'
            try {
                const response = await invoke('unzip_file', { source, destination })
                console.log(response) // Should log 'Files successfully extracted to: /path/to/extract/to'
            } catch (err) {
                console.error('Error unzipping file:', err)
            }
        },

        async selectAndCreateDirectory(dirName) {
            if(this.gameFolder != null && dirName === this.gameFolder){
                console.log(this.gameFolder);
            }else{
                console.log("cannot find path to folder");
                const path = await open({ directory: true })
                this.gameFolder = path
                // const newFolderPath = `${path}`
                // await invoke('create_directory', { path: newFolderPath }).catch(err => console.error(err))
                // const newFolderPath = `${path}/NewFolder`
                
                
            }
            
        },
        async downloadFile(url, dest) {
            
            console.log(dest);
             try {
                const response = await invoke('download_file', { args: { url, dest }});
                console.log(response); // Should log 'Download successful'
            } catch (err) {
                console.error('Error downloading file:', err);
            }
        }, 
    },
}
</script>

<style>

</style>
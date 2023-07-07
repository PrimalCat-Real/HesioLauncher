<template>
<div v-if="menuStore.getDownloadMenu()" class="fixed inset-0 bg-black/60" aria-hidden="true" />
<Dialog :open="menuStore.getDownloadMenu()" @close="setIsOpen" class="absolute z-50 sm:w-1/2 top-1/2 -translate-y-1/2 rounded-md">
    <DialogPanel>
      <DialogTitle>Deactivate account</DialogTitle>
      <DialogDescription>
        This will permanently deactivate your account
      </DialogDescription>
      <progress class="progress progress-secondary w-full" :value="menuStore.getDownloadProgress()" max="100"></progress>
      <p>
        Are you sure you want to deactivate your account? All of your data will be
        permanently removed. This action cannot be undone.
      </p>

      <button @click="menuStore.setDownloadMenu(false)">Deactivate</button>
    </DialogPanel>
</Dialog>
<div class="wrapper-col h-full sm:p-4">
  <div class="wrapper-row h-full">
    <ServerCard></ServerCard>
  </div>
</div>
  
</template>

<script>
import { useMenuStore } from "~/stores/MenuStore";
export default {
  data(){
    return{
      menuStore: useMenuStore()
    }
  }
}
</script>

<style>

</style>
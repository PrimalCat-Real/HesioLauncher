<template>
  <div class="flex flex-col items-center gap-2 sm:min-w-[200px] h-4/5 sm:px-6 sm:py-3 sm:min-h-[410px] sm:max-h-[410px] sm:max-w-[300px] xl:min-w-[400px] xl:min-h-[500px] xl:max-h-[600px] xl:rounded-xl bg-dark-200">
    <img src="~/assets/images/banner-vanila.png" alt="" srcset="" class="xl:w-3/4">
    <div class="wrapper-col gap-3 sm:gap-0 text-center sm:mt-2">
        <h1 class="title">Loft Vanilla</h1>
        <h2 class="subtitle-gradient">Server 1.19.4</h2>
    </div>
    <div class="wrapper-row sm:w-3/4 mt-auto mb-4">
      <div class="server-btn"></div>
      <PlayButton></PlayButton>
      <div class="server-btn"></div>
    </div>
    
  </div>
</template>

<script>
export default {

}
</script>

<style>

</style>
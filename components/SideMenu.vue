<template>
  <div class="xl:w-20 md:w-14 xl:py-8 bg-dark-200 w-12 h-full flex flex-col justify-between items-center sm:pt-3">
    <div class="logo">
        <IconLogo class="stroke-light-200"></IconLogo>
    </div>
    <NuxtLink to="/" class="wrapper-col">
        <IconHome class="nav-item"></IconHome>
    </NuxtLink>
    <div class="messages">
        <MessagesButton></MessagesButton>
    </div>
  </div>
</template>

<script>
export default {

}
</script>

<style>

</style>
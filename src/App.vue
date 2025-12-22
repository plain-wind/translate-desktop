<script setup lang="ts">
import Translate from '@components/Translate.vue';
</script>

<template>
  <div class="app-container">
    <Translate />
  </div>
</template>

<style lang="scss">
@use '@/styles/variables.scss' as *;

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica,
    Arial, sans-serif;
}

html {
  scroll-behavior: smooth;
  scrollbar-width: none;
  font-size: $font-size-lg;
}
</style>
<style scoped lang="scss"></style>
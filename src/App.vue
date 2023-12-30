<script setup>
import Image from './components/Image.vue';
</script>
<template>
  <Suspense>
    <Image/>
    <template #fallback>
      loading...
    </template>
  </Suspense>
</template>
<style scoped>
</style>
<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, onMounted } from 'vue'
import { PhysicalSize } from '@tauri-apps/api/window'

const imgPath = window.imgPath;
const el = ref()
onMounted(() => {
  el.value.onload = function () {
    let win = window.__TAURI__.window.getCurrent();
    const { width, height } = el.value;
    console.log(el.value, width, height)
    win.setSize(new PhysicalSize(width, height)).catch(console.error);
  };
})
</script>
<template>
  <img data-tauri-drag-region ref="el" :src="imgPath" />
</template>
<style scoped>
img {
  margin: 0;
  border: 0;
}
</style>
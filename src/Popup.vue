<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, onMounted } from 'vue'
import { PhysicalSize, LogicalSize, appWindow } from '@tauri-apps/api/window'

const imgPath = window.imgPath;
const el = ref()
onMounted(() => {
  el.value.onload = async function () {
    let win = window.__TAURI__.window.getCurrent();
    const { width, height } = el.value;
    const factor = await appWindow.scaleFactor();
    var size = new PhysicalSize(width, height);
    var logical = size.toLogical(factor);
    el.value.width = logical.width;
    el.value.height = logical.height;
    win.setSize(logical).catch(console.error);
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
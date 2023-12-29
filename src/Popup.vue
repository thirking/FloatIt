<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, onMounted } from 'vue'
import { PhysicalSize, PhysicalPosition, LogicalPosition, LogicalSize } from '@tauri-apps/api/window'

const imgPath = window.imgPath;

const el = ref();
let width = 0, height = 0, scaleFactor = 1;

async function resize(){
  let win = window.__TAURI__.window.getCurrent();
  let size = new PhysicalSize(Math.round(width * scaleFactor), Math.round(height * scaleFactor));
  win.setSize(size).catch(console.error);
}

const onLoaded = async () => {
  var image = new Image();
  image.src = imgPath;
  width = image.width;
  height = image.height;
  resize()
}

const handleWheel = async (e) => {
  let deltaFactor = 0;
  if (e.deltaY > 0) {
    scaleFactor = Math.max(0.1, scaleFactor - 0.1);
    deltaFactor = scaleFactor / (scaleFactor + 0.1);
  } else {
    scaleFactor += 0.1;
    deltaFactor = scaleFactor / (scaleFactor - 0.1);
  }
  // console.log(e, scaleFactor)
  let win = window.__TAURI__.window.getCurrent();
  let outerpos = await win.outerPosition();
  console.log(outerpos);
  let screenX = e.screenX;
  let screenY = e.screenY;
  let left = e.clientX;
  let top = e.clientY;
  let position = new LogicalPosition(Math.round(screenX - left * deltaFactor) , Math.round(screenY - top * deltaFactor));
  console.log(deltaFactor, screenX, screenY, left, top, position)
  win.setPosition(position).catch(console.error)
  resize();
}
</script>
<template>
  <img data-tauri-drag-region ref="el" :src="imgPath" @load="onLoaded" @mousewheel="handleWheel" />
</template>
<style scoped>
img {
  margin: 0;
  border: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}
</style>
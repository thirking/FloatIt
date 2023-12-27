<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
window.addEventListener('DOMContentLoaded',() => {
            const img = document.createElement('img');
            img.onload = function() {
                const {width, height} = img;
                window.__TAURI__.window.getCurrent().then(win => {
                    win.setSize({ width, height }).catch(console.error);
                });
            };
            img.setAttribute("data-tauri-drag-region", true);
            img.src = 'data:image/png;base64,{}';
            document.body.appendChild(img);
        });
</script>
<template>

</template>
<style scoped>
</style>
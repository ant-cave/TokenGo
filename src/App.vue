<script setup>
import Sidebar from './components/Sidebar.vue'
</script>

<template>
  <div
    style="display: flex; height: 100vh; border: 1px solid var(--borderColor-default, #d1d9e0); border-radius: 6px; overflow: hidden;">
    <Sidebar />
    <main style="flex: 1; overflow: auto; padding: 24px 32px;">
      <router-view />
    </main>
  </div>
</template>

<style>
a {
  text-decoration: none !important;
}
</style>

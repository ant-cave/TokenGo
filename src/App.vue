<script setup>
import { onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import Sidebar from './components/Sidebar.vue'

const router = useRouter()
const route = useRoute()

// 应用主题
function applyTheme(mode) {
  const html = document.documentElement
  html.removeAttribute('data-color-mode')
  
  if (mode === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    html.setAttribute('data-color-mode', prefersDark ? 'dark' : 'light')
  } else {
    html.setAttribute('data-color-mode', mode)
  }
}

// 加载并应用保存的主题
async function loadTheme() {
  try {
    const theme = await invoke('get_theme')
    applyTheme(theme)
  } catch (e) {
    console.error('加载主题失败:', e)
    // 失败时用 system 兜底
    applyTheme('system')
  }
}

// 检查登录状态，未登录时重定向到登录页
onMounted(async () => {
  // 先加载主题（不管在哪个页面都要加载）
  await loadTheme()
  
  // 在 auth 页面不需要检查登录
  if (route.path === '/auth' || route.path === '/') {
    return
  }
  
  try {
    const loggedIn = await invoke('is_logged_in')
    if (!loggedIn) {
      router.push('/auth')
    }
  } catch (e) {
    console.error('检查登录状态失败:', e)
    router.push('/auth')
  }
})
</script>

<template>
  <div v-if="$route.path === '/auth'" style="height: 100vh;">
    <router-view />
  </div>
  <div v-else class="app-layout">
    <Sidebar />
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<style>
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow: auto;
  padding: 24px 32px;
  background-color: var(--bgColor-default);
}

/* 竖屏适配 */
@media (max-width: 768px) {
  .app-layout {
    flex-direction: column;
  }
  
  .gh-sidebar {
    width: 100%;
    min-height: auto;
    border-right: none;
    border-bottom: 1px solid var(--borderColor-default);
  }
  
  .main-content {
    padding: 16px;
  }
}

a {
  text-decoration: none !important;
}
</style>

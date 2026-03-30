<script setup>
import { useRouter, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const route = useRoute()

const navItems = [
  { path: '/home', label: '密码列表', icon: 'key' },
  { path: '/add', label: '添加密钥', icon: 'plus' },
  { path: '/import', label: '导入密钥', icon: 'download' },
]

function isActive(path) {
  return route.path === path
}

async function logout() {
  await invoke('logout')
  router.push('/auth')
}
</script>

<template>
  <aside class="gh-sidebar">
    <div class="gh-sidebar-header">
      <svg height="32" viewBox="0 0 16 16" fill="currentColor" style="color: var(--fgColor-default);">
        <path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM5.78 8.75a9.64 9.64 0 0 0 1.9 3.48c.383-.477.78-1.102 1.1-1.947a20.2 20.2 0 0 0-1.1-1.947 9.64 9.64 0 0 0-1.9 3.48Zm5.024-.932a11.4 11.4 0 0 1 2.295 3.6H11.1c-.22-.92-.54-1.768-.94-2.532.4-.764.72-1.612.94-2.532Zm-.94-2.532a11.4 11.4 0 0 1 .94-2.532h2.997a11.4 11.4 0 0 1-2.295 3.6c-.4-.764-.72-1.612-.94-2.532ZM11.1 8.75h2.997a11.4 11.4 0 0 1-2.295 3.6 11.4 11.4 0 0 1-.94-2.532c.4-.764.72-1.612.94-2.532Zm-1.1-4.5a9.64 9.64 0 0 0-1.9-3.48c-.383.477-.78 1.102-1.1 1.947.32.845.717 1.47 1.1 1.947a9.64 9.64 0 0 0 1.9-3.48Zm-2.024.932a11.4 11.4 0 0 1-.94 2.532H4.14a11.4 11.4 0 0 1 2.295-3.6c.4.764.72 1.612.94 2.532Zm-.94 2.532c.22.92.54 1.768.94 2.532a11.4 11.4 0 0 1-2.295-3.6H7.14c.22.92.54 1.768.94 2.532ZM8 1.75a7.25 7.25 0 1 0 0 14.5 7.25 7.25 0 0 0 0-14.5Z"/>
      </svg>
      <div>
        <div class="gh-sidebar-title">TokenGo</div>
        <div class="gh-sidebar-subtitle">TOTP 生成器</div>
      </div>
    </div>
    
    <nav class="gh-sidebar-nav">
      <div class="gh-sidebar-section">
        <div class="gh-sidebar-section-title">功能</div>
        
        <a 
          v-for="item in navItems" 
          :key="item.path"
          class="gh-sidebar-item"
          :class="{ active: isActive(item.path) }"
          @click="router.push(item.path)"
        >
          <svg class="gh-sidebar-icon" viewBox="0 0 16 16" fill="currentColor">
            <!-- key icon -->
            <path v-if="item.icon === 'key'" d="M3.5 11.5a3.5 3.5 0 1 1 3.163-5H14L15 6l-3 3-1-1-1 1-1-1-1 1-1-1-1 1-1-1H6.663A3.5 3.5 0 0 1 3.5 11.5Z"/>
            <!-- plus icon -->
            <path v-else-if="item.icon === 'plus'" d="M7.75 2a.75.75 0 0 1 .75.75V7h4.25a.75.75 0 0 1 0 1.5H8.5v4.25a.75.75 0 0 1-1.5 0V8.5H2.75a.75.75 0 0 1 0-1.5H7V2.75A.75.75 0 0 1 7.75 2Z"/>
            <!-- download icon -->
            <path v-else-if="item.icon === 'download'" d="M7.47 10.78a.75.75 0 0 0 1.06 0l3.75-3.75a.75.75 0 0 0-1.06-1.06L8.75 8.44V1.75a.75.75 0 0 0-1.5 0v6.69L4.78 5.97a.75.75 0 0 0-1.06 1.06l3.75 3.75ZM3.75 13a.75.75 0 0 0 0 1.5h8.5a.75.75 0 0 0 0-1.5h-8.5Z"/>
          </svg>
          {{ item.label }}
        </a>
      </div>
      
      <div class="gh-sidebar-section" style="margin-top: auto;">
        <div class="gh-sidebar-section-title">账户</div>
        <a class="gh-sidebar-item" @click="logout">
          <svg class="gh-sidebar-icon" viewBox="0 0 16 16" fill="currentColor">
            <path d="M2 2.75C2 1.784 2.784 1 3.75 1h2.5a.75.75 0 0 1 0 1.5h-2.5a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25h2.5a.75.75 0 0 1 0 1.5h-2.5A1.75 1.75 0 0 1 2 13.25Zm10.44 4.5-1.97-1.97a.749.749 0 0 1 .326-1.275.749.749 0 0 1 .734.215l3.25 3.25a.75.75 0 0 1 0 1.06l-3.25 3.25a.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734l1.97-1.97H6.75a.75.75 0 0 1 0-1.5Z"/>
          </svg>
          退出登录
        </a>
      </div>
    </nav>
  </aside>
</template>

<style scoped>
.gh-sidebar-nav {
  display: flex;
  flex-direction: column;
  height: calc(100% - 64px);
}
</style>

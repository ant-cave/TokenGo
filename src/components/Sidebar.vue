<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import Icon from './Icon.vue'

const router = useRouter()
const route = useRoute()

const isSidebarOpen = ref(true)

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value
}

// 监听窗口大小，竖屏时自动收起
const checkScreenWidth = () => {
  isSidebarOpen.value = window.innerWidth > window.innerHeight
}

const routerLinkPress = () => {
  if (window.innerWidth < window.innerHeight) {
    isSidebarOpen.value = false
  }
}

onMounted(() => {
  checkScreenWidth()
  window.addEventListener('resize', checkScreenWidth)
})

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenWidth)
})

// 暴露给父组件控制
defineExpose({
  isSidebarOpen,
  toggleSidebar
})

const navItems = [
  { path: '/home', label: '密码列表', icon: 'key' },
  { path: '/add', label: '添加密钥', icon: 'plus' },
  { path: '/import', label: '导入密钥', icon: 'download' },
  { path: '/settings', label: '设置', icon: 'gear' },
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
  <aside class="gh-sidebar" :class="{ collapsed: !isSidebarOpen }">
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
          @click="router.push(item.path); routerLinkPress()"
        >
          <Icon :name="item.icon" class="gh-sidebar-icon" />
          {{ item.label }}
        </a>
      </div>
      
      <div class="gh-sidebar-section" style="margin-top: auto;">
        <div class="gh-sidebar-section-title">账户</div>
        <a class="gh-sidebar-item" @click="logout">
          <Icon name="sign-out" class="gh-sidebar-icon" />
          退出登录
        </a>
      </div>
    </nav>
  </aside>
  <button class="gh-card-action sidebar-toggle" @click="toggleSidebar">
    <span class="sidebar-toggle-icon" :style="{ transform: isSidebarOpen ? 'rotate(0deg)' : 'rotate(180deg)' }">
      <
    </span>
  </button>
</template>

<style scoped>
.sidebar-toggle {
  height: 30px;
  width: 30px;
  margin: 0;
  position: absolute;
  left: 215px;
  bottom: 16px;
  z-index: 101;
  transition: left 0.4s ease-in-out;
}

.gh-sidebar.collapsed + .sidebar-toggle {
  left: 16px;
}

.sidebar-toggle-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  transition: transform 0.4s ease-in-out;
}

.gh-sidebar {
  width: 260px;
  height: 100%;
  transition: width, padding 0.35s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  flex-shrink: 0;
  background: var(--bgColor-default);
}

.gh-sidebar.collapsed {
  width: 0;
  opacity: 0;
  padding-left: 0px;
  padding-right: 0px;
}

.gh-sidebar {
  transition: all 0.35s cubic-bezier(0.4, 0, 0.2, 1)
}

.gh-sidebar-nav {
  display: flex;
  flex-direction: column;
  height: calc(100% - 64px);
}

@media (orientation: portrait) {
  .gh-sidebar {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
    z-index: 100;
    /* 强制不透明背景 */
    background-color: #0d1117 !important;
  }

  .gh-sidebar:not(.collapsed) {
    width: 100%;
    background-color: #0d1117 !important;
  }

  .gh-sidebar.collapsed {
    background-color: #0d1117 !important;
    pointer-events: none;
  }

  .gh-sidebar.collapsed * {
    opacity: 0;
  }

  .sidebar-toggle {
    left: calc(100% - 46px) !important;
    right: auto !important;
  }

  .gh-sidebar.collapsed + .sidebar-toggle {
    left: 16px !important;
  }

  .gh-sidebar:not(.collapsed) + .sidebar-toggle {
    left: calc(100% - 46px) !important;
    z-index: 101;
  }
}
</style>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

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
</script>

<template>
  <aside class="gh-sidebar" :class="{ collapsed: !isSidebarOpen }">
    <div class="gh-sidebar-header">
      <div>
        <div class="gh-sidebar-title">TokenGo</div>
        <div class="gh-sidebar-subtitle">密钥管理</div>
      </div>
    </div>
    <nav class="gh-sidebar-nav">
      <div class="gh-sidebar-section">
        <div class="gh-sidebar-section-title">数据</div>
        <router-link to="/today" class="gh-sidebar-item" active-class="active" @click="routerLinkPress">
          今日
        </router-link>
        <router-link to="/calendar" class="gh-sidebar-item" active-class="active" @click="routerLinkPress">
          日历
        </router-link>
        <router-link to="/analysis" class="gh-sidebar-item" active-class="active" @click="routerLinkPress">
          分析
        </router-link>
      </div>
      <div class="gh-sidebar-section">
        <div class="gh-sidebar-section-title">设置</div>
        <router-link to="/settings" class="gh-sidebar-item" active-class="active" @click="routerLinkPress">
          设置
        </router-link>
        <router-link to="/about" class="gh-sidebar-item" active-class="active" @click="routerLinkPress">
          关于我们
        </router-link>
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
  background: white;
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

@media (orientation: portrait) {
  .gh-sidebar {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    z-index: 100;
    background: white;
  }

  .gh-sidebar:not(.collapsed) {
    width: 100%;
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

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Icon from '../components/Icon.vue'

// 主题设置：light/dark/system
const theme = ref('system')

// 密码修改相关
const currentPassword = ref('')
const newPassword = ref('')
const confirmNewPassword = ref('')
const passwordError = ref('')
const passwordSuccess = ref('')

// 加载保存的主题设置（从数据库）
onMounted(async () => {
  try {
    const saved = await invoke('get_theme')
    theme.value = saved
    applyTheme(theme.value)
  } catch (e) {
    console.error('加载主题失败:', e)
  }
})

// 应用主题
function applyTheme(mode) {
  const html = document.documentElement
  // 先清除之前的设置
  html.removeAttribute('data-color-mode')

  if (mode === 'system') {
    // 检测系统偏好
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    html.setAttribute('data-color-mode', prefersDark ? 'dark' : 'light')
  } else {
    html.setAttribute('data-color-mode', mode)
  }
}

// 切换主题
async function changeTheme(mode) {
  theme.value = mode
  applyTheme(mode)
  // 保存到数据库
  try {
    await invoke('save_theme', { theme: mode })
  } catch (e) {
    console.error('保存主题失败:', e)
  }
}

// 修改密码
async function changePassword() {
  passwordError.value = ''
  passwordSuccess.value = ''
  
  if (!currentPassword.value || !newPassword.value) {
    passwordError.value = '请填写完整信息'
    return
  }
  
  if (newPassword.value !== confirmNewPassword.value) {
    passwordError.value = '两次输入的新密码不一致'
    return
  }
  
  if (newPassword.value.length < 6) {
    passwordError.value = '新密码至少需要6位'
    return
  }
  
  try {
    await invoke('change_master_password', {
      oldPassword: currentPassword.value,
      newPassword: newPassword.value
    })
    passwordSuccess.value = '密码修改成功'
    currentPassword.value = ''
    newPassword.value = ''
    confirmNewPassword.value = ''
  } catch (e) {
    passwordError.value = '修改失败: ' + e
  }
}
</script>

<template>
  <div>
    <h1 class="h2-mktg mb-4">设置</h1>
    
    <!-- 外观设置 -->
    <div class="Box p-4 mb-4">
      <h3 class="h4-mktg mb-3 d-flex flex-items-center" style="gap: 8px;">
        <Icon name="paintbrush" :size="20" />
        外观
      </h3>
      
      <div class="form-group">
        <label class="form-label">主题模式</label>
        <div class="d-flex" style="gap: 12px;">
          <button 
            class="btn btn-outline"
            :class="{ 'btn-primary': theme === 'light' }"
            @click="changeTheme('light')"
          >
            <Icon name="sun" :size="16" class="mr-1" />
            亮色
          </button>
          <button 
            class="btn btn-outline"
            :class="{ 'btn-primary': theme === 'dark' }"
            @click="changeTheme('dark')"
          >
            <Icon name="moon" :size="16" class="mr-1" />
            暗色
          </button>
          <button 
            class="btn btn-outline"
            :class="{ 'btn-primary': theme === 'system' }"
            @click="changeTheme('system')"
          >
            <Icon name="desktop" :size="16" class="mr-1" />
            跟随系统
          </button>
        </div>
      </div>
    </div>
    
    <!-- 安全设置 -->
    <div class="Box p-4">
      <h3 class="h4-mktg mb-4 d-flex flex-items-center" style="gap: 8px;">
        <Icon name="shield" :size="20" />
        安全
      </h3>
      
      <div style="max-width: 400px;">
        <div class="form-group mb-3">
          <label class="form-label d-block mb-2">当前密码</label>
          <input 
            type="password" 
            class="form-control" 
            v-model="currentPassword"
            placeholder="输入当前主密码"
            style="width: 100%;"
          />
        </div>
        
        <div class="form-group mb-3">
          <label class="form-label d-block mb-2">新密码</label>
          <input 
            type="password" 
            class="form-control" 
            v-model="newPassword"
            placeholder="至少 6 位"
            style="width: 100%;"
          />
        </div>
        
        <div class="form-group mb-3">
          <label class="form-label d-block mb-2">确认新密码</label>
          <input 
            type="password" 
            class="form-control" 
            v-model="confirmNewPassword"
            placeholder="再次输入新密码"
            @keyup.enter="changePassword"
            style="width: 100%;"
          />
        </div>
        
        <div v-if="passwordError" class="flash flash-error mb-3 text-small">
          {{ passwordError }}
        </div>
        
        <div v-if="passwordSuccess" class="flash flash-success mb-3 text-small">
          {{ passwordSuccess }}
        </div>
        
        <button class="btn btn-primary" @click="changePassword">
          修改密码
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mr-1 {
  margin-right: 4px;
}
</style>

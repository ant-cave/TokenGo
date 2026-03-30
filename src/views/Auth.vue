<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()

// 状态
const hasPassword = ref(false)
const isSetup = ref(false)
const password = ref('')
const confirmPassword = ref('')
const error = ref('')
const loading = ref(false)

// 检查是否已设置主密码
onMounted(async () => {
  try {
    await invoke('init_database')
    hasPassword.value = await invoke('has_master_password')
    isSetup.value = !hasPassword.value
  } catch (e) {
    error.value = '初始化失败: ' + e
  }
})

// 设置主密码（首次使用）
async function setupPassword() {
  error.value = ''
  
  if (password.value.length < 6) {
    error.value = '密码至少6位'
    return
  }
  if (password.value !== confirmPassword.value) {
    error.value = '两次输入的密码不一致'
    return
  }
  
  loading.value = true
  try {
    await invoke('setup_master_password', { password: password.value })
    router.push('/home')
  } catch (e) {
    error.value = '设置失败: ' + e
  } finally {
    loading.value = false
  }
}

// 登录验证
async function login() {
  error.value = ''
  
  if (!password.value) {
    error.value = '请输入密码'
    return
  }
  
  loading.value = true
  try {
    const valid = await invoke('verify_master_password', { password: password.value })
    if (valid) {
      router.push('/home')
    } else {
      error.value = '密码错误'
    }
  } catch (e) {
    error.value = '验证失败: ' + e
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="auth-container">
    <div class="Box p-6" style="width: 100%; max-width: 360px;">
      <!-- 标题 -->
      <div class="text-center mb-4">
        <svg height="48" viewBox="0 0 16 16" fill="currentColor" style="color: var(--fgColor-default);">
          <path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM5.78 8.75a9.64 9.64 0 0 0 1.9 3.48c.383-.477.78-1.102 1.1-1.947a20.2 20.2 0 0 0-1.1-1.947 9.64 9.64 0 0 0-1.9 3.48Zm5.024-.932a11.4 11.4 0 0 1 2.295 3.6H11.1c-.22-.92-.54-1.768-.94-2.532.4-.764.72-1.612.94-2.532Zm-.94-2.532a11.4 11.4 0 0 1 .94-2.532h2.997a11.4 11.4 0 0 1-2.295 3.6c-.4-.764-.72-1.612-.94-2.532ZM11.1 8.75h2.997a11.4 11.4 0 0 1-2.295 3.6 11.4 11.4 0 0 1-.94-2.532c.4-.764.72-1.612.94-2.532Zm-1.1-4.5a9.64 9.64 0 0 0-1.9-3.48c-.383.477-.78 1.102-1.1 1.947.32.845.717 1.47 1.1 1.947a9.64 9.64 0 0 0 1.9-3.48Zm-2.024.932a11.4 11.4 0 0 1-.94 2.532H4.14a11.4 11.4 0 0 1 2.295-3.6c.4.764.72 1.612.94 2.532Zm-.94 2.532c.22.92.54 1.768.94 2.532a11.4 11.4 0 0 1-2.295-3.6H7.14c.22.92.54 1.768.94 2.532ZM8 1.75a7.25 7.25 0 1 0 0 14.5 7.25 7.25 0 0 0 0-14.5Z"/>
        </svg>
        <h1 class="h3-mktg mt-3 mb-1">TokenGo</h1>
        <p class="text-small color-fg-muted">TOTP 密码生成器</p>
      </div>

      <!-- 首次设置 -->
      <div v-if="isSetup">
        <p class="mb-3 text-small">首次使用，请设置主密码</p>
        
        <div class="form-group mb-3">
          <label class="form-label">主密码</label>
          <input 
            type="password" 
            class="form-control" 
            v-model="password"
            placeholder="至少6位"
            @keyup.enter="setupPassword"
          />
        </div>
        
        <div class="form-group mb-3">
          <label class="form-label">确认密码</label>
          <input 
            type="password" 
            class="form-control" 
            v-model="confirmPassword"
            placeholder="再次输入"
            @keyup.enter="setupPassword"
          />
        </div>
        
        <button 
          class="btn btn-primary btn-block" 
          @click="setupPassword"
          :disabled="loading"
        >
          {{ loading ? '设置中...' : '设置主密码' }}
        </button>
      </div>

      <!-- 登录 -->
      <div v-else>
        <p class="mb-3 text-small">请输入主密码解锁</p>
        
        <div class="form-group mb-3">
          <input 
            type="password" 
            class="form-control" 
            v-model="password"
            placeholder="主密码"
            @keyup.enter="login"
          />
        </div>
        
        <button 
          class="btn btn-primary btn-block" 
          @click="login"
          :disabled="loading"
        >
          {{ loading ? '验证中...' : '解锁' }}
        </button>
      </div>

      <!-- 错误提示 -->
      <div v-if="error" class="flash flash-error mt-3 text-small">
        {{ error }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.auth-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 16px;
  background-color: var(--bgColor-muted);
}

.btn-block {
  width: 100%;
}

.text-center {
  text-align: center;
}

.mt-3 {
  margin-top: 12px;
}

.mb-1 {
  margin-bottom: 4px;
}
</style>

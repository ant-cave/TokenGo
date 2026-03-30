<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 状态
const secrets = ref([])
const codes = ref({})  // id -> { code, progress }
const loading = ref(true)
const error = ref('')

// 查看明文相关
const showSecretModal = ref(false)
const selectedSecret = ref(null)
const plaintextSecret = ref('')
const confirmPassword = ref('')
const secretError = ref('')

// 删除相关
const showDeleteModal = ref(false)
const deleteTarget = ref(null)
const deletePassword = ref('')
const deleteError = ref('')

let timer = null

// 加载密钥列表
async function loadSecrets() {
  try {
    secrets.value = await invoke('get_totp_secrets')
    // 为每个密钥生成初始 TOTP 码
    for (const secret of secrets.value) {
      await refreshCode(secret.id)
    }
  } catch (e) {
    error.value = '加载失败: ' + e
  } finally {
    loading.value = false
  }
}

// 刷新单个 TOTP 码
async function refreshCode(id) {
  try {
    const code = await invoke('generate_totp_code', { id })
    const remaining = await invoke('get_time_remaining')
    codes.value[id] = {
      code,
      progress: (remaining / 30) * 100
    }
  } catch (e) {
    console.error('生成 TOTP 失败:', e)
    codes.value[id] = { code: '------', progress: 0 }
  }
}

// 定时更新倒计时
function startTimer() {
  timer = setInterval(async () => {
    const remaining = await invoke('get_time_remaining')
    
    // 更新进度条
    for (const id in codes.value) {
      if (codes.value[id]) {
        codes.value[id].progress = (remaining / 30) * 100
      }
    }
    
    // 时间步切换时刷新所有码
    if (remaining === 30 || remaining === 29) {
      for (const secret of secrets.value) {
        await refreshCode(secret.id)
      }
    }
  }, 1000)
}

// 复制到剪贴板
async function copyCode(code) {
  try {
    await navigator.clipboard.writeText(code)
    // 简单的视觉反馈，可以改进
    alert('已复制: ' + code)
  } catch (e) {
    console.error('复制失败:', e)
  }
}

// 打开查看明文弹窗
function openSecretModal(secret) {
  selectedSecret.value = secret
  plaintextSecret.value = ''
  confirmPassword.value = ''
  secretError.value = ''
  showSecretModal.value = true
}

// 验证密码并显示明文
async function revealSecret() {
  secretError.value = ''
  if (!confirmPassword.value) {
    secretError.value = '请输入密码'
    return
  }
  
  try {
    // 先验证密码
    const valid = await invoke('verify_master_password', { password: confirmPassword.value })
    if (!valid) {
      secretError.value = '密码错误'
      return
    }
    
    // 获取明文
    plaintextSecret.value = await invoke('get_secret_plaintext', { 
      id: selectedSecret.value.id 
    })
  } catch (e) {
    secretError.value = '获取失败: ' + e
  }
}

// 打开删除弹窗
function openDeleteModal(secret) {
  deleteTarget.value = secret
  deletePassword.value = ''
  deleteError.value = ''
  showDeleteModal.value = true
}

// 确认删除
async function confirmDelete() {
  deleteError.value = ''
  if (!deletePassword.value) {
    deleteError.value = '请输入密码确认删除'
    return
  }
  
  try {
    // 验证密码
    const valid = await invoke('verify_master_password', { password: deletePassword.value })
    if (!valid) {
      deleteError.value = '密码错误'
      return
    }
    
    // 删除
    await invoke('delete_totp_secret', { id: deleteTarget.value.id })
    
    // 关闭弹窗，刷新列表
    showDeleteModal.value = false
    await loadSecrets()
  } catch (e) {
    deleteError.value = '删除失败: ' + e
  }
}

onMounted(() => {
  loadSecrets()
  startTimer()
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})
</script>

<template>
  <div>
    <h1 class="h2-mktg mb-4">TOTP 密码列表</h1>
    
    <!-- 加载中 -->
    <div v-if="loading" class="text-center py-6">
      <p class="color-fg-muted">加载中...</p>
    </div>
    
    <!-- 空状态 -->
    <div v-else-if="secrets.length === 0" class="Box p-6 text-center">
      <svg height="48" viewBox="0 0 16 16" fill="currentColor" class="color-fg-muted mb-3">
        <path d="M3.5 11.5a3.5 3.5 0 1 1 3.163-5H14L15 6l-3 3-1-1-1 1-1-1-1 1-1-1-1 1-1-1H6.663A3.5 3.5 0 0 1 3.5 11.5Z"/>
      </svg>
      <p class="color-fg-muted mb-3">还没有添加任何密钥</p>
      <button class="btn btn-primary" @click="$router.push('/add')">
        添加第一个密钥
      </button>
    </div>
    
    <!-- 密钥列表 -->
    <div v-else class="d-flex flex-column" style="gap: 12px;">
      <div 
        v-for="secret in secrets" 
        :key="secret.id"
        class="Box p-4"
      >
        <div class="d-flex flex-items-center flex-justify-between mb-3">
          <h3 class="h4-mktg mb-0">{{ secret.name }}</h3>
          <div class="d-flex" style="gap: 8px;">
            <button 
              class="btn btn-sm" 
              @click="openSecretModal(secret)"
              title="查看密钥"
            >
              <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor">
                <path d="M8 2c1.981 0 3.671.992 4.933 2.078 1.27 1.091 2.187 2.345 2.637 3.023a1.62 1.62 0 0 1 0 1.798c-.45.678-1.367 1.932-2.637 3.023C11.67 13.008 9.981 14 8 14c-1.981 0-3.671-.992-4.933-2.078C1.797 10.83.88 9.576.43 8.898a1.62 1.62 0 0 1 0-1.798c.45-.678 1.367-1.932 2.637-3.023C4.33 2.992 6.019 2 8 2ZM1.679 7.932a.12.12 0 0 0 0 .136c.411.622 1.241 1.75 2.366 2.717C5.176 11.872 6.54 12.5 8 12.5c1.46 0 2.824-.628 3.955-1.715 1.125-.967 1.955-2.095 2.366-2.717a.12.12 0 0 0 0-.136c-.411-.622-1.241-1.75-2.366-2.717C10.824 4.128 9.46 3.5 8 3.5c-1.46 0-2.824.628-3.955 1.715-1.125.967-1.955 2.095-2.366 2.717ZM8 10a2 2 0 1 1 0-4 2 2 0 0 1 0 4Z"/>
              </svg>
            </button>
            <button 
              class="btn btn-sm btn-danger" 
              @click="openDeleteModal(secret)"
              title="删除"
            >
              <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor">
                <path d="M11 1.75V3h2.25a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1 0-1.5H5V1.75C5 .784 5.784 0 6.75 0h2.5C10.216 0 11 .784 11 1.75ZM4.496 6.675a.75.75 0 1 0-1.492.15l.66 6.6A1.75 1.75 0 0 0 5.405 15h5.19c.9 0 1.652-.681 1.741-1.576l.66-6.6a.75.75 0 0 0-1.492-.149l-.66 6.6a.25.25 0 0 1-.249.225h-5.19a.25.25 0 0 1-.249-.225l-.66-6.6Z"/>
              </svg>
            </button>
          </div>
        </div>
        
        <!-- TOTP 码显示 -->
        <div 
          class="d-flex flex-items-center flex-justify-between p-3 rounded-2"
          style="background-color: var(--bgColor-muted); cursor: pointer;"
          @click="copyCode(codes[secret.id]?.code || '------')"
        >
          <span class="f1-mktg" style="font-family: monospace; letter-spacing: 4px;">
            {{ codes[secret.id]?.code || '------' }}
          </span>
          <span class="text-small color-fg-muted">点击复制</span>
        </div>
        
        <!-- 倒计时进度条 -->
        <div class="mt-2" style="height: 4px; background-color: var(--borderColor-default); border-radius: 2px; overflow: hidden;">
          <div 
            class="progress-bar"
            :style="{ 
              width: (codes[secret.id]?.progress || 0) + '%',
              backgroundColor: (codes[secret.id]?.progress || 0) < 20 ? 'var(--color-danger-emphasis)' : 'var(--color-success-emphasis)'
            }"
          ></div>
        </div>
      </div>
    </div>
    
    <!-- 查看明文弹窗 -->
    <div v-if="showSecretModal" class="modal-overlay" @click.self="showSecretModal = false">
      <div class="modal-content Box p-4" style="width: 90%; max-width: 400px;">
        <h3 class="h4-mktg mb-3">查看密钥</h3>
        <p class="text-small color-fg-muted mb-3">
          查看明文密钥需要验证主密码
        </p>
        
        <div v-if="!plaintextSecret" class="form-group mb-3">
          <input 
            type="password" 
            class="form-control" 
            v-model="confirmPassword"
            placeholder="输入主密码"
            @keyup.enter="revealSecret"
          />
        </div>
        
        <div v-else class="flash mb-3">
          <div class="text-small color-fg-muted mb-1">密钥 (Base32):</div>
          <code class="text-mono" style="word-break: break-all;">{{ plaintextSecret }}</code>
        </div>
        
        <div v-if="secretError" class="flash flash-error mb-3 text-small">
          {{ secretError }}
        </div>
        
        <div class="d-flex flex-justify-end" style="gap: 8px;">
          <button class="btn" @click="showSecretModal = false">关闭</button>
          <button 
            v-if="!plaintextSecret"
            class="btn btn-primary" 
            @click="revealSecret"
          >
            验证并显示
          </button>
        </div>
      </div>
    </div>
    
    <!-- 删除确认弹窗 -->
    <div v-if="showDeleteModal" class="modal-overlay" @click.self="showDeleteModal = false">
      <div class="modal-content Box p-4" style="width: 90%; max-width: 400px;">
        <h3 class="h4-mktg mb-2 color-fg-danger">删除密钥</h3>
        <div class="flash flash-warn mb-3">
          <strong>警告：</strong>删除后无法恢复！
        </div>
        <p class="text-small mb-3">
          你正在删除 <strong>{{ deleteTarget?.name }}</strong>，请输入主密码确认。
        </p>
        
        <div class="form-group mb-3">
          <input 
            type="password" 
            class="form-control" 
            v-model="deletePassword"
            placeholder="输入主密码确认删除"
            @keyup.enter="confirmDelete"
          />
        </div>
        
        <div v-if="deleteError" class="flash flash-error mb-3 text-small">
          {{ deleteError }}
        </div>
        
        <div class="d-flex flex-justify-end" style="gap: 8px;">
          <button class="btn" @click="showDeleteModal = false">取消</button>
          <button class="btn btn-danger" @click="confirmDelete">确认删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.progress-bar {
  height: 100%;
  transition: width 1s linear;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal-content {
  max-height: 90vh;
  overflow: auto;
}

.f1-mktg {
  font-size: 32px;
  font-weight: 600;
}

@media (max-width: 768px) {
  .f1-mktg {
    font-size: 24px;
  }
}
</style>

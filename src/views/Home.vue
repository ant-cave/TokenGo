<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Icon from '../components/Icon.vue'

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
const isLongPressing = ref(false)
let pressTimer = null

// 长按查看明文
function startLongPress(secret) {
  isLongPressing.value = true
  pressTimer = setTimeout(() => {
    openSecretModal(secret)
    isLongPressing.value = false
  }, 500) // 500ms 长按
}

function cancelLongPress() {
  clearTimeout(pressTimer)
  isLongPressing.value = false
}

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
    showCopyToast(code)
  } catch (e) {
    console.error('复制失败:', e)
  }
}

// 显示复制提示
const showToast = ref(false)
const toastMessage = ref('')
let toastTimer = null

function showCopyToast(code) {
  toastMessage.value = '已复制: ' + code
  showToast.value = true
  
  if (toastTimer) clearTimeout(toastTimer)
  toastTimer = setTimeout(() => {
    showToast.value = false
  }, 500)
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
      <img src="/icon.png" alt="TokenGo" style="width: 48px; height: 48px; margin-bottom: 12px; opacity: 0.5;" />
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
              @mousedown="startLongPress(secret)"
              @mouseup="cancelLongPress()"
              @mouseleave="cancelLongPress()"
              @touchstart="startLongPress(secret)"
              @touchend="cancelLongPress()"
              :title="isLongPressing ? '松开查看' : '长按查看密钥'"
            >
              <Icon name="eye" :size="14" />
            </button>
            <button
              class="btn btn-sm btn-danger"
              @click="openDeleteModal(secret)"
              title="删除"
            >
              <Icon name="trash" :size="14" />
            </button>
          </div>
        </div>
        
        <!-- TOTP 码显示 -->
        <div 
          class="d-flex flex-items-center flex-justify-between p-3 rounded-2"
          style="background-color: var(--bgColor-muted); cursor: pointer;"
          @click="copyCode(codes[secret.id]?.code || '------')"
        >
          <span class="f1-mktg" style="font-family: 'Arial', 'Helvetica', sans-serif; letter-spacing: 4px;">
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
          <code style="word-break: break-all; font-family: 'Arial', 'Helvetica', sans-serif; font-size: 13px; line-height: 1.6;">{{ plaintextSecret }}</code>
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
    
    <!-- 复制提示 -->
    <div v-if="showToast" class="copy-toast">
      {{ toastMessage }}
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

.copy-toast {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  padding: 12px 24px;
  background-color: var(--color-success-emphasis);
  color: white;
  border-radius: 6px;
  font-size: 14px;
  z-index: 1000;
  animation: all 0.2s ease-in-out;
}

@keyframes fadeInOut {
  0%, 100% { opacity: 0; }
  10%, 90% { opacity: 1; }
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

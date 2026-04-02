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
    
    <!-- 软件信息 -->
    <div class="Box p-4 mt-4">
      <h3 class="h4-mktg mb-3 d-flex flex-items-center" style="gap: 8px;">
        <Icon name="info" :size="20" />
        软件信息
      </h3>
      <div class="d-flex flex-items-center" style="gap: 12px; margin-bottom: 16px;">
        <img src="/icon.png" alt="TokenGo" style="width: 48px; height: 48px;" />
        <div>
          <div class="h4-mktg mb-1">TokenGo</div>
          <div class="text-small color-fg-muted">TOTP 密码管理器</div>
        </div>
      </div>
      
      <div class="d-flex flex-items-center" style="gap: 8px; margin-bottom: 8px;">
        <span class="text-small color-fg-muted">作者：</span>
        <a href="https://github.com/ant-cave" target="_blank" class="text-small color-fg-muted text-underline">
          ant-cave
        </a>
      </div>
      
      <div class="d-flex flex-items-center" style="gap: 8px; margin-bottom: 8px;">
        <span class="text-small color-fg-muted">个人主页：</span>
        <a href="https://me.011420.xyz" target="_blank" class="text-small color-fg-muted text-underline">
          me.011420.xyz
        </a>
      </div>
      
      <div class="d-flex flex-items-center" style="gap: 8px;">
        <span class="text-small color-fg-muted">开源地址：</span>
        <a href="https://github.com/ant-cave/TokenGo" target="_blank" class="text-small color-fg-muted text-underline">
          github.com/ant-cave/TokenGo
        </a>
      </div>
    </div>
    
    <!-- AGPL 协议声明 -->
    <div class="Box p-4 mt-4" style="border-left: 4px solid var(--color-accent-emphasis);">
      <h3 class="h4-mktg mb-3 d-flex flex-items-center" style="gap: 8px;">
        <Icon name="shield" :size="20" />
        AGPL-3.0 协议声明
      </h3>
      <p class="text-small color-fg-muted mb-2">
        <strong>本软件遵循 AGPL-3.0 开源协议。</strong>
      </p>
      <p class="text-small color-fg-muted mb-3">
        你有权自由使用、修改和分发本软件，但任何修改后的版本也必须以 AGPL-3.0 协议发布。
      </p>
      <p class="text-small color-fg-muted">
        完整协议文本请参见：<a href="https://www.gnu.org/licenses/agpl-3.0.html" target="_blank">gnu.org/licenses/agpl-3.0</a>
      </p>
    </div>
    
    <!-- 开源软件声明 -->
    <div class="Box p-4 mt-4">
      <h3 class="h4-mktg mb-3 d-flex flex-items-center" style="gap: 8px;">
        <Icon name="code" :size="20" />
        开源软件声明
      </h3>
      <p class="text-small color-fg-muted mb-3">
        TokenGo 是一款开源的 TOTP 密码管理器，基于以下开源项目构建。协议信息参考 <a href="https://opensource.org/license" target="_blank">Open Source Initiative</a>。
      </p>
      
      <!-- Tauri -->
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://tauri.app" target="_blank">Tauri</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          构建轻量级桌面应用框架
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <!-- Vue 3 -->
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://vuejs.org" target="_blank">Vue 3</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          前端框架
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <!-- google-authenticator-libpam -->
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/google/google-authenticator-libpam" target="_blank">google-authenticator-libpam</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          TOTP 算法实现
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/BSD-2-Clause" target="_blank">BSD-2-Clause</a>
        </div>
      </div>
      
      <!-- 其他依赖 -->
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://tauri.app" target="_blank">tauri-plugin-opener</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          Tauri 插件
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://serde.rs" target="_blank">serde/serde_json</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          序列化库
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/RustCrypto/AES-gcm" target="_blank">aes-gcm</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          加密库
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/RustCrypto/KDFs" target="_blank">pbkdf2</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          密钥派生
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/rust-random/rand" target="_blank">rand</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          随机数生成
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/RustCrypto/hashes" target="_blank">sha2/hmac/sha1</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          哈希算法
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/paritytech/base32" target="_blank">base32</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          Base32 编码
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/rusqlite/rusqlite" target="_blank">rusqlite</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          SQLite 数据库
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <div class="Box p-3 mb-3">
        <div class="h5-mktg mb-2">
          <a href="https://github.com/matthieu-m/once_cell" target="_blank">once_cell</a>
        </div>
        <div class="text-small color-fg-muted mb-2">
          单例模式
        </div>
        <div class="text-small color-fg-muted">
          <a href="https://opensource.org/license/MIT" target="_blank">MIT</a>
        </div>
      </div>
      
      <p class="text-small color-fg-muted mt-3">
        本软件遵循 AGPL-3.0 开源协议。欢迎贡献代码或报告问题。
      </p>
    </div>
  </div>
</template>

<style scoped>
.mr-1 {
  margin-right: 4px;
}

/* 按钮文字颜色 - 使用 CSS 变量适配暗色模式 */
.btn-outline {
  color: var(--fgColor-default) !important;
}

/* 选中按钮文字白色 */
.btn-primary {
  color: #ffffff !important;
  background-color: var(--color-btn-primary-bg) !important;
  border-color: var(--color-btn-primary-border) !important;
}
</style>

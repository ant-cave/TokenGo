<!--
TokenGo - A lightweight TOTP password manager
Copyright (C) 2024 ant-cave <ANTmmmmm@outlook.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()

// 状态
const uri = ref('')
const error = ref('')
const success = ref('')
const loading = ref(false)
const parsed = ref(null)

// 解析 URI
async function parse() {
  error.value = ''
  success.value = ''
  parsed.value = null
  
  if (!uri.value.trim()) {
    error.value = '请输入 otpauth:// URI'
    return
  }
  
  try {
    const result = await invoke('parse_otpauth_uri', { uri: uri.value.trim() })
    parsed.value = result
  } catch (e) {
    error.value = '解析失败: ' + e
  }
}

// 确认导入
async function confirmImport() {
  if (!parsed.value) return
  
  loading.value = true
  try {
    await invoke('add_totp_secret', {
      request: {
        name: parsed.value.name,
        secret: parsed.value.secret
      }
    })
    
    success.value = '导入成功！'
    
    // 2秒后跳转
    setTimeout(() => {
      router.push('/home')
    }, 1500)
  } catch (e) {
    error.value = '导入失败: ' + e
    loading.value = false
  }
}

// 取消解析结果，重新输入
function reset() {
  parsed.value = null
  uri.value = ''
  error.value = ''
}
</script>

<template>
  <div>
    <h1 class="h2-mktg mb-4">导入密钥</h1>
    
    <div class="Box p-4" style="max-width: 560px;">
      <p class="text-small color-fg-muted mb-3">
        粘贴 otpauth:// 格式的 URI 快速导入密钥
      </p>
      
      <!-- 输入 URI -->
      <div v-if="!parsed" class="form-group mb-3">
        <textarea 
          class="form-control" 
          v-model="uri"
          rows="4"
          placeholder="otpauth://totp/Example:user@example.com?secret=JBSWY3DPEHPK3PXP&issuer=Example"
        ></textarea>
      </div>
      
      <!-- 解析结果预览 -->
      <div v-else class="flash mb-3">
        <div class="d-flex flex-items-center mb-2">
          <span class="text-small color-fg-muted" style="width: 60px;">名称：</span>
          <strong>{{ parsed.name }}</strong>
        </div>
        <div class="d-flex flex-items-center">
          <span class="text-small color-fg-muted" style="width: 60px;">密钥：</span>
          <code class="text-mono text-small">{{ parsed.secret }}</code>
        </div>
      </div>
      
      <!-- 错误提示 -->
      <div v-if="error" class="flash flash-error mb-3">
        {{ error }}
      </div>
      
      <!-- 成功提示 -->
      <div v-if="success" class="flash flash-success mb-3">
        {{ success }}
      </div>
      
      <!-- 按钮 -->
      <div v-if="!parsed" class="d-flex" style="gap: 12px;">
        <button 
          class="btn btn-primary" 
          @click="parse"
          :disabled="!uri.trim()"
        >
          解析
        </button>
        <button class="btn" @click="router.push('/home')">取消</button>
      </div>
      
      <div v-else class="d-flex" style="gap: 12px;">
        <button 
          class="btn btn-primary" 
          @click="confirmImport"
          :disabled="loading"
        >
          {{ loading ? '导入中...' : '确认导入' }}
        </button>
        <button class="btn" @click="reset">重新输入</button>
      </div>
    </div>
    
    <!-- 说明 -->
    <div class="Box p-3 mt-4" style="max-width: 560px;">
      <h4 class="h5 mb-2">如何获取 URI？</h4>
      <ul class="text-small color-fg-muted pl-3">
        <li>有些网站会提供"复制密钥链接"选项</li>
        <li>或从其他 TOTP 应用导出</li>
        <li>URI 格式：otpauth://totp/标签?secret=密钥&issuer=服务商</li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
code {
  background-color: var(--bgColor-muted);
  padding: 2px 6px;
  border-radius: 3px;
}
</style>

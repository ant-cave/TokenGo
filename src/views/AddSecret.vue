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

// 表单状态
const name = ref('')
const secret = ref('')
const error = ref('')
const success = ref('')
const loading = ref(false)

// 提交表单
async function submit() {
  error.value = ''
  success.value = ''
  
  // 简单验证
  if (!name.value.trim()) {
    error.value = '请输入密钥名称'
    return
  }
  if (!secret.value.trim()) {
    error.value = '请输入密钥'
    return
  }
  
  // 清理密钥（去掉空格，转大写）
  const cleanSecret = secret.value.replace(/\s/g, '').toUpperCase()
  
  // Base32 格式简单检查
  if (!/^[A-Z2-7]+=*$/.test(cleanSecret)) {
    error.value = '密钥格式不正确，应为 Base32 编码'
    return
  }
  
  loading.value = true
  try {
    await invoke('add_totp_secret', {
      request: {
        name: name.value.trim(),
        secret: cleanSecret
      }
    })
    
    success.value = '添加成功！'
    
    // 清空表单
    name.value = ''
    secret.value = ''
    
    // 2秒后跳转到首页
    setTimeout(() => {
      router.push('/home')
    }, 1500)
  } catch (e) {
    error.value = '添加失败: ' + e
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div>
    <h1 class="h2-mktg mb-4">添加密钥</h1>
    
    <div class="Box p-4" style="max-width: 480px;">
      <div class="form-group mb-3">
        <label class="form-label">密钥名称</label>
        <input 
          type="text" 
          class="form-control" 
          v-model="name"
          placeholder="例如：GitHub、Google"
        />
        <p class="note">给这个密钥起个名字方便识别</p>
      </div>
      
      <div class="form-group mb-4">
        <label class="form-label">密钥 (Base32)</label>
        <textarea 
          class="form-control" 
          v-model="secret"
          rows="3"
          placeholder="例如：JBSWY3DPEHPK3PXP"
        ></textarea>
        <p class="note">从服务商处获取的密钥，通常是 16-32 位字符</p>
      </div>
      
      <!-- 错误提示 -->
      <div v-if="error" class="flash flash-error mb-3">
        {{ error }}
      </div>
      
      <!-- 成功提示 -->
      <div v-if="success" class="flash flash-success mb-3">
        {{ success }}
      </div>
      
      <div class="d-flex" style="gap: 12px;">
        <button 
          class="btn btn-primary" 
          @click="submit"
          :disabled="loading"
        >
          {{ loading ? '添加中...' : '添加密钥' }}
        </button>
        <button class="btn" @click="router.push('/home')">取消</button>
      </div>
    </div>
    
    <!-- 提示信息 -->
    <div class="Box p-3 mt-4" style="max-width: 480px;">
      <h4 class="h5 mb-2">如何获取密钥？</h4>
      <ol class="text-small color-fg-muted pl-3">
        <li>登录对应网站的安全设置</li>
        <li>开启两步验证 / 双因素认证</li>
        <li>选择"使用认证器应用"</li>
        <li>复制显示的密钥（不是二维码）</li>
        <li>粘贴到上面的输入框</li>
      </ol>
    </div>
  </div>
</template>

<style scoped>
.note {
  font-size: 12px;
  color: var(--fgColor-muted);
  margin-top: 4px;
  margin-bottom: 0;
}

ol {
  line-height: 1.8;
}
</style>

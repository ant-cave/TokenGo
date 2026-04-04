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
import { computed } from 'vue'

// Element Plus 图标映射
// key - 用于密码列表
const ICON_MAP = {
  key: 'Key',
  plus: 'Plus',
  download: 'Download',
  'sign-out': 'SwitchButton',
  eye: 'View',
  trash: 'Delete',
  gear: 'Setting',
  paintbrush: 'Brush',
  sun: 'Sunny',
  moon: 'Moon',
  desktop: 'Monitor',
  shield: 'Lock',
  check: 'Check',
  alert: 'Warning',
  x: 'Close',
  repo: 'Folder',
  'mark-github': 'GitHub',
  search: 'Search',
  'triangle-down': 'ArrowDown',
  'three-bars': 'Menu',
  person: 'User',
  account: 'User',
  mail: 'Message',
  key2: 'Key'
}

const props = defineProps({
  name: {
    type: String,
    required: true
  },
  size: {
    type: Number,
    default: 16
  }
})

const iconName = computed(() => {
  const mapped = ICON_MAP[props.name]
  return mapped || 'QuestionMark'
})

const style = computed(() => ({
  width: `${props.size}px`,
  height: `${props.size}px`,
  display: 'inline-block',
  verticalAlign: 'text-bottom',
  lineHeight: 1
}))
</script>

<template>
  <span class="icon-wrapper" :style="style">
    <component :is="iconName" />
  </span>
</template>

<style scoped>
.icon-wrapper {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
.icon-wrapper :deep(svg) {
  display: block;
}
</style>

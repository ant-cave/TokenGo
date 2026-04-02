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

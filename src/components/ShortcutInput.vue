/**
* 快捷键输入组件
* 用于设置全局快捷键
* chatgpt
*/

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'
import { NInput, NTag } from 'naive-ui'

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', v: string): void
}>()

const recording = ref(false)
const keys = ref<Set<string>>(new Set())
// 我修改的
const keyWords = computed(() => {
  return props.modelValue.split('+');
});
// 

const displayValue = computed(() => {
  return recording.value
    ? '请按下快捷键…'
    : props.modelValue || '未设置'
})

function normalizeKey(e: KeyboardEvent): string | null {
  const k = e.key

  if (k === 'Control') return 'Ctrl'
  if (k === 'Shift') return 'Shift'
  if (k === 'Alt') return 'Alt'
  if (k === 'Meta') return 'Meta'

  if (k.length === 1) return k.toUpperCase()
  if (k.startsWith('F')) return k

  return null
}

function startRecord() {
  recording.value = true
  // 我修改的
  // keys.value.clear()
  emit('update:modelValue', '');
  // 
  window.addEventListener('keydown', onKeydown, true)
  window.addEventListener('keyup', onKeyup, true)
}

function stopRecord(commit = true) {
  window.removeEventListener('keydown', onKeydown, true)
  window.removeEventListener('keyup', onKeyup, true)

  if (commit && keys.value.size >= 2) {
    emit('update:modelValue', Array.from(keys.value).join('+'))
  }

  recording.value = false
}

function onKeydown(e: KeyboardEvent) {
  e.preventDefault()
  e.stopPropagation()

  if (e.key === 'Escape') {
    stopRecord(false)
    return
  }

  if (e.key === 'Backspace') {
    keys.value.clear()
    emit('update:modelValue', '')
    return
  }

  const key = normalizeKey(e)
  if (!key) return

  keys.value.add(key)
}

function onKeyup() {
  // 松开最后一个键时提交
  if (recording.value && keys.value.size > 0) {
    stopRecord(true)
  }
}

onUnmounted(() => {
  stopRecord(false)
})
</script>

<template>
  <div class="shortcut-input">
    <n-input :value="displayValue" readonly @click="startRecord" />

    <!-- 我修改的 -->
    <div class="preview" v-if="keyWords.length > 1">
      <n-tag v-for="k in keyWords" :key="k" size="small" type="info">
        {{ k }}
      </n-tag>
    </div>
  </div>
</template>

<style scoped>
.shortcut-input {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.preview {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}
</style>

// translate 组件
<script setup lang="ts">
import Controlbar from '@components/Controlbar.vue';
import LangSelect from '@components/LangSelect.vue';
import LangTexarea from '@components/LangTexarea.vue';
import type { TranslateProps } from '@/types/TranslateProps';
import { ref, watch } from 'vue';
import { FromLang, ToLang } from '@/types/Lang';
import { debounce } from 'lodash-es';

const form = ref<TranslateProps>({
  from: 'auto',
  to: 'zh',
  text: '',
  translate: '',
});

// 目标语言选项
const toOptions: { label: string; value: ToLang }[] = [
  { label: '中文', value: 'zh' },
  { label: '英文', value: 'en' },
  { label: '日语', value: 'jp' },
  { label: '韩语', value: 'kor' },
  { label: '法语', value: 'fra' },
  { label: '德语', value: 'de' },
  { label: '西班牙语', value: 'spa' },
  { label: '俄语', value: 'ru' },
  { label: '葡萄牙语', value: 'pt' },
  { label: '意大利语', value: 'it' },
];
// 源语言选项
const fromOptions: { label: string; value: FromLang }[] = [
  { label: '自动检测', value: 'auto' },
  ...toOptions,
];

// 交换源语言和目标语言
const swapLangs = () => {
  if (form.value.from === 'auto') {
    return;
  }

  [form.value.from, form.value.to] = [form.value.to, form.value.from];
};

import { invoke } from "@tauri-apps/api/core";

// 翻译加载状态
const loading = ref(false)
const error = ref('')

// 翻译文本
const translate = async () => {
  if (!form.value.text.trim()) {
    form.value.translate = ''
    return
  }

  loading.value = true
  error.value = ''

  try {
    const result = await invoke<{ translated: string }>("translate_text", {
      req: {
        text: form.value.text,
        source: form.value.from,
        target: form.value.to
      }
    })
    form.value.translate = result.translated
  } catch (err) {
    error.value = err instanceof Error ? err.message : '翻译失败'
  } finally {
    loading.value = false
  }
}

// 监听文本、源语言、目标语言变化，触发翻译
watch(
  () => [form.value.text, form.value.from, form.value.to],
  debounce(() => {
    console.log('翻译了.....')
    translate()
  }, 500)
)

</script>

<template>
  <div class="translate-container">
    <controlbar title="百度翻译软件" />
    <div class="lang-selector">
      <lang-select label="源语言" v-model="form.from" :options="fromOptions" />
      <button class="swap-btn" :class="{ 'disabled': form.from === 'auto' }" @click="swapLangs">↔</button>
      <lang-select label="目标语言" v-model="form.to" :options="toOptions" />
    </div>
    <div class="translate-content">
      <lang-texarea label="输入" v-model="form.text" placeholder="输入需要翻译的文本..." />
      <lang-texarea label="结果" v-model="form.translate" placeholder="翻译结果将显示在这里..." readonly />
    </div>
  </div>
</template>

<style scoped lang="scss">
@use '@/styles/variables.scss' as *;

/* 主容器 - 模拟macOS窗口风格 */
.translate-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  min-width: 500px;
  height: max(200px, 100vh);
  background-color: $white;
  border-radius: $border-radius;
  box-shadow: $shadow-lg;
  overflow: hidden;
  border: $border-width $border-style $border-color;
}

/* 语言选择区域 */
.lang-selector {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: $spacing-lg $spacing-xl;
  border-bottom: $border-width $border-style $border-color;

  .swap-btn {
    width: 2rem;
    height: 2rem;
    border-radius: $border-radius-round;
    border: none;
    background-color: $gray-light;
    color: $text-primary;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: $transition-fast;

    &:hover {
      background-color: $gray-medium;
    }

    &.disabled {
      background-color: $gray-medium;
      color: $text-secondary;
      cursor: not-allowed;
      opacity: 0.6;

      &:hover {
        background-color: $gray-medium;
      }
    }
  }
}

.translate-content {
  display: flex;
  flex-grow: 1;
}

/* 响应式适配 */
@media (max-width: 768px) {
  .translate-content {
    flex-direction: column;
  }
}
</style>

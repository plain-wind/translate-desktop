// config.vue
<script setup lang="ts">
import { NButton, NForm, NFormItem, NInput, NSwitch, useMessage } from 'naive-ui';
import { ref, onMounted } from 'vue';
import type { ConfigProps } from '@/types/ConfigProps';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits<{
  (e: 'close'): void;
}>();

// 引入消息提示组件
const message = useMessage();
// 引入表单数据模型
const form = ref<ConfigProps>({
  app_id: '',
  app_key: '',
  shortcut: '',
  isShortcutEnabled: true,
  isTopmost: false,
});

// 提交表单
const handleSubmit = async () => {
  try {
    await invoke('set_config', {
      config: {
        baidu: {
          appid: form.value.app_id,
          secret: form.value.app_key,
        },
        shortcut: {
          key: form.value.shortcut || '',
          enabled: form.value.isShortcutEnabled,
        },
      },
    });
    // 关闭配置弹窗
    emit('close');
    message.success('配置保存成功');
  } catch (error) {
    console.error(error);
    message.error('请检查配置信息');
  }
}

// 重置表单
const handleReset = () => {
  form.value = {
    app_id: '',
    app_key: '',
    shortcut: '',
    isShortcutEnabled: true,
    isTopmost: false,
  };
};

// 组件挂载时读取配置
onMounted(async () => {
  try {
    const config = await invoke<any>('get_config');

    if (config?.baidu) {
      form.value.app_id = config.baidu.appid || '';
      form.value.app_key = config.baidu.secret || '';
      form.value.shortcut = config.shortcut.key || '';
      form.value.isShortcutEnabled = config.shortcut.enabled || true;
      form.value.isTopmost = config.isTopmost || false;
    }
  } catch (e) {
    console.error('读取配置失败', e);
  }
})
</script>

<template>
  <div class="config">
    <n-form label-placement="left" :model="form">
      <n-form-item label="App ID" prop="app_id">
        <n-input size="large" v-model:value="form.app_id" placeholder="请输入百度翻译 App ID" />
      </n-form-item>
      <n-form-item label="App Key" prop="app_key">
        <n-input size="large" v-model:value="form.app_key" placeholder="请输入百度翻译 App Key" />
      </n-form-item>
      <n-form-item label="设置快捷键">
        <n-input :disabled="!form.isShortcutEnabled" size="large" v-model:value="form.shortcut"
          placeholder="例如Ctrl+F7" />
      </n-form-item>
      <n-form-item label="快捷键开关">
        <n-switch v-model:value="form.isShortcutEnabled" size="large" />
      </n-form-item>
      <n-form-item label="置顶窗口">
        <n-switch v-model:value="form.isTopmost" size="large" />
      </n-form-item>
      <n-form-item>
        <div class="form-actions">
          <n-button type="primary" @click="handleSubmit" size="large" round>
            保存配置
          </n-button>
          <n-button @click="handleReset" size="large" round>
            重置
          </n-button>
        </div>
      </n-form-item>
    </n-form>
  </div>
</template>

<style scoped lang="scss">
@use '@/styles/variables.scss' as *;

/* 主容器 - 与Translate组件保持一致的macOS窗口风格 */
.config {
  width: 500px;
  height: fit-content;
  background-color: $white;
  border-radius: $border-radius;
  box-shadow: $shadow-lg;
  overflow: auto;
  scrollbar-width: none;
  border: $border-width $border-style $border-color;
}

/* 表单样式 - 与Translate组件风格统一 */
:deep(.n-form) {
  display: flex;
  flex-direction: column;
  gap: $spacing-md;
  padding: $spacing-lg;
  background: $white;
}

:deep(.n-form-item) {
  display: flex;
  align-items: center;
}

:deep(.n-form-item-label) {
  color: $text-secondary;
  font-weight: $font-weight-medium;
  font-size: $font-size-base;
}

:deep(.n-form-item-blank) {
  flex: 1;
}

/* 输入框样式 - 与Translate组件的选择器风格统一 */
:deep(.n-input) {
  border-radius: $border-radius;
  border: $border-width $border-style $border-color;
  transition: $transition-fast;

  &:hover {
    border-color: rgba(0, 0, 0, 0.15);
    box-shadow: none;
  }

  &:focus {
    border-color: $primary-color;
    background-color: $white;
    box-shadow: 0 0 0 2px rgba($primary-color, 0.2);
  }
}

:deep(.n-input__input) {
  font-size: $font-size-base;
  color: $text-primary;
  background-color: transparent;
}

:deep(.n-base-input__state-border) {
  border-radius: $border-radius;
}

// :deep(.n-form-item-feedback) {
//   font-size: $font-size-sm;
//   color: #ff4d4f;
// }

/* 按钮区域样式 */
.form-actions {
  display: flex;
  flex: 1;
  gap: $spacing-md;
  justify-content: flex-end;
  margin-top: $spacing-md;
  // padding: $spacing-lg $spacing-xl;
}

/* 按钮样式 - 与Translate组件的按钮风格统一 */
:deep(.n-button) {
  font-weight: $font-weight-medium;
  font-size: $font-size-base;
  border-radius: $border-radius;
  transition: $transition-fast;
  box-shadow: none;
  border: $border-width $border-style $border-color;

  &:hover {
    transform: none;
    box-shadow: none;
  }

  &:active {
    transform: none;
  }
}

:deep(.n-button--primary) {
  background: $primary-gradient;
  border: none;
  color: $white;

  &:hover {
    opacity: 0.9;
  }
}

:deep(.n-button--default) {
  background-color: $gray-light;
  border: $border-width $border-style $border-color;
  color: $text-primary;

  &:hover {
    background-color: $gray-medium;
    border-color: rgba(0, 0, 0, 0.15);
  }
}
</style>
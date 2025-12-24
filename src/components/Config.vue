// config.vue
<script setup lang="ts">
import { NButton, NForm, NFormItem, NInput, NSwitch, useMessage } from 'naive-ui';
import { ref, onMounted } from 'vue';
import type { ConfigProps } from '@/types/ConfigProps';
import { invoke } from '@tauri-apps/api/core';
import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';
import ShortcutInput from '@/components/ShortcutInput.vue';

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
  isShortcutEnabled: false,
  isTopmost: false,
  isAutoStart: false,
});
// 表单副本
const formCopy: ConfigProps = {
  app_id: '',
  app_key: '',
  shortcut: '',
  isShortcutEnabled: false,
  isTopmost: false,
  isAutoStart: false,
};


const autoStart = async () => {
  // 是否为开发环境
  const isDev = import.meta.env.MODE === 'development';

  // 开发环境下不启用自启动
  if (isDev) {
    return;
  }

  // 自启动状态未改变时不执行
  if (form.value.isAutoStart === formCopy.isAutoStart) {
    return;
  }

  if (form.value.isAutoStart) {
    await enable();
    const enabled = await isEnabled();
    if (!enabled) {
      throw new Error('自启动启用失败');
    }
    message.info('已尝试开启自启动，如未生效，请检查系统启动项或安全软件');
  } else {
    await disable();
  }
}


// 提交表单
const handleSubmit = async () => {
  try {
    // 启用/禁用自启动
    await autoStart();
    // 保存配置
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
        is_topmost: form.value.isTopmost,
        is_auto_start: form.value.isAutoStart,
      },
    });
    // 设置置顶窗口
    await invoke('set_topmost', { topmost: form.value.isTopmost });
    // 热重载快捷键
    await invoke('reload_shortcut');


    // 关闭配置弹窗
    emit('close');
    message.success('配置保存成功');
  } catch (error: any) {
    console.error(error);
    message.error(error);
  }
}

// 撤销表单
const handleUndo = () => {
  form.value = { ...formCopy };
};

// 组件挂载时读取配置
onMounted(async () => {
  try {
    const config = await invoke<any>('get_config');

    if (config?.baidu) {
      formCopy.app_id = config.baidu.appid || '';
      formCopy.app_key = config.baidu.secret || '';
      formCopy.shortcut = config.shortcut?.key || '';
      formCopy.isShortcutEnabled = config.shortcut?.enabled ?? false;
      formCopy.isTopmost = config.is_topmost ?? false;
      formCopy.isAutoStart = config.is_auto_start ?? false;

      form.value = { ...formCopy };
    }
  } catch (e) {
    console.error('读取配置失败', e);
  }
})
</script>

<template>
  <div class="config">
    <n-form label-placement="left" label-width="auto" :model="form">
      <n-form-item label="App ID" prop="app_id">
        <n-input size="large" v-model:value="form.app_id" placeholder="请输入百度翻译 App ID" />
      </n-form-item>
      <n-form-item label="App Key" prop="app_key">
        <n-input size="large" v-model:value="form.app_key" placeholder="请输入百度翻译 App Key" />
      </n-form-item>
      <n-form-item label="设置快捷键">
        <ShortcutInput v-model="form.shortcut" />
      </n-form-item>
      <n-form-item label="快捷键开关">
        <n-switch v-model:value="form.isShortcutEnabled" size="large" />
      </n-form-item>
      <n-form-item label="置顶窗口">
        <n-switch v-model:value="form.isTopmost" size="large" />
      </n-form-item>
      <n-form-item label="自动启动">
        <n-switch v-model:value="form.isAutoStart" size="large" />
      </n-form-item>
      <n-form-item>
        <div class="form-actions">
          <n-button type="primary" @click="handleSubmit" size="large" round>
            保存配置
          </n-button>
          <n-button @click="handleUndo" size="large" round>
            撤销
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
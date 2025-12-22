// config.vue
<script setup lang="ts">
import Controlbar from '@components/Controlbar.vue';
import { NButton, NForm, NFormItem, NInput, useMessage } from 'naive-ui';
import type { FormInst, FormRules } from 'naive-ui';
import { ref } from 'vue';
import type { ConfigProps } from '@/types/ConfigProps';

const message = useMessage();

const formRef = ref<FormInst | null>(null);

const form = ref<ConfigProps>({
  app_id: '',
  app_key: '',
});

const rules: FormRules = {
  app_id: [
    { required: true, message: '请输入 App ID', trigger: 'blur' },
  ],
  app_key: [
    { required: true, message: '请输入 App Key', trigger: 'blur' },
  ],
};

const handleSubmit = async () => {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    // 这里可以添加保存配置的逻辑
    message.success('配置保存成功');
    console.log('保存的配置:', form.value);
  } catch (error) {
    message.error('请检查配置信息');
  }
};

const handleReset = () => {
  form.value = {
    app_id: '',
    app_key: '',
  };
  if (formRef.value) {
    formRef.value.restoreValidation();
  }
};

</script>

<template>
  <div class="config">
    <controlbar title="百度翻译配置" />
    <n-form :model="form" :rules="rules" ref="formRef">
      <n-form-item label="App ID" prop="app_id">
        <n-input v-model:value="form.app_id" placeholder="请输入百度翻译 App ID" />
      </n-form-item>
      <n-form-item label="App Key" prop="app_key">
        <n-input v-model:value="form.app_key" placeholder="请输入百度翻译 App Key" />
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

/* 表单样式 - 与Translate组件风格统一 */
:deep(.n-form) {
  padding: $spacing-lg $spacing-xl;
  background: $white;
  border-bottom: $border-width $border-style $border-color;
  display: flex;
  flex-direction: column;
  gap: $spacing-md;
}

:deep(.n-form-item) {
  margin-bottom: 0;
  display: flex;
  flex-direction: column;
  gap: $spacing-sm;
}

:deep(.n-form-item-label) {
  color: $text-secondary;
  font-weight: $font-weight-medium;
  font-size: $font-size-base;
  margin-bottom: 0;
}

/* 输入框样式 - 与Translate组件的选择器风格统一 */
:deep(.n-input) {
  border-radius: $border-radius;
  border: $border-width $border-style $border-color;
  // background-color: $gray-light;
  transition: $transition-fast;

  &:hover {
    border-color: rgba(0, 0, 0, 0.15);
    // background-color: $gray-medium;
    box-shadow: none;
  }

  &:focus {
    border-color: $primary-color;
    background-color: $white;
    box-shadow: 0 0 0 2px rgba($primary-color, 0.2);
  }
}

:deep(.n-input__input) {
  padding: 10px 12px;
  font-size: $font-size-base;
  color: $text-primary;
  background-color: transparent;
}

:deep(.n-base-input__state-border) {
  border-radius: $border-radius;
}

:deep(.n-form-item-feedback) {
  font-size: $font-size-sm;
  color: #ff4d4f;
  margin-top: $spacing-xs;
}

/* 按钮区域样式 */
.form-actions {
  display: flex;
  gap: $spacing-md;
  justify-content: flex-start;
  margin-top: $spacing-md;
  padding: $spacing-lg $spacing-xl;
}

/* 按钮样式 - 与Translate组件的按钮风格统一 */
:deep(.n-button) {
  font-weight: $font-weight-medium;
  font-size: $font-size-base;
  padding: 10px 24px;
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
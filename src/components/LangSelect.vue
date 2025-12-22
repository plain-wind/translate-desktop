<script setup lang="ts">
import { FromLang, ToLang } from '@/types/Lang';
import { ref, computed, toRefs } from 'vue';

const emit = defineEmits<{
  (e: 'update:modelValue', value: FromLang | ToLang): void;
}>();

const props = withDefaults(defineProps<{
  label: string;
  modelValue: FromLang | ToLang;
  icon?: string;
  options: { label: string; value: FromLang | ToLang }[];
}>(), {
  icon: '▼',
});

const { label, modelValue, icon, options } = toRefs(props);

// 下拉框是否激活
const isSelectActive = ref(false);
// 下拉框显示的文本
const text = computed(() => options.value.find(item => item.value === modelValue.value)?.label || modelValue.value);

// 切换下拉框激活状态
const changeSelect = () => {
  isSelectActive.value = !isSelectActive.value;
}

// 选择下拉框选项
const changeOption = (value: FromLang | ToLang) => {
  emit('update:modelValue', value);
  isSelectActive.value = false;
}

</script>

<template>
  <div class="container">
    <div class="label">{{ label }}</div>
    <div class="custom-select">
      <div class="select-trigger" :class="{ 'active': isSelectActive }" @click="changeSelect">
        <span class="select-text">{{ text }}</span>
        <span class="select-arrow">{{ icon }}</span>
      </div>
      <div class="select-options" :class="{ 'show': isSelectActive }">
        <div v-for="{ label, value } in options" :key="value" class="select-option"
          :class="{ 'active': value === modelValue }" @click="changeOption(value);">
          {{ label }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@use '@/styles/variables.scss' as *;

.container {
  display: flex;
  align-items: center;

  .label {
    font-size: $font-size-base;
    color: $text-secondary;
    margin-right: $spacing-md;
  }

  /* ========== 纯自定义下拉框 ========== */
  .custom-select {
    position: relative;
    width: 7rem;
    font-size: $font-size-base;

    /* 下拉框触发按钮 - 调整右侧内边距 */
    .select-trigger {
      width: 100%;
      padding: $spacing-sm $spacing-md;
      border-radius: $border-radius;
      border: $border-width $border-style $border-color;
      background-color: $gray-light;
      color: $text-primary;
      cursor: pointer;
      transition: $transition-fast;
      display: flex;
      justify-content: space-between;
      align-items: center;

      &:hover {
        background-color: $gray-medium;
        border-color: rgba(0, 0, 0, 0.15);
      }

      /* 下拉箭头 - 调整定位，拉近边框 */
      .select-arrow {
        font-size: $font-size-sm - 4px;
        color: $text-secondary;
        transition: transform $transition-fast;
        margin-left: $spacing-xs;
        /* 保证和文字有最小间距 */
      }

      &.active .select-arrow {
        transform: rotate(180deg);
      }
    }

    /* 下拉选项面板 */
    .select-options {
      position: absolute;
      top: 100%;
      left: 0;
      width: 100%;
      background-color: $white;
      border: $border-width $border-style $border-color;
      border-radius: $border-radius;
      box-shadow: $shadow-md;
      overflow: hidden;
      max-height: 0;
      opacity: 0;
      visibility: hidden;
      transition: $transition-fast;
      z-index: 100;

      &.show {
        max-height: 200px;
        opacity: 1;
        visibility: visible;
        overflow-y: auto;
        scrollbar-width: none;
      }
    }

    /* 下拉选项 */
    .select-option {
      padding: 0.6rem 1rem;
      color: $text-primary;
      cursor: pointer;
      transition: $transition-fast;
      border-radius: $border-radius - 2px;
      margin: $spacing-xs;

      &:hover {
        background-color: $gray-medium;
      }

      &.active {
        background-color: rgba($primary-color, 0.1);
        color: $primary-color;
      }
    }
  }
}
</style>
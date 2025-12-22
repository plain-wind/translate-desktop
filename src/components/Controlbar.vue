<script setup lang="ts">
import { NIcon } from 'naive-ui';
import Config from '@/components/Config.vue';
import Mask from '@/components/Mask.vue';
import { ref } from 'vue';

defineProps<{
  title: string;
}>();

const isShowConfig = ref(false);
</script>

<template>
  <div class="controlbar">
    <div class="button">
      <div class="close"></div>
      <div class="minimize"></div>
      <div class="zoom"></div>
    </div>
    <div class="title">{{ title }}</div>
    <div class="setting">
      <n-icon size="20" @click="isShowConfig = true">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 32 32">
          <path
            d="M27 16.76V16v-.77l1.92-1.68A2 2 0 0 0 29.3 11l-2.36-4a2 2 0 0 0-1.73-1a2 2 0 0 0-.64.1l-2.43.82a11.35 11.35 0 0 0-1.31-.75l-.51-2.52a2 2 0 0 0-2-1.61h-4.68a2 2 0 0 0-2 1.61l-.51 2.52a11.48 11.48 0 0 0-1.32.75l-2.38-.86A2 2 0 0 0 6.79 6a2 2 0 0 0-1.73 1L2.7 11a2 2 0 0 0 .41 2.51L5 15.24v1.53l-1.89 1.68A2 2 0 0 0 2.7 21l2.36 4a2 2 0 0 0 1.73 1a2 2 0 0 0 .64-.1l2.43-.82a11.35 11.35 0 0 0 1.31.75l.51 2.52a2 2 0 0 0 2 1.61h4.72a2 2 0 0 0 2-1.61l.51-2.52a11.48 11.48 0 0 0 1.32-.75l2.42.82a2 2 0 0 0 .64.1a2 2 0 0 0 1.73-1l2.28-4a2 2 0 0 0-.41-2.51zM25.21 24l-3.43-1.16a8.86 8.86 0 0 1-2.71 1.57L18.36 28h-4.72l-.71-3.55a9.36 9.36 0 0 1-2.7-1.57L6.79 24l-2.36-4l2.72-2.4a8.9 8.9 0 0 1 0-3.13L4.43 12l2.36-4l3.43 1.16a8.86 8.86 0 0 1 2.71-1.57L13.64 4h4.72l.71 3.55a9.36 9.36 0 0 1 2.7 1.57L25.21 8l2.36 4l-2.72 2.4a8.9 8.9 0 0 1 0 3.13L27.57 20z"
            fill="currentColor"></path>
          <path
            d="M16 22a6 6 0 1 1 6-6a5.94 5.94 0 0 1-6 6zm0-10a3.91 3.91 0 0 0-4 4a3.91 3.91 0 0 0 4 4a3.91 3.91 0 0 0 4-4a3.91 3.91 0 0 0-4-4z"
            fill="currentColor"></path>
        </svg>
      </n-icon>
    </div>
  </div>
  <teleport to="body">
    <Mask v-if="isShowConfig">
      <Config @close="isShowConfig = false" />
    </Mask>
  </teleport>
</template>

<style scoped lang="scss">
@use '@/styles/variables.scss' as *;

.controlbar {
  height: 3rem;
  display: flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: flex-start;
  padding: 0 2rem;
  background: $controlbar-gradient;
  color: $font-color;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  user-select: none;
}

.button {
  display: flex;
  align-items: center;
  gap: 10px;

  // mac 风格的关闭、最小化、最大化按钮
  .close,
  .minimize,
  .zoom {
    width: $button-size;
    height: $button-size;
    border-radius: $border-radius-round;
    transition: $transition-base;
    cursor: pointer;
    position: relative;
    overflow: hidden;
    box-shadow: $shadow-sm;

    &:hover {
      transform: scale(1.1);
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    }

    &:active {
      transform: scale(0.95);
    }
  }

  // 关闭按钮
  .close {
    background-color: $red;
  }

  // 最小化按钮
  .minimize {
    background-color: $yellow;
  }

  // 最大化按钮
  .zoom {
    background-color: $green;
  }
}

.title {
  font-weight: $font-weight-medium;
  color: $font-color;
  letter-spacing: 0.5px;
  margin-left: $spacing-lg;
}

.setting {
  display: flex;
  align-items: center;
  margin-left: auto;
  transition: $transition-base;

  &:hover {
    transform: scale(1.1);
    cursor: pointer;
  }
}
</style>

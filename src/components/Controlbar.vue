<script setup lang="ts">
defineProps<{
  title: string;
}>();
</script>

<template>
  <div class="controlbar">
    <div class="button">
      <div class="close"></div>
      <div class="minimize"></div>
      <div class="zoom"></div>
    </div>
    <div class="title">{{ title }}</div>
  </div>
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

.title {
  margin-left: $spacing-lg;
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
}
</style>

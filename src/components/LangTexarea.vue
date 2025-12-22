<script setup lang="ts">
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

withDefaults(defineProps<{
  label: string;
  modelValue: string;
  placeholder: string;
  readonly?: boolean;
}>(), {
  readonly: false,
});

const handleInput = ({ target }: InputEvent) => {
  const { value } = target as HTMLTextAreaElement;
  emit('update:modelValue', value);
};



</script>
<template>
  <div class="area" :id="readonly ? 'readonly' : 'default'">
    <span class="label">{{ label }}</span>
    <textarea class="text-editor" :class="{ 'readonly': readonly }" :placeholder="placeholder" :readonly="readonly"
      @input="handleInput">{{ modelValue }}</textarea>
  </div>
</template>

<style scoped lang="scss">
@use '@/styles/variables.scss' as *;

#default {
  border-right: $border-width $border-style $border-color;
}

.area {
  flex: 1;
  padding: $spacing-lg $spacing-xl;
  position: relative;

  .label {
    font-size: $font-size-sm;
    color: $text-secondary;
    margin-bottom: $spacing-sm;
    display: block;
  }

  .text-editor {
    width: 100%;
    height: calc(100% - 30px);
    border: none;
    outline: none;
    resize: none;
    font-size: $font-size-lg;
    color: $text-primary;
    background-color: transparent;
    line-height: 1.6;
    scrollbar-width: none;

    &.readonly {
      color: $text-primary;
      cursor: default;
    }
  }
}

/* 响应式适配 */
@media (max-width: 768px) {
  .translate-content {
    flex-direction: column;
  }

  #default {
    border-right: none;
    border-bottom: $border-width $border-style $border-color;
  }
}
</style>
<template>
  <q-input
    :model-value="displayValue"
    type="number"
    outlined
    dense
    input-class="dt-field__input dt-field__input--left"
    @update:model-value="handleInput"
  >
    <template v-if="showControls" #prepend>
      <q-btn flat dense round icon="remove" @click="stepDown" />
    </template>
    <template v-if="suffix" #append>
      <span class="dt-field__suffix">{{ suffix }}</span>
    </template>
    <template v-if="showControls" #after>
      <q-btn flat dense round icon="add" @click="stepUp" />
    </template>
  </q-input>
</template>

<script setup>
const props = defineProps({
  modelValue: {
    type: [Number, String, null],
    default: "",
  },
  min: {
    type: Number,
    default: undefined,
  },
  max: {
    type: Number,
    default: undefined,
  },
  step: {
    type: Number,
    default: 1,
  },
  precision: {
    type: Number,
    default: undefined,
  },
  suffix: {
    type: String,
    default: "",
  },
  showControls: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(["update:modelValue"])

const displayValue = computed(() => (props.modelValue ?? "") === "" ? "" : String(props.modelValue))

function clamp(value) {
  let next = value

  if (typeof props.min === "number") {
    next = Math.max(props.min, next)
  }
  if (typeof props.max === "number") {
    next = Math.min(props.max, next)
  }
  if (typeof props.precision === "number") {
    next = Number(next.toFixed(props.precision))
  }

  return next
}

function toNumber(value) {
  const parsed = Number(value)
  return Number.isNaN(parsed) ? null : clamp(parsed)
}

function handleInput(value) {
  if (value === "" || value === null || value === undefined) {
    emit("update:modelValue", "")
    return
  }

  const parsed = toNumber(value)
  emit("update:modelValue", parsed ?? props.modelValue)
}

function stepBy(delta) {
  const current = typeof props.modelValue === "number" ? props.modelValue : Number(props.modelValue || 0)
  emit("update:modelValue", clamp(current + delta))
}

function stepDown() {
  stepBy(-props.step)
}

function stepUp() {
  stepBy(props.step)
}
</script>

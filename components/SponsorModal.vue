<script setup lang="ts">
defineEmits(["close-modal"])
defineProps({
  modalActive: {
    type: Boolean,
    default: false,
  },
})
</script>

<template>
  <div
    v-show="modalActive"
    class="dark:text-white absolute place-content-center text-center text-sm w-full bg-sky-800 bg-opacity-30 flex top-0 left-0 px-3 pt-2"
  >
    <div
      v-if="modalActive"
      class="absolute w-full bg-sky-800 bg-opacity-30 flex justify-center px-3 pb-2"
    >
      If you'd like to sponsor me, you can send me ETH!
      <br />
      <br />
      0x9B707cB77D0c2995Bd6D3B593ecabd39614acc88
      <div class="place-content-center">
      </div>
    </div>
  </div>
</template>

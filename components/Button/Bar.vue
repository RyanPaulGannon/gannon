<script setup lang="ts">
const modalActive = ref(false)
function toggleModal() {
  modalActive.value = !modalActive.value
}
</script>

<template>
  <button p-1>
    <ColorModeToggle />
  </button>
  <NuxtLink to="https://twitter.com/RyanPaulGannon" target="_blank" p-1>
    <Icon name="twitter" />
  </NuxtLink>
  <NuxtLink to="https://github.com/RyanPaulGannon" target="_blank" p-1>
    <Icon name="github" />
  </NuxtLink>
  <button to="" target="" p-1 @click.prevent="toggleModal">
    <Icon name="ethereum" />
  </button>
</template>

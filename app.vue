<script setup lang="ts">
const modalActive = ref(false)
const toggleModal = () => {
  modalActive.value = !modalActive.value
}
</script>

<template>
  <div class="bg-white dark:bg-slate-800 h-screen">
    <div class="pt-20">
      <div class="flex place-content-center">
        <button class="p-1">
          <ThemeToggle />
        </button>
        <NuxtLink
          to="https://twitter.com/RyanPaulGannon"
          target="_blank"
          class="p-1"
        >
          <Icon name="twitter" />
        </NuxtLink>
        <NuxtLink
          to="https://github.com/RyanPaulGannon"
          target="_blank"
          class="p-1"
        >
          <Icon name="github" />
        </NuxtLink>
        <button to="" target="" class="p-1" @click.prevent="toggleModal">
          <Icon name="ethereum" />
        </button>
      </div>
    </div>
    <div class="place-content-center p-1">
      <SponsorModal :modalActive="modalActive" @close-modal="toggleModal" />
      <div
        class="dark:text-white border-solid border-2 rounded border-sky-500 max-w-2xl px-4 py-10 m-auto bg-white dark:bg-slate-800 sm:px-8 sm:shadow"
      >
        <h1 class="text-2xl">Ryan Paul Gannon</h1>
        <br />
        <div>
          I'm Ryan Gannon, currently a freelance Software Developer at
          <a href="https://orderio.me">Orderio</a>.

          <div class="pt-4">
            You can find me contributing to Open Source on GitHub, and working
            on some personal projects.
          </div>

          <div class="pt-4">
            I've a keen sporting interest, mostly for ⚽️, 🏏 and the NFL 🏈.
          </div>

          <div class="pt-4">Some projects I'm working on:</div>

          <div class="grid md:grid-cols-3 place-content-center text-center">
            <NuxtLink
              to="https://fcwesthoughton.co.uk"
              target="_blank"
              class="p-1 hover:text-teal-300"
              >FC Westhoughton
            </NuxtLink>
            <NuxtLink
              to="https://merklin.xyz"
              target="_blank"
              class="p-1 hover:text-teal-300"
              >Merklin
            </NuxtLink>
            <NuxtLink
              to="https://fcc-headerparser-api.azurewebsites.net/"
              target="_blank"
              class="p-1 hover:text-teal-300"
              >Node Server
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

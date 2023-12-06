<script setup lang="ts">
const colorMode = useColorMode()

const modalActive = ref(false)
function toggleModal () {
  modalActive.value = !modalActive.value
}
</script>

<template>
    <div h-screen bg-white dark:bg-black> 
    <div pt-20>
      <div class="flex place-content-center">
        <button p-1>
            <!-- <ColorModeToggle /> -->
          <ColorModeIconLight />
        </button>
        <NuxtLink
          to="https://twitter.com/RyanPaulGannon"
          target="_blank"
          p-1 
        >
          <Icon name="twitter" />
        </NuxtLink>
        <NuxtLink
          to="https://github.com/RyanPaulGannon"
          target="_blank"
          p-1 
        >
          <Icon name="github" />
        </NuxtLink>
        <button to="" target="" p-1 @click.prevent="toggleModal">
          <Icon name="ethereum" />
        </button>
      </div>
    </div>
    <div p-1>
      <SponsorModal :modalActive="modalActive" @close-modal="toggleModal" />
      <div
        class="dark:text-white border-solid border-2 rounded border-sky-500 max-w-2xl px-4 py-10 m-auto bg-white dark:bg-slate-800 sm:px-8 sm:shadow"
      >
      <h1 text="black 5xl hover:blue" fw100>Ryan Paul Gannon</h1>
        <div pt-4 text-black>
          I'm Ryan Gannon, currently a freelance Software Developer at
          <a href="https://ekatree.com">Ekatree</a>.

          <div pt-4 text-black>
            You can find me contributing to Open Source on GitHub, and working
            on some personal projects.
          </div>

          <div pt-4>
            I've a keen sporting interest, mostly for ⚽️, 🏏 and the NFL 🏈.
          </div>

          <div pt-4>Some projects I'm working on:</div>

          <div class="grid md:grid-cols-3 place-content-center text-center">
            <NuxtLink
              to="https://fcwesthoughton.co.uk"
              target="_blank"
              p-1 
              text="black hover:teal-300"
              >FC Westhoughton
            </NuxtLink>
            <NuxtLink
              to="https://merklin.xyz"
              target="_blank"
              p-1
              text="black hover:teal-300"
              >Merklin
            </NuxtLink>
            <NuxtLink
              to="https://github.com/scribe-org/Scribe-Desktop"
              target="_blank"
              p-1 
              text="black hover:teal-300"
              >Scribe Desktop
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
body {
  background-color: #fff;
  color: rgba(0,0,0,0.8);
}
.dark-mode body {
  background-color: #091a28;
  color: #ebf4f1;
}
.sepia-mode body {
  background-color: #f1e7d0;
  color: #433422;
}
</style>

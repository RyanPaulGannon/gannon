<script setup lang="ts">
const modalActive = ref(false)
function toggleModal() {
  modalActive.value = !modalActive.value
  return 'test'
}
</script>

<template>
  <div h-screen bg-white dark:bg-black>
    <div pt-20>
      <div class="flex place-content-center">
      </div>
    </div>
    <div p-1>
      <SponsorModal :modalActive="modalActive" @close-modal="toggleModal" />
      <div
        class="dark:text-white border-solid border-2 rounded border-sky-500 max-w-2xl px-4 py-10 m-auto bg-white dark:bg-slate-800 sm:px-8 sm:shadow">
        <h1 text="black 5xl hover:blue" fw100>Ryan Paul Gannon</h1>
        <div pt-4 text="black dark:red">
          Bonjour, I'm Ryan, enchanté! I'm a Full Stack Developer from
          Manchester, UK 🏴󠁧󠁢󠁥󠁮󠁧󠁿
          <br />
          Currently working at
          <a class="hover:text-blue" href="https://ekatree.com">Ekatree</a>.

          <br />
          <br />
          You can find me contributing to Open Source on GitHub, and working on
          some personal projects ou apprendre le francais.

          <br />
          <br />
          I've a keen sporting interest, mostly for ⚽️, 🏏 and the NFL 🏈.

          <div pt-4>Some projects I'm working on:</div>

          <div class="grid md:grid-cols-3 place-content-center p-5 text-center">
            <div class="md:col-span-1"></div>
            <!-- Empty column for centering -->
            <div class="md:col-span-1">
              <Carousel />
            </div>
            <!-- Carousel centered in the second column -->
            <div class="md:col-span-1"></div>
            <!-- Empty column for centering -->
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
body {
  background-color: #fff;
  color: rgba(0, 0, 0, 0.8);
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

<script setup lang="ts">
import { ref } from 'vue'

const showModal = ref(false)

function flashModal() {
    showModal.value = true
    setTimeout(() => {
        showModal.value = false
    }, 4000)
}
</script>

<template>
    <div>
        <a href="https://rust-lang.org" target="_blank">
            <img src="/rust.svg" class="logo" alt="Rust logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
            <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
        </a>
    </div>

    <h1>Ryan Paul Gannon</h1>

    <div class="card">
        <button type="button" @click="flashModal">Hire me?</button>
    </div>

    <a href="mailto:ryan@gannon.one" v-if="showModal">ryan@gannon.one</a>

    <p>
        Bonjour, I'm Ryan, enchanté! I'm a Full Stack Developer from
        🏴󠁧󠁢󠁥󠁮󠁧󠁿 Manchester, UK
        <br />
        Currently working at
        <a class="hover:text-blue" href="https://ekatree.com">Ekatree</a>.
    </p>
    <p>
        You can find me contributing to Open Source on GitHub (with a particular interest in memory management
        <br />
        and Rust), and working on some personal projects ou apprendre le francais.
    </p>
    <p>
        I've a keen sporting interest, mostly for ⚽️, 🏏 and the NFL 🏈. I'll talk almost anything sport!
    </p>

    <!-- <div pt-4>Some projects I'm working on/contributing to:</div> -->
</template>

<style scoped>
.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
}

.logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #42b883aa);
}
</style>

<script setup lang="ts">
const currentIndex = ref(0)
const carouselLinks = ref([
  { url: 'https://fcwesthoughton.co.uk', target: '_blank', text: 'FC Westhoughton' },
  { url: 'https://merklin.xyz', target: '_blank', text: 'Merklin' },
  { url: 'https://github.com/scribe-org/Scribe-Desktop', target: '_blank', text: 'Scribe Desktop' },
])

function next() {
  currentIndex.value = (currentIndex.value + 1) % carouselLinks.value.length
}

function prev() {
  currentIndex.value = (currentIndex.value - 1 + carouselLinks.value.length) % carouselLinks.value.length
}
</script>

<template>
  <div class="carousel">
    <transition name="slide-fade">
      <div :key="currentIndex" class="carousel-item">
        <!-- Navigation symbols within carousel item -->
        <span @click="prev" class="nav-symbol">&lt;</span>
        <template v-if="typeof carouselLinks[currentIndex] === 'object'">
          <a class="item" v-if="carouselLinks[currentIndex].url" :href="carouselLinks[currentIndex].url"
            :target="carouselLinks[currentIndex].target">
            {{ carouselLinks[currentIndex].text }}
          </a>
        </template>
        <template v-else>
          {{ carouselLinks[currentIndex] }}
        </template>
        <span @click="next" class="nav-symbol">&gt;</span>
      </div>
    </transition>
  </div>
</template>

<style>
.carousel {
  position: relative;
}

.carousel-item {
  position: relative;
  width: calc(100% - 20px);
  /* Considering 20px padding for the container */
  min-height: 20px;
  /* Minimum height of 20 pixels */
  border-radius: 15px;
  background-color: #f0f0f0;
  padding: 10px;
  box-sizing: border-box;
  /* Ensures padding is included in the width */
  /* Add any other styles you need */
}

/* Styling for navigation symbols */
.nav-symbol {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  cursor: pointer;
  font-size: 20px;
  /* Adjust size as needed */
  /* Add any other styles you need */
}

.nav-symbol:first-child {
  left: 5px;
}

.nav-symbol:last-child {
  right: 5px;
}
</style>

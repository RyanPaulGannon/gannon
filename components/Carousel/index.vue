<script setup lang="ts">
const currentIndex = ref(0)
const carouselLinks = ref([
  {
    url: 'https://fcwesthoughton.co.uk', altText: 'FC Westhoughton', image: 'https://pbs.twimg.com/profile_images/1462172131693899777/mgSt2lZl_400x400.jpg'
  },
  { url: 'https://merklin.xyz', altText: 'Merklin', image: 'https://user-images.githubusercontent.com/2703233/213878836-a1b509a5-264a-41a6-ad51-c7d7db6e696c.jpg' },
  { url: 'https://github.com/scribe-org/Scribe-Desktop', altText: 'Scribe Desktop', image: 'https://raw.githubusercontent.com/scribe-org/Organization/main/logo/ScribeGitHubOrgBanner.png' },
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
        <span @click="prev" class="nav-symbol prev">◄</span>
        <a :href="carouselLinks[currentIndex].url" target="_blank">
          <NuxtImg :src="carouselLinks[currentIndex].image" :placeholder="[30, 20]" width="300" height="200"
            class="rounded" />
        </a>
        <span @click="next" class="nav-symbol next">►</span>
      </div>
    </transition>
  </div>
</template>

<style>
.carousel {
  position: relative;
  width: 300px;
  height: 200px;
  overflow: hidden;
  border-radius: 5px;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  margin-top: 10px;
}

.carousel-item {
  position: relative;
}

.carousel-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 5px;
}

/* Navigation symbols */
.nav-symbol {
  font-size: 24px;
  color: white;
  cursor: pointer;
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  z-index: 1;
}

.prev {
  left: 10px;
}

.next {
  right: 10px;
}
</style>


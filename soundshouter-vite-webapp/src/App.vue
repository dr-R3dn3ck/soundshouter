<!-- <template>
  <div>
    <header class="sticky top-0 z-20">
      <NavBar @type-Event="filterSounds" />
    </header>

    <main class="dark:bg-gray-800">
      <SoundTable :barState="sideBarState">
        <template #soundelements>
          <SoundElement
            v-for="sound in soundsFiltered"
            :key="sound.id"
            :id="sound.id"
            :name="sound.name"
            :duration="sound.duration"
            :play_count="sound.play_count"
            :category_id="sound.category_id"
            :caterory="sound.category"
            :subcategory_id="sound.subcategory_id"
            @emit-shout-event="shoutNow"
          />
        </template>
      </SoundTable>
    </main>

    <SideBar
      :barState="sideBarState"
      @switch-side-bar-state="changeSideBarState"
    >
      <template #element>
        <SideBarElement
          v-for="category in categories"
          :key="category.id"
          :name="category.name"
          :id="category.id"
          @click-cat-event="filterSubCategories"
        />
      </template>
    </SideBar>

    <footer
      class="p-4 bg-gray-800 shadow dark:bg-gray-800 sticky bottom-0 z-20"
      :class="{ 'ml-64': sideBarState }"
    >
      <Footer :subCatProps="subcategoriesFiltered" />
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount, provide } from 'vue'
import { initFlowbite } from 'flowbite'

import NavBar from './components/NavBar.vue'
import SoundTable from './components/SoundTable.vue'
import SoundElement from './components/SoundElement.vue'
import SideBar from './components/SideBar.vue'
import SideBarElement from './components/SideBarElement.vue'
import Footer from './components/Footer.vue'

import {
  categories,
  filterSounds,
  soundsFiltered,
  subcategoriesFiltered,
  filterSubCategories,
  filterSoundsBySubCatergorie,
  shoutNow
} from './js/data.js'

// Local reactive sidebar state
const sideBarState = ref(true)

function changeSideBarState() {
  sideBarState.value = !sideBarState.value
}

// Auto-collapse on small screens
function handleResize() {
  const isSmall = window.matchMedia('(max-width: 1023px)').matches
  sideBarState.value = !isSmall
}

onMounted(() => {
  initFlowbite()
  filterSounds("")
  handleResize()
  window.addEventListener('resize', handleResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})

// Provide to child components
function activateOnEvent(id, event) {
  filterSoundsBySubCatergorie(id, event)
}

provide('parentFn', activateOnEvent)
</script> -->
<template>
  <div>
    <header class="sticky top-0 z-20">
      <NavBar @type-Event="filterSounds" />
    </header>

    <main class="dark:bg-gray-800">
      <SoundTable :barState="sideBarState">
        <template #soundelements>
          <div v-if="loadingSounds" class="text-center p-8">
            <span class="text-white">Loading sounds...</span>
          </div>
          <SoundElement
            v-else
            v-for="sound in soundsFiltered"
            :key="sound.id"
            :id="sound.id"
            :name="sound.name"
            :duration="sound.duration"
            :play_count="sound.play_count"
            :category_id="sound.category_id"
            :caterory="sound.category"
            :subcategory_id="sound.subcategory_id"
            @emit-shout-event="shoutNow"
          />
        </template>
      </SoundTable>
    </main>

    <SideBar
      :barState="sideBarState"
      @switch-side-bar-state="changeSideBarState"
    >
      <template #element>
        <div v-if="loadingCategories" class="text-center p-4">
          <span class="text-white">Loading categories...</span>
        </div>
        <SideBarElement
          v-else
          v-for="category in categories"
          :key="category.id"
          :name="category.name"
          :id="category.id"
          @click-cat-event="filterSubCategories"
        />
      </template>
    </SideBar>

    <footer
      class="p-4 bg-gray-800 shadow dark:bg-gray-800 sticky bottom-0 z-20"
      :class="{ 'ml-64': sideBarState }"
    >
      <div v-if="loadingSubcategories" class="text-center">
        <span class="text-white">Loading subcategories...</span>
      </div>
      <Footer v-else :subCatProps="subcategoriesFiltered" />
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount, provide } from 'vue'
import { initFlowbite } from 'flowbite'

import NavBar from './components/NavBar.vue'
import SoundTable from './components/SoundTable.vue'
import SoundElement from './components/SoundElement.vue'
import SideBar from './components/SideBar.vue'
import SideBarElement from './components/SideBarElement.vue'
import Footer from './components/Footer.vue'

import {
  categories,
  filterSounds,
  soundsFiltered,
  subcategoriesFiltered,
  filterSubCategories,
  filterSoundsBySubCatergorie,
  shoutNow,
  getSounds,
  getCategories,
  getSubcategories
} from './js/data.js'

// Local reactive sidebar state
const sideBarState = ref(true)

// Loading states
const loadingSounds = ref(true)
const loadingCategories = ref(true)
const loadingSubcategories = ref(true)

function changeSideBarState() {
  sideBarState.value = !sideBarState.value
}

// Auto-collapse on small screens
function handleResize() {
  const isSmall = window.matchMedia('(max-width: 1023px)').matches
  sideBarState.value = !isSmall
}

onMounted(() => {
  initFlowbite()

  // Load all data
  getSounds().finally(() => {
    loadingSounds.value = false
  })
  getCategories().finally(() => {
    loadingCategories.value = false
  })
  getSubcategories().finally(() => {
    loadingSubcategories.value = false
  })
  
  filterSounds("")
  handleResize()
  window.addEventListener('resize', handleResize)

  
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})

// Provide to child components
function activateOnEvent(id, event) {
  filterSoundsBySubCatergorie(id, event)
}

provide('parentFn', activateOnEvent)
</script>

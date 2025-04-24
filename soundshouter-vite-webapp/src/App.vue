<template>
  <header class="sticky top-0 z-20">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <NavBar @type-Event="filterSounds" />
  </header>

  <body class="dark:bg-gray-800">
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
  </body>

  <SideBar :barState="sideBarState" @switch-side-bar-state="changeSideBatState">
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
    :class="sideBarState === true ? 'ml-64' : ''"
  >
    <Footer :subCatProps="subcategoriesFiltered" />
  </footer>
</template>

<script setup>
import { onMounted, provide } from 'vue'
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
  sideBarState,
  changeSideBatState
} from './js/data.js'

onMounted(() => {
  initFlowbite()
  filterSounds("") // load all sounds initially
})

function activateOnEvent(id, event) {
  filterSoundsBySubCatergorie(id, event)
}

provide('parentFn', activateOnEvent)
</script>

<template>

    <header class="sticky top-0 z-20">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <div>
            <NavBar @type-Event="filterSounds" />
        </div>
    </header>

    <body class="dark:bg-gray-800">

        <div>
            <SoundTable :barState="sideBarState">
                <template #soundelements>

                    <SoundElement v-for="sound in soundsFiltered" :id="sound.id" :name="sound.name"
                        :duration="sound.duration" :play_count="sound.play_count" :category_id="sound.category_id"
                        :caterory="sound.category" :subcategory_id="sound.subcategory_id" @emit-shout-event="shoutNow">

                    </SoundElement>
                </template>
            </SoundTable>
        </div>
    </body>

    <div>
        <SideBar :barState="sideBarState" @switch-side-bar-state="changeSideBatState">
            <template #element>
                <div>
                    <SideBarElement v-for="category in categories" :name="category.name" :id="category.id"
                        @click-cat-event="filterSubCategories">
                    </SideBarElement>
                </div>
            </template>

        </SideBar>
    </div>

    <footer class="p-4 bg-gray-800 shadow dark:bg-gray-800 sticky bottom-0 z-20"
    :class="sideBarState === true ? 'ml-64' : ''">
        <div>
            <Footer :subCatProps="subcategoriesFiltered"/>
        </div>
    </footer>


</template>


<script setup>
import SoundTable from "./components/SoundTable.vue"
import NavBar from "./components/NavBar.vue"
import Footer from "./components/Footer.vue"
import SideBar from "./components/SideBar.vue"
import SideBarElement from "./components/SideBarElement.vue"
import { categories, filterSounds, soundsFiltered, subcategoriesFiltered, filterSubCategories, filterSoundsBySubCatergorie, shoutNow, sideBarState, changeSideBatState, getSounds, getCategories, getSubcategories } from "./js/data.js"
import SoundElement from './components/SoundElement.vue';

import { onMounted } from 'vue'
import { provide } from 'vue';
import { initFlowbite } from 'flowbite'

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

// Used Provide Inject feature beacuse emit is in parent->child-Suchild
// https://vuejs.org/guide/components/provide-inject.html
// https://stackoverflow.com/questions/75812064/vue-3-how-to-forward-all-emitted-events-to-a-parent-component
function activateOnEvent(id, event) {
    filterSoundsBySubCatergorie(id, event)
}
provide('parentFn', activateOnEvent);
// Call Filter once to get all sounds when site is loaded
getSounds()
getCategories()
getSubcategories()
//filterSounds("")

// use emit to change data from child instead of props
// https://stackoverflow.com/questions/40915436/vuejs-update-parent-data-from-child-component


</script>

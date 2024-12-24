<template>

    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
    </head>

    <header class="sticky top-0 z-20">
        <div>
            <NavBar @type-Event="filterSounds" />
        </div>
    </header>

    <body class="dark:bg-gray-800">
    
        <div class="sticky top-36 z-20 bg-white dark:bg-gray-950 h-auto min-h-12"
            :class="sideBarState === true ? 'ml-64' : ''">
            <SubCatElement v-for="sub in subcategoriesFiltered" :id="sub.id" :subcat="sub.name"
                @click-sub-cat-event="filterSoundsBySubCatergorie">
            </SubCatElement>
        </div>

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
            <Footer />
        </div>
    </footer>


</template>


<script setup>
import SoundTable from "./components/SoundTable.vue"
import NavBar from "./components/NavBar.vue"
import Footer from "./components/Footer.vue"
import SideBar from "./components/SideBar.vue"
import SideBarElement from "./components/SideBarElement.vue"
import { categories, filterSounds, soundsFiltered, subcategoriesFiltered, filterSubCategories, filterSoundsBySubCatergorie, shoutNow, sideBarState, changeSideBatState } from "./js/data.js"
import SubCatElement from "./components/SubCatElement.vue"
import SoundElement from './components/SoundElement.vue';

import { reactive } from 'vue'
import { onMounted } from 'vue'
import { initFlowbite } from 'flowbite'
import axios, { isCancel, AxiosError } from 'axios';

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

// Call Filter once to get all sounds when site is loaded
filterSounds("")

// use emit to change data from child instead of props
// https://stackoverflow.com/questions/40915436/vuejs-update-parent-data-from-child-component


// Make a request for a user with a given ID
//axios.get('http://127.0.0.1:8000/api/v1/sounds')
//  .then(function (response) {
// handle success
//    console.log(response);
// })
// .catch(function (error) {
// handle error
//    console.log(error);
//  })
//  .finally(function () {
// always executed
//  });

</script>

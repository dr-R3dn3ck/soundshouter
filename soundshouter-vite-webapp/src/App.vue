<template>

    <header class="sticky top-0 z-20">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <NavBar @type-Event="filterSounds"/>
    </header>

    <body class="dark:bg-gray-800">
        <SideBar>
            <template #element>
                <SideBarElement v-for="category in categories" :name="category.name">
                </SideBarElement>
            </template>
            <template #main>
                <SoundTable :soundsprops="soundsFiltered" :subcatprops="subcategories">
                </SoundTable>
            </template>

        </SideBar>
    </body>

    <footer class="p-4 sm:ml-64 bg-gray-800 shadow dark:bg-gray-800 sticky bottom-0 z-20">
        <Footer />
    </footer>


</template>


<script setup>
import SoundTable from "./components/SoundTable.vue"
import NavBar from "./components/NavBar.vue"
import Footer from "./components/Footer.vue"
import SoundElement from "./components/SoundElement.vue"
import SideBar from "./components/SideBar.vue"
import SideBarElement from "./components/SideBarElement.vue"
import SubCatElement from "./components/SubCatElement.vue"
import { sounds, categories, subcategories, filterSounds, soundsFiltered, categoriesFiltered, subcategoriesFiltered} from "./js/data.js"

import { reactive } from 'vue'
import { onMounted } from 'vue'
import { initFlowbite } from 'flowbite'
import axios, { isCancel, AxiosError } from 'axios';

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

// use emit to change data from child instead of props
// https://stackoverflow.com/questions/40915436/vuejs-update-parent-data-from-child-component
//filterSounds(sounds, "appl")

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

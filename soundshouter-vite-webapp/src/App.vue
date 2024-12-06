
<template>

  <header class="sticky top-0 z-20">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <NavBar />
  </header>

  <body class="px-20 min-h-screen max-h-screen width dark:bg-gray-800">
    <SoundTable>
      <SoundElement v-for="sound in sounds" 
      :id="sound.id" 
      :name="sound.name"
      :duration="sound.duration" 
      :play_count="sound.play_count"
      :category_id="sound.category_id" 
      :caterory="sound.category"
      :subcategory_id="sound.subcategory_id" >
        
      </SoundElement>
    </SoundTable>
  </body>

  <footer class="bg-gray-800 shadow dark:bg-gray-800 sticky bottom-0 z-20">
    <Footer />
  </footer>

</template>


<script setup>
import SoundTable from "./components/SoundTable.vue"
import NavBar from "./components/NavBar.vue"
import Footer from "./components/Footer.vue"
import SoundElement from "./components/SoundElement.vue"

import { reactive } from 'vue'
import { onMounted } from 'vue'
import { initFlowbite } from 'flowbite'
import axios, {isCancel, AxiosError} from 'axios';

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

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

const sounds = reactive([
    {
        "id": 1,
        "name": "applause-cheer-236786",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 2,
        "name": "relaxing-guitar-loop-v5-245859",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 3,
        "name": "astral-creepy-dark-logo-254198",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 4,
        "name": "dark-future-logo-196217",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 5,
        "name": "applause-cheer-236786",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 6,
        "name": "relaxing-guitar-loop-v5-245859",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 7,
        "name": "astral-creepy-dark-logo-254198",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 8,
        "name": "dark-future-logo-196217",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 9,
        "name": "stab-f-01-brvhrtz-224599",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 10,
        "name": "applause-cheer-236786",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 2
    }
])

const categories = reactive([
    {
        "id": 1,
        "name": "terstcategory"
    },
    {
        "id": 2,
        "name": "testcategoroie2"
    },
    {
        "id": 3,
        "name": "testcategorie1"
    }
])

const subcategories = reactive([
    {
        "id": 1,
        "name": "Subcategory2",
        "category_id": 1
    },
    {
        "id": 2,
        "name": "Subcategory1",
        "category_id": 1
    },
    {
        "id": 3,
        "name": "subcatergory3",
        "category_id": 3
    }
])
</script>

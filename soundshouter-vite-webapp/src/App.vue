
<template>

  <header class="sticky top-0 z-20">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <NavBar />
  </header>

  <body class="min-h-screen max-h-screen dark:bg-gray-800">
    <SoundTable>
      <SoundElement v-for="sound in sndObject" 
      :title="sound.title" 
      :caterory="sound.category">
        
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

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

const sndObject = reactive(
  [{
    id: 0,
    title: "Sound1",
    category: "games"
  },
  {
    id: 1,
    title: "Sound2",
    category: "games"
  }
]
)
</script>

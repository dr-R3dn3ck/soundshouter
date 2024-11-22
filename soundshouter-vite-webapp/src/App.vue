<script setup>
import SoundTable from "./components/SoundTable.vue"
import NavBar from "./components/NavBar.vue"
import Footer from "./components/Footer.vue"
</script>

<template >

    <header>
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <NavBar />
    </header>

    <body>
      <div class="container mx-auto">
        <SoundTable />
      </div>
    </body>

    <footer>
      <Footer />
    </footer>
    

</template>

<script setup>
import SoundTable from "./components/SoundTable.vue"
import NavBar from "./components/NavBar.vue"
import Footer from "./components/Footer.vue"
</script>

<template>

  <header class="sticky top-0 z-20">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <NavBar />
  </header>

  <body class="min-h-screen max-h-screen dark:bg-gray-800">
    <SoundTable />
  </body>


  <footer class="bg-gray-800 shadow dark:bg-gray-800 sticky bottom-0 z-20">
    <Footer />
  </footer>


</template>

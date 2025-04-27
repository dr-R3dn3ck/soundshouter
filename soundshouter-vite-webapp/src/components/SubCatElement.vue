<script setup>
import { onMounted, defineEmits } from 'vue'
import { initFlowbite } from 'flowbite'
import { inject } from 'vue'; 

const parentFn = inject('parentFn');


// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

defineProps({
  id: Number,
  subcat: String,
})
</script>

<!-- <template>
    <button type="button" @click="$emit('clickSubCatEvent', id, $event)" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm italic px-2 py-1 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">{{ subcat }}</button>
</template> -->
<template>
  <li type="li" @click="parentFn(id, $event)" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm italic px-2 py-1 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">{{ subcat }}</li>
</template>
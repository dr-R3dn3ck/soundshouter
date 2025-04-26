<script setup>
import { onMounted, defineProps, defineEmits } from 'vue'
import { initFlowbite } from 'flowbite'
import SubCatElement from './SubCatElement.vue';

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})
defineProps({
    subCatProps: Object
})


function redirectEmit(id, event){
    $emit('clickSubCatEventSub', id, event)
}
</script>

<template>

    <div class="w-full mx-auto max-w-screen-xl p-8 md:flex md:items-center md:justify-between">
        <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400"><a
                class="hover:underline">Soundshouter WebUI</a>. Crafted for LANs <br> by Max
        </span>
        <ul class="flex flex-wrap items-center mt-3 text-sm font-medium text-gray-500 dark:text-gray-400 sm:mt-0">
            <SubCatElement v-for="sub in subCatProps" :id="sub.id" :subcat="sub.name">
            </SubCatElement>
        </ul>
    </div>
</template>

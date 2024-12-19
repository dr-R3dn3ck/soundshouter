<script setup>

import { onMounted } from 'vue'
import { initFlowbite } from 'flowbite'
import SoundElement from './SoundElement.vue';
import SubCatElement from "./SubCatElement.vue"

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

defineProps({
    soundsprops: Object,
    subcatprops: Object
})
</script>

<template>
    <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
        <div class="pb-5 bg-white dark:bg-gray-900 h-auto sticky top-0">

            <SubCatElement v-for="sub in subcatprops" :subcat="sub.name">

            </SubCatElement>
        </div>
        <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                <tr>
                    <th scope="col" class="p-4">
                        <div class="flex items-center">
                            <input id="checkbox-all-search" type="checkbox"
                                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600">
                            <label for="checkbox-all-search" class="sr-only">checkbox</label>
                        </div>
                    </th>
                    <th scope="col" class="px-6 py-3">
                        Sound name
                    </th>
                    <th scope="col" class="px-6 py-3">
                        Category
                    </th>
                    <th scope="col" class="px-6 py-3">
                        Duration
                    </th>
                    <th scope="col" class="px-6 py-3">
                        Play Count
                    </th>
                    <th scope="col" class="px-6 py-3">
                        Action
                    </th>
                </tr>
            </thead>

            <tbody class="overflow-y-auto">
                <SoundElement v-for="sound in soundsprops" :id="sound.id" :name="sound.name" :duration="sound.duration"
                    :play_count="sound.play_count" :category_id="sound.category_id" :caterory="sound.category"
                    :subcategory_id="sound.subcategory_id">

                </SoundElement>
            </tbody>
        </table>
    </div>

</template>
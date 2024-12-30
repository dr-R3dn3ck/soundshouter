<script setup>

import { onMounted, defineProps } from 'vue'
import { initFlowbite } from 'flowbite'

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})
defineProps({
    barState: Boolean
})

</script>

<template class="">
        <div class="max-lg:overflow-auto min-h-screen shadow-md sm:rounded-lg"
        :class="barState === true ? 'ml-64' : ''">
            <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                <thead class="max-lg:hidden fixed w-full top-48 z-20 text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <!-- <th scope="col" class="sticky top-30 z-20 p-4"> 
                        </th> -->
                        <th scope="col" class="w-96 px-6 py-3">
                            Sound name
                        </th>
                        <th scope="col" class="w-60 px-6 py-3">
                            Category
                        </th>
                        <th scope="col" class="w-32 sticky top-30 z-20 px-6 py-3">
                            Duration
                        </th>
                        <th scope="col" class="w-32 px-6 py-3">
                            Play Count
                        </th>
                        <th scope="col" class="w-32 px-6 py-3">
                            Action
                        </th>
                    </tr>
                </thead>

                <tbody>
                    <slot name="soundelements"></slot>
                </tbody>
            </table>
        </div>
</template>
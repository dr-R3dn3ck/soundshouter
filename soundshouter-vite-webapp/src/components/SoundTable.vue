<script setup>

import { onMounted } from 'vue'
import { initFlowbite } from 'flowbite'

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

</script>

<template class="">
        <div class="min-h-screen shadow-md sm:rounded-lg sm:ml-64">
            <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                <thead class="sticky top-48 z-20 text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="sticky top-30 z-20 p-4"> 
                        </th>
                        <th scope="col" class="sticky top-30 z-20 px-6 py-3">
                            Sound name
                        </th>
                        <th scope="col" class="sticky top-30 z-20 px-6 py-3">
                            Category
                        </th>
                        <th scope="col" class="sticky top-30 z-20 px-6 py-3">
                            Duration
                        </th>
                        <th scope="col" class="sticky top-30 z-20 px-6 py-3">
                            Play Count
                        </th>
                        <th scope="col" class="sticky top-30 z-20 px-6 py-3">
                            Action
                        </th>
                    </tr>
                </thead>

                <tbody class="overscroll-y-auto">
                    <slot name="soundelements"></slot>

                </tbody>
            </table>
        </div>
</template>
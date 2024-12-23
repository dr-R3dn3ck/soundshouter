<script setup>

import { onMounted } from 'vue'
import { initFlowbite } from 'flowbite'

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

</script>

<template>
        <div class="overflow-clip overflow-x-auto min-h-screen shadow-md sm:rounded-lg sm:ml-64">
            <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                <thead
                    class=" sticky top-36 text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="p-4">
                            
                        </th>
                        <th scope="col" class="sticky top-20 z-20 px-6 py-3">
                            Sound name
                        </th>
                        <th scope="col" class="sticky top-40 px-6 py-3">
                            Category
                        </th>
                        <th scope="col" class="sticky top-36 px-6 py-3">
                            Duration
                        </th>
                        <th scope="col" class="sticky top-36 px-6 py-3">
                            Play Count
                        </th>
                        <th scope="col" class="sticky top-36 px-6 py-3">
                            Action
                        </th>
                    </tr>
                </thead>

                <tbody class="overflow-auto">
                    <slot name="soundelements"></slot>

                </tbody>
            </table>
        </div>
</template>
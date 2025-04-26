<script setup>
import { onMounted, defineProps, defineEmits } from 'vue'
import { initFlowbite } from 'flowbite'

// Initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

defineProps({
    barState: Boolean
})

defineEmits(["emitShoutEvent"])
</script>

<template>
    <div class="min-h-screen shadow-md sm:rounded-lg" :class="barState === true ? 'ml-64' : ''">
        <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
            <!-- Sticky Header -->
            <thead class="w-full text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400 
                sticky top-[9rem] z-20 max-lg:top-[9rem]">
                <tr>
                    <th scope="col" class="w-96 px-6 py-3">
                        Sound name
                    </th>
                    <th scope="col" class="w-60 px-6 py-3 max-lg:hidden">
                        Category
                    </th>
                    <th scope="col" class="w-60 sticky top-30 z-20 px-6 py-3 max-lg:hidden">
                        Duration
                    </th>
                    <th scope="col" class="w-60 px-6 py-3 max-lg:hidden">
                        Play Count
                    </th>
                    <th scope="col" class="w-80 px-6 py-3">
                        Action
                    </th>
                </tr>
            </thead>

            <tbody class="max-lg:overflow-auto">
                <slot name="soundelements"></slot>
            </tbody>
        </table>
    </div>
</template>

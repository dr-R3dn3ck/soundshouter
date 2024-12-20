<script setup>
import { onMounted, defineProps, defineEmits } from 'vue'
import { initFlowbite } from 'flowbite'

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})

defineProps({
    id: Number,
    name: String,
    duration: Number,
    play_count: Number,
    category_id: Number,
    caterory: Number,
    subcategory_id: Number
})
defineEmits(["emitShoutEvent"])
</script>

<template>
    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
        <td class="w-4 p-4">
            <div class="flex items-center">
                <input id="checkbox-table-search-2" type="checkbox"
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600">
                <label for="checkbox-table-search-2" class="sr-only">checkbox</label>
            </div>
        </td>
        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
            {{ name }}
        </th>
        <td class="px-6 py-4">
            {{ category_id }}
        </td>
        <td class="px-6 py-4">
            {{ duration }}
        </td>
        <td class="px-6 py-4">
            {{ play_count }}
        </td>
        <td>
            <button class="flex flex-row flex-auto place-items-center px-6 py-4 hover:bg-blue-600 hover:rounded-xl">
                <b class="font-medium text-blue-600 dark:text-blue-500">SHOUT</b>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
                    <path fill="currentColor"
                        d="m12 12l7.07 7.07a9.953 9.953 0 0 1-14.14 0C1 15.17 1 8.84 4.93 4.93C8.83 1 15.16 1 19.07 4.93zm7-2a2 2 0 0 0-2 2a2 2 0 0 0 2 2a2 2 0 0 0 2-2a2 2 0 0 0-2-2" />
                </svg>
            </button>
        </td>
    </tr>

</template>
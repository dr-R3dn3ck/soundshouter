<script setup>
import { defineProps, defineEmits } from 'vue'

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
    <tr
        class="overflow-x-auto bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
        <th scope="row"
            class="w-full sm:w-96 px-6 py-4 font-medium text-gray-900 whitespace-normal break-words dark:text-white">
            {{ name }}
        </th>

        <td class="px-6 py-4 max-lg:hidden">
            {{ category_id }}
        </td>
        <td class="px-6 py-4 max-lg:hidden">
            {{ duration }}
        </td>
        <td class="px-6 py-4 max-lg:hidden">
            {{ play_count }}
        </td>
        <td>
            <button @click="$emit('emitShoutEvent', id, $event)"
                class="flex flex-row flex-auto place-items-center px-6 py-4 hover:bg-blue-600 hover:rounded-xl">
                <b class="font-medium text-blue-600 dark:text-blue-500">SHOUT</b>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
                    <path fill="currentColor"
                        d="m12 12l7.07 7.07a9.953 9.953 0 0 1-14.14 0C1 15.17 1 8.84 4.93 4.93C8.83 1 15.16 1 19.07 4.93zm7-2a2 2 0 0 0-2 2a2 2 0 0 0 2 2a2 2 0 0 0 2-2a2 2 0 0 0-2-2" />
                </svg>
            </button>
        </td>
    </tr>
</template>

// import { ref, reactive } from "vue"

//     export const sounds = reactive([
//         {
//             "id": 1,
//             "name": "applause-cheer-236786",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 1,
//             "subcategory_id": null
//         },
//         {
//             "id": 2,
//             "name": "relaxing-guitar-loop-v5-245859",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 1,
//             "subcategory_id": null
//         },
//         {
//             "id": 3,
//             "name": "astral-creepy-dark-logo-254198",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 1,
//             "subcategory_id": null
//         },
//         {
//             "id": 4,
//             "name": "dark-future-logo-196217",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 1,
//             "subcategory_id": null
//         },
//         {
//             "id": 5,
//             "name": "applause-cheer-236786",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 1,
//             "subcategory_id": 1
//         },
//         {
//             "id": 6,
//             "name": "relaxing-guitar-loop-v5-245859",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 1,
//             "subcategory_id": 1
//         },
//         {
//             "id": 7,
//             "name": "astral-creepy-dark-logo-254198",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 1,
//             "subcategory_id": 1
//         },
//         {
//             "id": 8,
//             "name": "dark-future-logo-196217",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 3,
//             "subcategory_id": 1
//         },
//         {
//             "id": 9,
//             "name": "stab-f-01-brvhrtz-224599",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 3,
//             "subcategory_id": 1
//         },
//         {
//             "id": 10,
//             "name": "applause-cheer-236786",
//             "duration": 0.0,
//             "play_count": 0,
//             "category_id": 2,
//             "subcategory_id": 2
//         }
//     ])

//     export const categories = reactive([
//         {
//             "id": 1,
//             "name": "terstcategory"
//         },
//         {
//             "id": 2,
//             "name": "testcategoroie2"
//         },
//         {
//             "id": 3,
//             "name": "testcategorie1"
//         }
//     ])

//     export const subcategories = reactive([
//         {
//             "id": 1,
//             "name": "Subcategory2",
//             "category_id": 1
//         },
//         {
//             "id": 2,
//             "name": "Subcategory1",
//             "category_id": 1
//         },
//         {
//             "id": 3,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 4,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 5,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 6,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 7,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 8,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 9,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 10,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 11,
//             "name": "subcatergory3",
//             "category_id": 3
//         },
//         {
//             "id": 12,
//             "name": "subcatergory3",
//             "category_id": 3
//         }
//     ])

// // export const sounds = ref([])
// // export const subcategories = ref([])
// // export const categories = ref([])

// import axios from 'axios';
// // Axios
// //const axios = require('axios');
// export function getSounds() {
//     // Make a request for a user with a given ID
//     axios.get('/api/v1/sounds?limit=1000')
//         .then(function (response) {
//             // handle success
//             console.log("response data")
//             console.log(response)
//             console.log(response.data)
//             //sounds.value = response.data
//             for (var i = 0; i < response.data.length; i++) {
//                 sounds.value.push(response.data[i])
//             }
//             //sounds.value = response.data
//             console.log("first sounds.value then sounds1")
//             console.log(sounds.value)
//             filterSounds("", sounds)
//         })
//         .catch(function (error) {
//             // handle error
//             console.log(error);
//         })
//         .finally(function () {
//             // always executed
//         });
// }

// export function getCategories() {
//     // Make a request forv1 a user with a given ID
//     axios.get('/api/v1/categories')
//         .then(function (response) {
//             // handle success
//             categories.value = response.data
//         })
//         .catch(function (error) {
//             // handle error
//             console.log(error);
//         })
//         .finally(function () {
//             // always executed
//         });
// }

// export function getSubcategories() {
//     // Make a request for a user with a given ID
//     axios.get('/api/v1/subcategories')
//         .then(function (response) {
//             // handle success
//             subcategories.value = response.data
//         })
//         .catch(function (error) {
//             // handle error
//             console.log(error);
//         })
//         .finally(function () {
//             // always executed
//         });
// }


// export const soundsFiltered = ref([])
// export const categoriesFiltered = ref([])
// export const subcategoriesFiltered = ref([])
// export const sideBarState = ref(true)

// export function changeSideBatState() {
//     if (sideBarState.value === true) {
//         sideBarState.value = false
//     }
//     else {
//         sideBarState.value = true
//     }
//     console.log(sideBarState.value)
// }

// export function filterSounds(filter) {
//     soundsFiltered.value.length = 0
//     console.log("length")
//     console.log(sounds.value.length)
//     console.log(sounds.value)
//     for (var i = 0; i < sounds.value.length; i++) {
//         let name = sounds.value[i].name
//         console.log("name of sound checked")
//             console.log(sounds.value[i].name)
//         if (name.includes(filter)) {
//             console.log("sounds matching name")
//             console.log(sounds.value[i])
//             soundsFiltered.value.push(sounds.value[i])
//         }
//     }
//     console.log("HUHU")
//     console.log(soundsFiltered)
// }

// export function filterSubCategories(id, e) {
//     subcategoriesFiltered.value.length = 0
//     for (var i = 0; i < subcategories.value.length; i++) {
//         if (id === subcategories.value[i].category_id) {
//             subcategoriesFiltered.value.push(subcategories.value[i])
//         }
//     }

//     soundsFiltered.value.length = 0
//     for (var i = 0; i < sounds.value.length; i++) {
//         if (id === sounds.value[i].category_id) {
//             soundsFiltered.value.push(sounds.value[i])
//         }
//     }
// }

// export function filterSoundsBySubCatergorie(id, e) {
//     soundsFiltered.value.length = 0
//     for (var i = 0; i < sounds.value.length; i++) {
//         if (id === sounds.value[i].subcategory_id) {
//             soundsFiltered.value.push(sounds.value[i])
//         }
//     }
// }

// export function shoutNow(id, e) {
//     console.log(id)
//     axios.post('/api/v1/play/'+id)
//       .then(function (response) {
//         console.log(response);
//       })
//       .catch(function (error) {
//         console.log(error);
//       });
// }











// src/js/data.js

import { ref } from 'vue'

// Dummy data with `ref` instead of `reactive`
export const sounds = ref([
    { id: 1, name: "applause-cheer-236786", duration: 0.0, play_count: 0, category_id: 1, subcategory_id: null },
    { id: 2, name: "relaxing-guitar-loop-v5-245859", duration: 0.0, play_count: 0, category_id: 1, subcategory_id: null },
    { id: 3, name: "astral-creepy-dark-logo-254198", duration: 0.0, play_count: 0, category_id: 1, subcategory_id: null },
    { id: 4, name: "dark-future-logo-196217", duration: 0.0, play_count: 0, category_id: 1, subcategory_id: null },
    { id: 5, name: "applause-cheer-236786", duration: 0.0, play_count: 0, category_id: 1, subcategory_id: 1 },
    { id: 6, name: "relaxing-guitar-loop-v5-245859", duration: 0.0, play_count: 0, category_id: 1, subcategory_id: 1 },
    { id: 7, name: "astral-creepy-dark-logo-254198", duration: 0.0, play_count: 0, category_id: 1, subcategory_id: 1 },
    { id: 8, name: "dark-future-logo-196217", duration: 0.0, play_count: 0, category_id: 3, subcategory_id: 1 },
    { id: 9, name: "stab-f-01-brvhrtz-224599", duration: 0.0, play_count: 0, category_id: 3, subcategory_id: 1 },
    { id: 10, name: "applause-cheer-236786", duration: 0.0, play_count: 0, category_id: 2, subcategory_id: 2 }
])

export const categories = ref([
    { id: 1, name: "terstcategory" },
    { id: 2, name: "testcategoroie2" },
    { id: 3, name: "testcategorie1" }
])

export const subcategories = ref([
    { id: 1, name: "Subcategory2", category_id: 1 },
    { id: 2, name: "Subcategory1", category_id: 1 },
    { id: 3, name: "subcatergory3", category_id: 3 },
    { id: 4, name: "subcatergory3", category_id: 3 },
    { id: 5, name: "subcatergory3", category_id: 3 },
    { id: 6, name: "subcatergory3", category_id: 3 },
    { id: 7, name: "subcatergory3", category_id: 3 },
    { id: 8, name: "subcatergory3", category_id: 3 },
    { id: 9, name: "subcatergory3", category_id: 3 },
    { id: 10, name: "subcatergory3", category_id: 3 },
    { id: 11, name: "subcatergory3", category_id: 3 },
    { id: 12, name: "subcatergory3", category_id: 3 }
])

export const soundsFiltered = ref([])
export const categoriesFiltered = ref([])
export const subcategoriesFiltered = ref([])
export const sideBarState = ref(true)

export function changeSideBatState() {
    sideBarState.value = !sideBarState.value
    console.log(sideBarState.value)
}

export function filterSounds(filter) {
    soundsFiltered.value.length = 0
    for (let i = 0; i < sounds.value.length; i++) {
        let name = sounds.value[i].name
        if (name.includes(filter)) {
            soundsFiltered.value.push(sounds.value[i])
        }
    }
}

export function filterSubCategories(id, e) {
    subcategoriesFiltered.value.length = 0
    for (let i = 0; i < subcategories.value.length; i++) {
        if (id === subcategories.value[i].category_id) {
            subcategoriesFiltered.value.push(subcategories.value[i])
        }
    }

    soundsFiltered.value.length = 0
    for (let i = 0; i < sounds.value.length; i++) {
        if (id === sounds.value[i].category_id) {
            soundsFiltered.value.push(sounds.value[i])
        }
    }
}

export function filterSoundsBySubCatergorie(id, e) {
    soundsFiltered.value.length = 0
    for (let i = 0; i < sounds.value.length; i++) {
        if (id === sounds.value[i].subcategory_id) {
            soundsFiltered.value.push(sounds.value[i])
        }
    }
}

export function shoutNow(id, e) {
    console.log("Shouting sound with ID:", id)
    // Fake action — replace with real request if needed
}

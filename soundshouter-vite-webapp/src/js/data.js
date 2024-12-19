import { ref, reactive } from "vue"

// NOT USED FROM
const categoriesobj = reactive({
    id:Number,
    name:String
})

const soundobj = reactive({
    id:Number,
    name:String,
    duration:Number,
    playcounter:Number,
    categoryid:Number,
    subcategoryid:categoriesobj
})

const subcategoriesobj = reactive({
    id:Number,
    name:String,
    categoryid:categoriesobj
})

export const dataobj = reactive({
    sounds:[soundobj],
    categories:[categoriesobj],
    subcategories:[subcategoriesobj]
})

export const parseSounds = (sounds) =>{
    for (var i=0;i<sounds.length; i++){
        console.log(sounds[i].id)
        soundobj = (id=sounds[i].id)
        //soundobj = (id=ele.id)
        //dataobj.sounds.push('baz')
        //dataobj.sounds.values.push()
    }
}
// NOT USED TILL

export const soundsFiltered = reactive([])
export const categoriesFiltered = reactive([])
export const subcategoriesFiltered = reactive([])

export function filterSounds(filter){
    //reset list first
    //for (var j=0;j<soundsFiltered.length; j++){
     //   soundsFiltered.pop()
    //}
    soundsFiltered.length=0
    for (var i=0;i<sounds.length; i++){
        let name = sounds[i].name
        if (name.includes(filter)){
            soundsFiltered.push(sounds[i])
        }
    }
    console.log(soundsFiltered)
}

export const sounds = reactive([
    {
        "id": 1,
        "name": "applause-cheer-236786",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 2,
        "name": "relaxing-guitar-loop-v5-245859",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 3,
        "name": "astral-creepy-dark-logo-254198",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 4,
        "name": "dark-future-logo-196217",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": null
    },
    {
        "id": 5,
        "name": "applause-cheer-236786",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 6,
        "name": "relaxing-guitar-loop-v5-245859",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 7,
        "name": "astral-creepy-dark-logo-254198",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 8,
        "name": "dark-future-logo-196217",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 9,
        "name": "stab-f-01-brvhrtz-224599",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 1
    },
    {
        "id": 10,
        "name": "applause-cheer-236786",
        "duration": 0.0,
        "play_count": 0,
        "category_id": 1,
        "subcategory_id": 2
    }
])

export const categories = reactive([
    {
        "id": 1,
        "name": "terstcategory"
    },
    {
        "id": 2,
        "name": "testcategoroie2"
    },
    {
        "id": 3,
        "name": "testcategorie1"
    }
])

export const subcategories = reactive([
    {
        "id": 1,
        "name": "Subcategory2",
        "category_id": 1
    },
    {
        "id": 2,
        "name": "Subcategory1",
        "category_id": 1
    },
    {
        "id": 3,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 4,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 5,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 6,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 7,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 8,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 9,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 10,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 11,
        "name": "subcatergory3",
        "category_id": 3
    },
    {
        "id": 12,
        "name": "subcatergory3",
        "category_id": 3
    }
])
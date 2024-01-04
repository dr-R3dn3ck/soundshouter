use std::collections::HashSet;
use leptos_query::*;
use leptos::*;
use leptos::logging::{log};

use crate::api::{use_sound_query, play_sound};
use crate::api::models::Sound;

#[component]
fn SoundButton(sound: Sound) -> impl IntoView {
    let play_sound = create_action(move |_id: &i32| {
        async move { play_sound(sound.id).await; }
    });

    view! {
        <button
            class="snd-button"
            on:click=move |_| {
                log!("playing sound {}", sound.id);
                play_sound.dispatch(sound.id)
            }>
            <div>{format!("{}", sound.pretty_name())}</div>
        </button>
    }
}

#[component]
pub fn SoundGrid(
        cat_filter: ReadSignal<HashSet::<i32>>,
        subcat_filter: ReadSignal<HashSet::<i32>>,
        query: ReadSignal<String>,
    ) -> impl IntoView {

    let QueryResult {
        data,
        state: _,
        is_loading: _,
        is_fetching: _,
        is_stale: _,
        is_invalid: _,
        refetch: _,
    } = use_sound_query(move || -1);

    view! {
        <div class="sound-grid">
            {
                move || data.get().unwrap_or(Some(vec![])).unwrap().into_iter()
                    .filter(|n| {!cat_filter.get().contains(&n.category_id)})
                    // negative values aren't used as ids (currently by convention)
                    .filter(|n| {!subcat_filter.get().contains(&n.subcategory_id.unwrap_or(-1))})
                    .filter(|n| {query.get().len() == 0 || n.match_query(query.get())})
                    .map(|n| {
                        view! { <SoundButton sound=n /> }
                    })
                    .collect::<Vec<_>>()
            }

        </div>
    }
}

#[component]
pub fn SoundList() -> impl IntoView {

    let QueryResult {
        data,
        state: _,
        is_loading,
        is_fetching: _,
        is_stale: _,
        is_invalid: _,
        refetch: _,
    } = use_sound_query(move || -1);

    view! {
        <div>
            <span>"Loading Status: "</span>
            <span>{move || { if is_loading.get() { "Loading..." } else { "Loaded" } }}</span>
        </div>

        <table>
            <tr>
                <th></th>
                // <th></th>
            </tr>
            {
                move || data.get().unwrap_or(Some(vec![])).unwrap().into_iter()
                    .map(|n| view! {
                        <tr>
                            <td><SoundButton sound=n /></td>
                            // <td>{format!("{}", n.name)}</td>
                        </tr>
                    })
                    .collect::<Vec<_>>()
            }

        </table>
    }
}
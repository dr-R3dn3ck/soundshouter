use std::collections::HashSet;
use leptos::*;
use crate::api::{use_category_query, use_subcategory_query};

#[component]
pub fn SoundShouterNav(
    cat_filter: ReadSignal<HashSet::<i32>>,
    set_cat_filter: WriteSignal<HashSet::<i32>>,
    subcat_filter: ReadSignal<HashSet::<i32>>,
    set_subcat_filter: WriteSignal<HashSet::<i32>>,
    set_query: WriteSignal<String>,
) -> impl IntoView {
    let cat = use_category_query(move || -1);
    let sub = use_subcategory_query(move || -1);

    let update_filter = |filter: ReadSignal<HashSet::<i32>>, set_filter: WriteSignal<HashSet::<i32>>, ev, id| {
        let mut filter = filter.get();
        if event_target_checked(&ev) {
            filter.remove(&id);
        }
        else {
            filter.insert(id);
        }
        set_filter(filter);
    };

    view! {
        <nav id="main-menu">
            <div class="dropdown">
                <button class="dropbtn">Kategorien</button>
                <div class="dropdown-content">
                {
                    move || cat.data.get().unwrap_or(Some(vec![])).unwrap().into_iter()
                        .map(|n| view! {
                            <label class="category"><input
                                type="checkbox"
                                name=&n.name
                                value=n.id
                                on:change=move |ev| {
                                    update_filter(cat_filter, set_cat_filter, ev, n.id);
                                }
                                checked = !cat_filter.get().contains(&n.id)
                            />
                                {n.name}
                            </label><br/>
                            {
                                move || sub.data.get().unwrap_or(Some(vec![])).unwrap().into_iter()
                                    .filter(|sn| {sn.category_id == n.id})
                                    .map(|sn| {
                                        view! {
                                            <label class="subcategory"><input
                                                type="checkbox"
                                                name=&sn.name
                                                value=sn.id
                                                on:change=move |ev| {
                                                    update_filter(subcat_filter, set_subcat_filter, ev, sn.id);
                                                }
                                                checked = !subcat_filter.get().contains(&sn.id)
                                                            && !cat_filter.get().contains(&n.id)
                                            />
                                                {sn.name}
                                            </label><br/>
                                    }}).collect::<Vec<_>>()
                            }
                        })
                        .collect::<Vec<_>>()
                }
              </div>
            </div>

            <div class="search">
                <input
                    type="search"
                    on:input=move |ev| {
                        let txt = event_target_value(&ev);
                        set_query(txt);
                    }
                />
                // <svg xmlns="http://www.w3.org/2000/svg" width="16"
                //     height="16" fill="currentColor"
                //     class="bi bi-search" viewBox="0 0 16 16">
                //   <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0"/>
                // </svg>
            </div>
        </nav>
    }
}

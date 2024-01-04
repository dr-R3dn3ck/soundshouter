use std::collections::HashSet;
use leptos::{component, IntoView, view, create_signal, SignalGet};

pub mod sound;
mod nav;

use crate::ui::sound::{SoundGrid};
use crate::ui::nav::SoundShouterNav;

#[component]
pub fn SoundShouter() -> impl IntoView {
    // category filter
    let (category_filter, set_category_filter) = create_signal(HashSet::<i32>::new());
    let (subcategory_filter, set_subcategory_filter) = create_signal(HashSet::<i32>::new());

    // search filter
    let (query, set_query) = create_signal("".to_string());

    view! {
        {move || {
            view! {
                <div>
                    {format!("category filter = {:?} ", category_filter.get())}
                    {format!("subcategory filter = {:?} ", subcategory_filter.get())}
                    {format!("search query = \"{}\" ", query.get())}
                </div>
            }
        }}
        <SoundShouterNav
            cat_filter=category_filter
            set_cat_filter=set_category_filter
            subcat_filter=subcategory_filter
            set_subcat_filter=set_subcategory_filter
            set_query=set_query
        />
        <SoundGrid
            cat_filter=category_filter
            subcat_filter=subcategory_filter
            query=query
        />
    }
}
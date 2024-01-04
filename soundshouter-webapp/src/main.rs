mod api;
mod ui;

use leptos_query::*;
use leptos::*;

use ui::{SoundShouter};


fn main() {
    provide_query_client();

    mount_to_body(move ||
        view! {class=class_name,
            <SoundShouter />
        }
    )
}
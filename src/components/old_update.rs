use leptos::*;
use std::time::Duration;

#[component]
pub fn OldUpdates(length: usize) -> impl IntoView {
    // Create a signal to hold the items, initialized with the range from 0 to length
    let (items, set_items) = create_signal((0..length).collect::<Vec<_>>());

    set_interval(
        move || {
            set_items.update(|items| {
                for item in items.iter_mut() {
                    *item += 1;
                }
            });
        },
        Duration::from_millis(100),
    );

    view! {
    <div>
        {move || items.get().iter().map(|&item| view! {
            <div class="item">{item}</div>
        }).collect::<Vec<_>>()}
        </div>
    }
}

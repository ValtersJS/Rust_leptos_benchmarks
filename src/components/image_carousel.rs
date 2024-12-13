use leptos::*;

#[component]
pub fn ImageCarousel(images: Vec<String>) -> impl IntoView {
    let (index, set_index) = create_signal(0);

    let image_count = images.len();

    let handle_next_image = move |_| {
        // set_index.update(|index: &mut i32| *index += 1);
        set_index.update(|index| *index = (*index + 1) % image_count);
    };

    view! {
        <div>
            <button class="btn-prev"
            on:click=handle_next_image>
            "Next"
            </button>
            <img class="w-36 h-36" src=move || images.get(index()).cloned().unwrap_or_default() />

        </div>
    }
}

use leptos::*;
use leptos_router::*;

use super::components::div_rendering::DivRendering;
use super::components::image_carousel::ImageCarousel;
use super::components::old_update::OldUpdates;
use super::components::static_list::StaticList;

#[component]
pub fn App() -> impl IntoView {
    let images = vec![
        "dis.svg".to_string(),
        "pic.jpg".to_string(),
        "dis.svg".to_string(),
    ];

    let images_signal = create_rw_signal(images);

    view! {
    <Router>
        <nav class="sticky top-0 z-10 block w-full max-w-full px-4 py-2 text-white bg-white border rounded-none shadow-md h-max border-white/80 bg-opacity-80 backdrop-blur-2xl backdrop-saturate-200 lg:px-8 lg:py-4">
            <p>Hello</p>
            <A href="/">"Home"</A>
            <A href="/counters">"Counters"</A>
            <A href="/images">"Images"</A>
            <A href="/divs">"Divs"</A>
            <A href="/updates">"Updates"</A>
        </nav>
        <Routes>
            <Route
                path="/"
                view=move || view! {
                    <StaticList length=10/>
                    <DivRendering/>
                    <ImageCarousel images=images_signal.get()/>
                    <OldUpdates length=100 />
                }
            />
            <Route path="/counters" view=move || view! { <StaticList length=10/> } />
            <Route path="/images" view=move || view! { <ImageCarousel images=images_signal.get()/> } />
            <Route path="/divs" view=move || view! { <DivRendering/> } />
            <Route path="/updates" view=move || view! { <OldUpdates length=10000/> } />
        </Routes>

    </Router>
    }
}

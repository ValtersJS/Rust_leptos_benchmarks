use leptos::set_interval;
use leptos::*;
use leptos_router::*;
use std::time::Duration;
use web_sys::window;

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
                }
            />
            <Route path="/counters" view=move || view! { <StaticList length=10/> } />
            <Route path="/images" view=move || view! { <ImageCarousel images=images_signal.get()/> } />
            <Route path="/divs" view=move || view! { <DivRendering/> } />
            <Route path="/updates" view=move || view! { <MassiveUpdates length=10000/> } />
        </Routes>

    </Router>
    }
}

#[derive(Clone, Copy)]
struct Counter {
    id: usize,
    value: i32,
}

#[component]
fn StaticList(length: usize) -> impl IntoView {
    let counters = (1..=length)
        .map(|id| create_signal(Counter { id, value: 0 }))
        .collect::<Vec<_>>();

    let color_count = create_rw_signal(0);

    create_effect(move |_| {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let style = body.style();

        if color_count.get() % 2 == 0 {
            style.set_property("background-color", "lightblue").unwrap();
        } else {
            style
                .set_property("background-color", "lightcoral")
                .unwrap();
        }
    });

    let handle_click = move |counter: WriteSignal<Counter>| {
        counter.update(|c: &mut Counter| c.value += 1);
        color_count.update(|count: &mut i32| *count += 1);
    };

    let counter_buttons = counters
        .into_iter()
        .map(|(counter, set_counter)| {
            view! {
            <button class="h-fit align-middle select-none font-sans font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none text-xs py-3 px-6 rounded-lg bg-gray-900 text-white shadow-md shadow-gray-900/10 hover:shadow-lg hover:shadow-gray-900/20 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none"
              on:click=move |_| handle_click(set_counter)>
                {move || counter.get().value}
                {move || counter.get().id}
            </button>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div>
            <ul class="flex flex-row space-x-4 h-fit">{counter_buttons}</ul>
            <Outlet/>
        </div>
    }
}

#[component]
fn DivRendering(#[prop(default = 3)] count: usize) -> impl IntoView {
    let divs = (0..count)
        .map(|_| {
            view! {
                <div>
                    <div class="pl-12 text-sky-400">"heel0o"</div>
                    <img src="dis.svg" alt="alt" height="100" width="100"/>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div>{divs}</div>
    }
}

#[component]
fn ImageCarousel(images: Vec<String>) -> impl IntoView {
    let (index, set_index) = create_signal(0);

    let image_count = images.len();

    let handle_next_image = move |_| {
        // set_index.update(|index: &mut i32| *index += 1);
        set_index.update(|index| *index = (*index + 1) % image_count);
    };

    // let handle_previous_image = move |_| {
    //     set_index.update(|index| *index -= 1);
    // };

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

#[component]
fn MassiveUpdates(length: usize) -> impl IntoView {
    // Create a signal to hold the items, initialized with the range from 0 to length
    let (items, set_items) = create_signal((0..length).collect::<Vec<_>>());

    // Effect to update the items every 100ms
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

    // Render the list of items
    view! {
        <div>
            {move || items.get().iter().map(|&item| view! {
                <div class="item">{item}</div>
            }).collect::<Vec<_>>()}
        </div>
    }
}

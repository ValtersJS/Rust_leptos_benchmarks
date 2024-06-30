use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let images = vec![
        "dis.svg".to_string(),
        "pic.jpg".to_string(),
        "dis.svg".to_string(),
    ];

    view! {
            <Router>
                <nav class="sticky top-0 z-10 block w-full max-w-full px-4 py-2 text-white bg-white border rounded-none shadow-md h-max border-white/80 bg-opacity-80 backdrop-blur-2xl backdrop-saturate-200 lg:px-8 lg:py-4">
                    <p>Hello</p>
                </nav>
                <Routes>
    /*                 <Route path="/" view=|| view! { <StaticList length=10/> }>
                        <Route path="" view=|| view! {<DivRendering/>} />
                        <Route path="img" view=move || view! { <ImageCarousel images=images.clone()/> } />
                    </Route> */

                    <Route path="/" view=|| view! {
                        <StaticList length=10/>
                        <DivRendering/>
                    }>
                        <Route path="img" view=move || view! { <ImageCarousel images=images.clone()/> } />
                    </Route>
                    // <Route path="/img" view=move || view! { <ImageCarousel images=images.clone()/> } />
                </Routes>
                // <h1>"Static list"</h1>
                // <StaticList length=10/>
                // <DivRendering count=10/>
            </Router>
        }
}

#[component]
fn StaticList(length: usize) -> impl IntoView {
    let counters = (1..length).map(|idx| create_signal(idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                    <button class="h-fit align-middle select-none font-sans font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none text-xs py-3 px-6 rounded-lg bg-gray-900 text-white shadow-md shadow-gray-900/10 hover:shadow-lg hover:shadow-gray-900/20 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none"
                     on:click=move |_| set_count.update(|n| *n += 1)>
                        {count}
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
                    <img class="profile-pic" src="https://avatars.githubusercontent.com/u/46683255?s=96&v=4" alt="github profile picture"/>
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
    //     set_index.update(|index: &mut i32| *index -= 1);
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

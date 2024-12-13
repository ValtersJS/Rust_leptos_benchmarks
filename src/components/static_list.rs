use leptos::*;
use leptos_router::*;

use web_sys::window;

#[derive(Clone, Copy)]
struct Counter {
    id: usize,
    value: i32,
}

#[component]
pub fn StaticList(length: usize) -> impl IntoView {
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

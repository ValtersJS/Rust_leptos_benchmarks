use leptos::*;

fn main() {
    // _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component()]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    for i in 1..=10 {
        view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count() % 2 == 1
        >
            "Click me: "
            {count}
        </button>

        <br/>

        // <progress max="50" value=count></progress>
        <ProgressBar progress=count/>
        <br/>
        // <progress max="50" value=double_count></progress>
        <ProgressBar progress=Signal::derive(double_count)/>
        <br/>
        <p>"Double count: " {double_count}</p>
        }
    }
}

// 'static is lifetime. -> i32 means: return i32
/// Shows progress toward a goal
#[component()]
// <F> would indicate that ProgressBar is a generic type that takes the param F
fn ProgressBar(
    #[prop(default = 10)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
    }
}

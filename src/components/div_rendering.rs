use leptos::*;

#[component]
pub fn DivRendering(#[prop(default = 3)] count: usize) -> impl IntoView {
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

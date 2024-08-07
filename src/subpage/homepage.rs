use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let onclick = move |_| {
        set_count.update(|n| *n += 1);
    };

    view! {
        <h1>"Home"</h1>
        <p>count is {count}</p>
        <button on:click=onclick>Click Me!!</button>
    }
}

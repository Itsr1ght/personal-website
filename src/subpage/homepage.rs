use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("home".to_string());

    view! {
        <div>
            <div class="tabs">
                <button on:click=move|_| set_active_tab("home".to_string())>Home</button>
                <button on:click=move|_| set_active_tab("about".to_string())>About</button>
                <button on:click=move|_| set_active_tab("contact".to_string())>Contact</button>
            </div>
            <div class="tab-content">{
                move|| {
                    match active_tab.get().as_str(){
                        "home" => view! {<p>Home</p>},
                        "about" => view! {<p>About</p>},
                        "contact" => view! {<p>Contact</p>},
                        _ => view!{<p>404</p>},
                    }
                }
            }
            </div>
        </div>
    }
}

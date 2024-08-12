use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("home".to_string());

    view! {
        <div>
            <div class="container">
                <div class="box"><h1 class="tabs">BJ</h1></div>
                <div class="tabs">
                    <button on:click=move|_| set_active_tab("home".to_string())>Home</button>
                    <button on:click=move|_| set_active_tab("projects".to_string())>Projects</button>
                    <button on:click=move|_| set_active_tab("about".to_string())>About</button>
                    <button on:click=move|_| set_active_tab("contact".to_string())>Contact</button>
                </div>
            </div>
            <div class="tab-content">{
                move|| {
                    match active_tab.get().as_str(){
                        "home" => view! {<p>Home</p>},
                        "projects" => view! {<p>Projects</p>},
                        "about" => view! {<p>About</p>},
                        "contact" => view! {<p>Contact</p>},
                        _ => view!{<p>404</p>},
                    }
                }
            }
                <div class="foot">
                    <div class="footer">
                        <div>made with</div>
                        <a href="https://leptos.dev/" target="_blank">
                            <img src="https://www.leptos.dev/images/header_logo.svg" height=20 alt="Leptos logo" />
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}

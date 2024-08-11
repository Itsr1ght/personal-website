use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::subpage::{homepage::HomePage, not_found::NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="color-scheme" content="dark"/>
        <Stylesheet id="leptos" href="/pkg/personal-website.css"/>
        <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap" rel="stylesheet"/>
        <Title text="Welcome to My Site!"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}


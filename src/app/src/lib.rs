mod top_nav;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use top_nav::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    
    view! {
        cx,
        <div class="">
            <Stylesheet id="leptos" href="/pkg/lekiu.css"/>
            <Title text="Cargo Leptos" />
            <AppRoute />
        </div>
    }
}

#[component]
pub fn AppRoute(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <TopNav />
            <div class="py-9 px-28">
                <Routes>
                    <Route path="/" view=RootPage />
                    <Route path="/blog/:id" view=Blog />
                    <Route path="/blogs" view=BlogList />
                </Routes>
            </div>
        </Router>
    }
}


#[component]
pub fn RootPage(cx: Scope) -> impl IntoView {
    view! {cx,
        <strong class="">"Root Page"</strong>
    }
}

#[component]
pub(crate) fn Blog(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <strong>"Blog Page"</strong>
    }
}

#[component]
pub(crate) fn BlogList(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <strong>"Blog List"</strong>
    }
}


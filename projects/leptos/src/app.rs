use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <Splash />
    }
}

/// First Component
#[component]
fn Splash(cx: Scope) -> impl IntoView {
    let (splash_count, set_splash_count) = create_signal(cx, 20);
    let on_click_half = move |_| set_splash_count.update(|splash_count| *splash_count /= 2);
    let on_click_double = move |_| set_splash_count.update(|splash_count| *splash_count *= 2);

    view! {
      cx, <div>"Splash Component"</div>
        <button on:click=on_click_half>"Half"</button>
        <span>{splash_count}</span>
        <button on:click=on_click_double>"Double"</button>
    }
}

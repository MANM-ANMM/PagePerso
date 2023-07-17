use leptos::*;
use leptos_router::*;
use leptos_meta::*;

fn main() {
    mount_to_body(|cx| view! {cx , <App/> })
}

#[component]
fn App (cx: Scope) -> impl IntoView {

    view! { 
        cx,
        <Style>{include_str!("style.css")}</Style>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Home/> } />
                    <Route path="/explosivite" view=|cx| view! { cx, <Explosivite/> } />
                </Routes>
            </main>
            <nav>
                <A href="/">"Home"</A>
                <A href="/explosivite">"Explosivite"</A>
            </nav>

        </Router>
    }
}


#[component]
fn Explosivite(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="page">
            <h1>"Explosivité"</h1>
            {"Explosivité est un plateformer dans lequel l'énergie pour effectuer des actions spéciales comme des sauts est aussi la vie du personnage."}
        </div>
    }
}


#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <h1>"Salut page 1"</h1>

        </div>
    }
}

#[component]
fn ProgressBar (
    cx: Scope, 
    #[prop(default = 100)]
    /// Maximum value for the progress bar
    max : u16,
    /// How much progress should be displayed
    #[prop(into)]
    progress: Signal<i32>
    ) -> impl IntoView {
    view! {cx, 
        <progress
            max = max
            value = progress
        />
    }
}


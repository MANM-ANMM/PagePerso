use leptos::*;
use leptos_meta::*;
use leptos_router::*;

fn main() {
    mount_to_body(|cx| view! {cx , <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Style>{include_str!("style.css")}</Style>
        <Router>
            <main>
            <Routes>
                <Route path={PATH_HOME} view=|cx| view! { cx, <Home/> } />
                <Route path={PATH_EXPLOSIVITE} view=|cx| view! { cx, <Explosivite/> } />
            </Routes>
            <div class="foot_space"/>
            </main>
            <Navigation/>
        </Router>
    }
}

#[component]
fn Explosivite(cx: Scope) -> impl IntoView {
    view! { cx,
    <div class="explosivite">
        <h1>"Explosivité"</h1>
        <p>"Explosivité est un plateformer dans lequel l'énergie pour effectuer des actions spéciales comme des sauts est aussi la vie du personnage."</p>
        <img src="img/explosivite.png" alt="Image titre du jeu Explosivité"/>

        <iframe src="https://itch.io/embed/893284" width=552 height=167 frameborder=0>
            <a href="https://anmm.itch.io/explosivite">
                "Explosivite by ANMM"
            </a>
        </iframe>

        <p>"J'ai créé Explosivité pendant ma première année de DUT (2020-2021)"</p>
    </div>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="home">
            <h1>"Martin Rouault"</h1>
            <p>""</p>
        </div>
    }
}

#[component]
fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <A href={PATH_HOME}>"Home"</A>
            <A href={PATH_EXPLOSIVITE}>"Explosivite"</A>
        </nav>
    }
}

static PATH_HOME: &str = "/";
static PATH_EXPLOSIVITE: &str = "/explosivite";

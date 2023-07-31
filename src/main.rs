use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use const_format::concatcp;

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
				<Route path={PATH_SITE} view=|cx| view! { cx, <Site/> } />
				<Route path="/*any" view=|cx| view! { cx, <Home/> } />
			</Routes>
			<div class="foot_space"/>
			</main>
			<Navigation/>
		</Router>
	}
}

#[component]
fn Site(cx: Scope) -> impl IntoView {
	view! { cx,
	<div class="site">
		<h1>Ce site</h1>
		<p>"Ce site expose une partie des projets que j'ai créé ces dernières années. Le code de ce site est disponible sur "<a href="https://github.com/MANM-ANMM/PagePerso">github</a>"."</p>
		<p>"J'ai dévelopé ce site en Rust, en utilisant la bibliothèque leptos."</p>
		<iframe src={PATH_ROOT}/>
	</div>
	}
}

#[component]
fn Explosivite(cx: Scope) -> impl IntoView {
	view! { cx,
	<div class="explosivite">
		<h1>"Explosivité"</h1>
		<p>"Explosivité est un plateformer dans lequel l'énergie pour effectuer des actions spéciales comme des sauts est aussi la vie du personnage."</p>
		<img src={PATH_ROOT.to_owned() + "img/explosivite.png"} alt="Image titre du jeu Explosivité"/>

		<iframe src="https://itch.io/embed/893284" width=552 height=167 frameborder=0>
			<a href="https://anmm.itch.io/explosivite">
				"Explosivite by ANMM"
			</a>
		</iframe>

		<p>"J'ai créé Explosivité pendant ma première année de DUT (2020-2021) avec Godot."</p>
	</div>
	}
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="home">
			<h1>"Martin Rouault"</h1>
			<p>"Je suis étudiant en informatique en 2ème année d'école d'ingénieur."</p>
			<p>"La plus part des projets ici sont des jeux-vidéo développés dans mon temps libre. Soit sous forme de projet au long court, soit en game jam de 48 heures."</p>
		</div>
	}
}

#[component]
fn Navigation(cx: Scope) -> impl IntoView {
	view! { cx,
		<nav>
			<A href={PATH_HOME}>"Home"</A>
			<A href={PATH_EXPLOSIVITE}>"Explosivite"</A>
			<A href={PATH_SITE}>"Ce site"</A>
		</nav>
	}
}


const PATH_ROOT : &str = match cfg!(debug_assertions) {
	true => "/",
	false => "/~marouault/",
};

static PATH_HOME: &str = PATH_ROOT;
static PATH_EXPLOSIVITE: &str = concatcp!(PATH_ROOT, "explosivite");
static PATH_SITE: &str = concatcp!(PATH_ROOT, "site");



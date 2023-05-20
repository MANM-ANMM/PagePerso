use std::str::FromStr;

use leptos::*;

fn main() {
    let projs = vec![Projet::Jeu(JeuData { titre: String::from_str("yoo").unwrap() }), Projet::Programme(ProgrammeData { nom: String::from_str("Pouineatr").unwrap() })]; 

    mount_to_body(|cx| view! {cx , <App projets=projs/> })

}

enum Projet {
    Jeu(JeuData),
    Programme(ProgrammeData),
}

struct JeuData {
    titre:String
}

struct ProgrammeData {
    nom:String,
}

#[derive(Clone, PartialEq)]
enum Page {
    Page1,
    Page2,
}

#[component]
fn App (cx: Scope, projets:Vec<Projet>) -> impl IntoView {
    

    let (page, set_page) = create_signal(cx, Page::Page1);
    
    view! { 
        cx,
        <button
            on:click=move |_| {
                set_page(
                    match page() {
                        Page::Page1 => Page::Page2,
                        Page::Page2 => Page::Page1,
                    }    
                )
            }
            >
                "Switch page"
        </button>
        
        <div class:project_list=true>
            {
                projets.iter().map(|p| view! {
                    cx,
                    <p>{
                        match p {
                            Projet::Jeu(n) => n.titre.clone(),
                            Projet::Programme(n) => n.nom.clone(),
                        }
                    }</p>
                }).collect::<Vec<_>>()
            }
        </div>


        <Show
            when= move || page.get() == Page::Page1
            fallback= |cx| view! {cx, "Geauie"}
            >
                <Page1/>
        </Show>
    }
}

#[component]
fn Page1(cx: Scope) -> impl IntoView {
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


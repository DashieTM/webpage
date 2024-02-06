use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/webpage.css"/>
        <Title text="DashieDev"/>
        <Router>
            <main class="grid">
                <Navigation/>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/Games" view=GamePage />
                    <Route path="/Games/Stapid" view=Stapid />
                    <Route path="/*any" view=NotFound />
                </Routes>
                <footer class="footer">
                    Made with <a href="https://leptos.dev/">Leptos</a>
                </footer>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="regular responsive">
            <h1>Dashie/Fabio Lenherr</h1>
            <span class="text">
            "I am currently a full time student at OST (Eastern Switzerland University of Applied Sciences) studying IT.\nMy big passions in IT lie with everything open source, notably, Linux, Neovim, or the rust programming language."
            </span>
            <h2>"My Projects"</h2>
        </div>
        <CardView>
            <Card
                link="https://github.com/DashieTM/hyprdock".into()
                name="Hyprdock".into()
                desc="A wrapper docking tool for Hyprland written in rust.".into()
                picture="/assets/hyprdock.svg".into()
            />
            <Card
                link="https://github.com/DashieTM/OxiCalc".into()
                name="Oxicalc".into()
                desc="A gtk calculator written in rust.".into()
                picture="/assets/calculator.svg".into()
            />
            <Card
                link="https://github.com/DashieTM/OxiNoti".into()
                name="Oxinoti".into()
                desc="A notification daemon written in rust.".into()
                picture="/assets/oxinoti.svg".into()
            />
            <Card
                link="https://github.com/Xetibo/ReSet".into()
                name="ReSet".into()
                desc="An environment agnostic settings application written in rust.".into()
                picture="/assets/ReSet.svg".into()
            />
            <Card
                link="https://github.com/DashieTM/OxiShut".into()
                name="OxiShut".into()
                desc="A logout prompt written in gtk/rust.".into()
                picture="/assets/oxishut.svg".into()
            />
            <Card
                link="https://github.com/Xetibo/stapid".into()
                name="Stapid".into()
                desc="A small test game made with bevy/rust.".into()
                picture="/assets/stick.png".into()
            />
        </CardView>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    // #[cfg(feature = "ssr")]
    // {
    //     let resp = expect_context::<leptos_actix::ResponseOptions>();
    //     resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    // }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[component]
fn GamePage() -> impl IntoView {
    view! {
    <div class="regular responsive">
      <div>
        <h1>Games</h1>
        <h2>GlobiGaem</h2>
        <span class="text">
          "This was my first attempt at creating a game or any program in general.\nIt was one of the driving factors that lead me to programming.\nThis little game is just here as legacy."
        </span>
      </div>
      <iframe class="stapid youtube" src="https://www.youtube-nocookie.com/embed/Z9ddOhKWpTQ?si=7KwTKJNkiToXbWqd" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
      <div>
        <h2>Stapid</h2>
        <span class="text">
          "This 2D game was my proper attempt at learning the rust language.\nWhile it doesn't look very fancy,\n it did it's job as entertainment at the family Xmas party."
        </span>
      </div>
      <div class="stapid-spacer">
        <A href="/Games/Stapid" id="stapid-button">Play Stapid</A>
      </div>
      <img src="/assets/stapid.png" alt="Picture of stapid" class="stapid"/>
    </div>
    }
}

#[component]
fn Stapid() -> impl IntoView {
    view! {
      <Script type_="module">
        "import init from '/websrc/stapid.js';
         init().catch((error) => {
              if (!error.message.startsWith('Using exceptions for control flow.')) {
                throw error;
              }
            });" 
      </Script>
      <canvas id="stapid-canvas" class="regular" width="1920" height="1080"></canvas>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <div class="navigation regular responsive">
            <A href="/" class="navigation-button">Home</A>
            <A href="/Games" class="navigation-button">Games</A>
            <div class="navigation-end">
                <svg class="navigation-button" on:click=move |_|{ window().location().set_href("https://github.com/DashieTM").expect("Could not move to Github.");} xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-github"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path></svg>
                <svg class="navigation-button" on:click=move |_|{ window().location().set_href("https://matrix.to/#/%21sdKHhlAnzYUzkxvRfX%3Amatrix.org?via=matrix.org").expect("Could not access Matrix.");}  xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-message-circle"><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path></svg>
                <svg class="navigation-button" on:click=move |_|{ window().location().set_href("mailto:fabio.lenherr@gmail.com").expect("Could not create Email.");}  xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-mail"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path><polyline points="22,6 12,13 2,6"></polyline></svg>
            </div>
        </div>
    }
}

#[component]
fn CardView(children: Children) -> impl IntoView {
    view! {
        <div class="card-view">
            {children()}
        </div>
    }
}

#[component]
fn Card(link: String, name: String, desc: String, picture: String) -> impl IntoView {
    let picture_path = if !picture.is_empty() {
        picture.clone()
    } else {
        "/assets/empty2.svg".to_string()
    };
    view! {
        <div class="card">
            <div>
                <h3 class="card-title">{&name}</h3>
                <h4 class="description card-description">{desc}</h4>
                <a href={link} class="card-repository">Repository</a>
            </div>
            <img src={&picture_path} class="card-picture" alt={&picture_path} width=100 height=100 />// alt="Picture of ".to_string() + {&name} width=100 height=100 />
        </div>
    }
}

use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx!(
        div {
            text_align: "center",
            background_color: "gray",
            h1 { "High-Five counter: {count}" }
            img { src: "ferris.png", width: "10%", height: "10%"}
            br{}
            button { onclick: move |_| set_count(count + 1), "Up high!"}
            br{}
            button { onclick: move |_| set_count(count - 1), "Down low!" }
        }
    ))
}
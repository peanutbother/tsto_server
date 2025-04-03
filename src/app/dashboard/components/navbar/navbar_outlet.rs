use dioxus::prelude::*;

const BACKGROUND: Asset = asset!("/assets/background.png");
const BACKGROUND_DARK: Asset = asset!("/assets/background_dark.png");

#[component]
pub fn NavbarOutlet(children: Element) -> Element {
    rsx! {

        style { dangerous_inner_html: ".bg {{
            background-image: url({BACKGROUND});
            }}
            @media (prefers-color-scheme: dark) {{
                .bg {{
                    background-image: url({BACKGROUND_DARK.to_string()});
                }}
            }}" }
        div { class: "pt-16 min-h-screen flex flex-col items-center justify-center dark:bg-blue-800 bg-cover bg",
            {children}
        }
    }
}

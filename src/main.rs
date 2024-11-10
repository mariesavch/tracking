#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus::launch(App);
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: TrackingData,
}

#[derive(Debug, Deserialize)]
struct TrackingData {
    checkpoints: Vec<Checkpoint>,
}
#[derive(Debug, Deserialize)]
struct Checkpoint {
    time: String,
    status_raw: String,
}

async fn get_tracker_info(
    tracking_number: &str,
    provider: &str,
) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!(
        "https://corsanywhere-two.vercel.app/api/proxy?url=https://gdeposylka.ru/api/v4/tracker/{}/{}",
        provider, tracking_number
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "X-Authorization-Token",
        HeaderValue::from_static(
            "e1e9872ba84c0e91a99bf560f92bf60b572cb03074497d59021c3f5904494f6103cfd9b227c4ed9e",
        ),
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        eprintln!("Error: {} - Response Body: {}", status, body);
        return Err(format!("Request failed: {}", status).into());
    }

    let response_body = response.text().await?;

    let api_response: ApiResponse = from_str(&response_body).map_err(|e| {
        eprintln!("Failed to decode response body: {}", e);
        e
    })?;

    Ok(api_response)
}

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        ":root" {
            __rosewater: "#ff8389",
            __flamingo: "#ff8389",
            __red: "#ff8389",
            __maroon: "#ff8389",
            __pink: "#ff7eb6",
            __mauve: "#be95ff",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#08bdba",
            __teal: "#33b1ff",
            __sky: "#33b1ff",
            __sapphire: "#33b1ff",
            __blue: "#78a9ff",
            __lavender: "#78a9ff",
            __text: "#ffffff",
            __subtext1: "#f4f4f4",
            __subtext0: "#e0e0e0",
            __overlay2: "#adadad",
            __overlay1: "#949494",
            __overlay0: "#7a7a7a",
            __surface2: "#4f4f4f",
            __surface1: "#383838",
            __surface0: "#2e2e2e",
            __base: "#161616",
            __mantle: "#0d0d0d",
            __crust: "#000000",
        },
    "@media (prefers-color-scheme: light)" {
        ":root" {
            __rosewater: "#da1e28",
            __flamingo: "#da1e28",
            __red: "#da1e28",
            __maroon: "#da1e28",
            __pink: "#d02670",
            __mauve: "#8a3ffc",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#007d79",
            __teal: "#1192e8",
            __sky: "#1192e8",
            __sapphire: "#1192e8",
            __blue: "#0f62fe",
            __lavender: "#0f62fe",
            __text: "#000000",
            __subtext1: "#404040",
            __subtext0: "#474747",
            __overlay2: "#575757",
            __overlay1: "#595959",
            __overlay0: "#737373",
            __surface2: "#8c8c8c",
            __surface1: "#d1d1d1",
            __surface0: "#e6e6e6",
            __base: "#ffffff",
            __mantle: "#f2f2f2",
            __crust: "#ebebeb",
            }
        },
        ":root" {
            background_color: "var(--base)",
            color: "var(--text)",
            line_height: "1.6",
            font_family: "Cartograph CF",
        },
        "@media (hover: hover) and (pointer: fine)" {
            ".animated_list li" {
                all: "unset",
                transition_property: "all",
                transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
                transition_duration: "300ms",
            },
            ".animated_list:hover li" {
                opacity: "0.5",
            },
            ".animated_list:hover li:hover" {
                opacity: "1",
            }
        },
        ".input" {
            all: "unset",
            padding_top: "0.5rem",
            padding_bottom: "0.5rem",
            padding_left: "1rem",
            padding_right: "1rem",
            border: "1px solid var(--surface0)",
            text_transform: "capitalize",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            color: "var(--text)",
            width: "100%",
        },
        ".input:hover" {
            border_color: "var(--surface1)",
        },
        ".input:focus" {
            border_color: "var(--surface2)",
        },
        ".input:placeholder" {
            color: "var(--overlay0)",
        },
        ".section" {
            padding_top: "24px",
        },
        "@media(min-width: 950px)" {
            ".section" {
                padding_top: "64px",
            }
        },
    }
}

#[component]
fn App() -> Element {
    use_style_provider_quickstart(|| EmptyTheme);
    let cls: &MyClasses = &MyClasses::use_style();

    let mut track_number =
        use_synced_storage::<LocalStorage, String>("track_number".to_string(), || {
            "AER008741799".to_string()
        });
    let mut track_provider =
        use_synced_storage::<LocalStorage, String>("track_provider".to_string(), || {
            "cainiao".to_string()
        });
    let tracking_info = use_resource(move || async move {
        get_tracker_info(&track_number.to_string(), &track_provider.to_string()).await
    });

    rsx! {
        style {
            "
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Regular.woff2') format('woff2');
            }}
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-RegularItalic.woff2') format('woff2');
            font-style: italic;
            }}
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Bold.woff2') format('woff2');
            font-weight: bold;
            }}"
        }
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: &cls.section as &str,
                div { display: "flex",
                    input {
                        aria_label: "Enter track number",
                        placeholder: "Enter track number",
                        spellcheck: false,
                        value: track_number,
                        r#type: "text",
                        autofocus: true,
                        oninput: move |event| track_number.set(event.value()),
                        class: &cls.input as &str,
                    }
                    input {
                        aria_label: "Enter provider eg: cainiao",
                        placeholder: "Enter provider eg: cainiao",
                        spellcheck: false,
                        value: track_provider,
                        r#type: "text",
                        autofocus: true,
                        oninput: move |event| track_provider.set(event.value()),
                        class: &cls.input as &str,
                    }
                }
                div { margin_top: "32px",
                    if let Some(Ok(data)) = tracking_info.read().as_ref() {
                        ul {
                            all: "unset",
                            class: &cls.animated_list as &str,
                            display: "flex",
                            flex_direction: "column",
                            {data.data.checkpoints.iter().map(|trackdata| rsx! {
                                li { padding_bottom: "32px",
                                    span { font_style: "bold", margin_right: "20px",
                                        if trackdata.status_raw == "GTMS_SIGNED" {
                                            "Reveived"
                                        } else {
                                            "{trackdata.status_raw}"
                                        }
                                    }
                                    span { color: "var(--overlay0)", font_style: "italic", "{trackdata.time}" }
                                }
                            })}
                        }
                    } else if let Some(Err(error)) = tracking_info.read().as_ref() {
                        p { font_style: "bold", color: "var(--red)", "Try again :) error: {error}" }
                    } else {
                        p { font_style: "italic", color: "var(--overlay0)", "loading..." }
                    }
                }
            }
        }
    }
}

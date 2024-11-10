#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::from_str;
use sir::{css, global_css, AppStyle};
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

#[component]
fn App() -> Element {
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

    global_css!(
        "
       :root {
        --rosewater: #ff8389;
        --flamingo: #ff8389;
        --red: #ff8389;
        --maroon: #ff8389;
        --pink: #ff7eb6;
        --mauve: #be95ff;
        --peach: #d44a1c;
        --yellow: #ab8600;
        --green: #08bdba;
        --teal: #33b1ff;
        --sky: #33b1ff;
        --sapphire: #33b1ff;
        --blue: #78a9ff;
        --lavender: #78a9ff;
        --text: #ffffff;
        --subtext1: #f4f4f4;
        --subtext0: #e0e0e0;
        --overlay2: #adadad;
        --overlay1: #949494;
        --overlay0: #7a7a7a;
        --surface2: #4f4f4f;
        --surface1: #383838;
        --surface0: #2e2e2e;
        --base: #161616;
        --mantle: #0d0d0d;
        --crust: #000000;
    } 

    @media (prefers-color-scheme: light) {
        :root {
            --rosewater: #da1e28;
            --flamingo: #da1e28;
            --red: #da1e28;
            --maroon: #da1e28;
            --pink: #d02670;
            --mauve: #8a3ffc;
            --peach: #d44a1c;
            --yellow: #ab8600;
            --green: #007d79;
            --teal: #1192e8;
            --sky: #1192e8;
            --sapphire: #1192e8;
            --blue: #0f62fe;
            --lavender: #0f62fe;
            --text: #000000;
            --subtext1: #404040;
            --subtext0: #474747;
            --overlay2: #575757;
            --overlay1: #595959;
            --overlay0: #737373;
            --surface2: #8c8c8c;
            --surface1: #d1d1d1;
            --surface0: #e6e6e6;
            --base: #ffffff;
            --mantle: #f2f2f2;
            --crust: #ebebeb;
        }
    }

    :root {
        background-color: var(--base);
        color: var(--text);
        line-height: 1.6;
    }

    "
    );

    let animated_list = css!(
        "
    @media (hover: hover) and (pointer: fine) {
        li {
            all: unset;
            transition-property: all;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 300ms;
        }
        &:hover li {
            opacity: 0.5;
        }
        &:hover li:hover {
            opacity: 1;
        }
    }
    "
    );

    let input = css!(
        "
        all: unset;
        padding-top: 0.5rem;
        padding-bottom: 0.5rem; 
        padding-left: 1rem;
        padding-right: 1rem;
        border: 1px solid var(--surface0); 
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms; 
        color: var(--text);
        width: 100%;

        &:hover {
            border-color: var(--surface1);
        }
        &:focus {
            border-color: var(--surface2);
        }
        &:placeholder {
            color: var(--overlay0);
        }
        "
    );

    rsx! {
        AppStyle {}
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: css!("padding-top: 24px; @media(min-width: 950px) { padding-top: 64px; }"),
                div { display: "flex",
                    input {
                        aria_label: "Enter track number",
                        placeholder: "Enter track number",
                        spellcheck: false,
                        value: track_number,
                        r#type: "text",
                        autofocus: true,
                        oninput: move |event| track_number.set(event.value()),
                        class: input,
                        margin_right: "-1px",
                        margin_bottom: "-1px",
                    }
                    input {
                        aria_label: "Enter provider eg: cainiao",
                        placeholder: "Enter provider eg: cainiao",
                        spellcheck: false,
                        value: track_provider,
                        r#type: "text",
                        autofocus: true,
                        oninput: move |event| track_provider.set(event.value()),
                        class: input,
                    }
                }
                div { margin_top: "32px",
                    if let Some(Ok(data)) = tracking_info.read().as_ref() {
                        ul {
                            all: "unset",
                            class: animated_list,
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

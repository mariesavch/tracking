#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;
use tailwind_fuse::tw_merge;

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

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        main { class: "mx-auto max-w-[850px] px-6 pb-20",
            div { class: "pt-6 min-[950px]:pt-16",
                div {
                    input {
                        aria_label: "Enter track number",
                        placeholder: "Enter track number",
                        spellcheck: false,
                        value: track_number,
                        r#type: "text",
                        autofocus: true,
                        oninput: move |event| track_number.set(event.value()),
                        class: tw_merge!(
                            "rounded-t-lg lg:rounded-none lg:rounded-l-lg w-full",
                            "lg:w-1/2 border border-surface0 bg-base py-2 px-4",
                            "outline-none transition-colors duration-300",
                            "placeholder:text-overlay0 hover:border-surface1",
                            "focus:text-text focus:border-surface2 mr-[-1] mb-[-1]"
                        ),
                    }
                    input {
                        aria_label: "Enter provider eg: cainiao",
                        placeholder: "Enter provider eg: cainiao",
                        spellcheck: false,
                        value: track_provider,
                        r#type: "text",
                        autofocus: true,
                        oninput: move |event| track_provider.set(event.value()),
                        class: tw_merge!(
                            "rounded-b-lg lg:rounded-none lg:rounded-r-lg border w-full",
                            "lg:w-1/2 border-surface0 bg-base py-2 px-4",
                            "outline-none transition-colors duration-300",
                            "placeholder:text-overlay0 hover:border-surface1",
                            "focus:text-text focus:border-surface2"
                        ),
                    }
                }
                div { class: "mt-8",
                    if let Some(Ok(data)) = tracking_info.read().as_ref() {
                        ul { class: "animated-list",
                            {data.data.checkpoints.iter().map(|trackdata| rsx! {
                                li { class: "pb-8",
                                    span { class: "font-bold mr-5",
                                        if trackdata.status_raw == "GTMS_SIGNED" {
                                            "Reveived"
                                        } else {
                                            "{trackdata.status_raw}"
                                        }
                                    }
                                    span { class: "text-overlay0 italic", "{trackdata.time}" }
                                }
                            })}
                        }
                    } else if let Some(Err(error)) = tracking_info.read().as_ref() {
                        p { class: "font-bold text-red", "Try again :) error: {error}" }
                    } else {
                        p { class: "text-overlay0 italic", "loading..." }
                    }
                }
            }
        }
    }
}

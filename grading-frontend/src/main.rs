// main.rs

use dioxus::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::FileReader;
use web_sys::Request;
use web_sys::RequestInit;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    // Store the file
    // let (file, set_file) = use_state(&cx, || "".to_string());
    // let (count, set_count) = use_state(&cx, || 0);

    // let file = use_ref(&cx, || File::create("foo.txt").unwrap());

    println!("test");
    log::info!("Reloading path list for ");

    cx.render(rsx! {
        div { "hello, wasm!" }
        // Get a file upload element
        form {
            input {
                r#type: "file",
                onchange: move |evt| {
                    log::info!("{:?}", evt);
                    // Load file from path
                    let path = evt.value.parse::<String>().unwrap();

                    // Request the file through the fetch api
                    // (note, I'm not sure this is the right way to get a file)
                    let opts = RequestInit::new();
                    let request = Request::new_with_str_and_init(&path, &opts).unwrap();

                    use_future(&cx, (), move |_| {
                        async move {
                            let window = web_sys::window().unwrap();
                            let resp_value = match JsFuture::from(window.fetch_with_request(&request))
                                .await {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!("{:?}", e);
                                        return;
                                    }
                                };

                            // Debug the file
                            log::info!("{:?}", resp_value);
                        }
                    });
                },
            }
            input {
                r#type: "submit",
                value: "Submit",
            }
        }
    })
}

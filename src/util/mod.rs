cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::js_sys;
        use web_sys::window;
        use js_sys::Promise;

        pub async fn sleep(ms: u32) {
            let promise = Promise::new(&mut |resolve, _| {
                let window = window().unwrap();
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve,
                        ms as i32,
                    )
                    .unwrap();
            });
            JsFuture::from(promise).await.unwrap();
        }
    }
}



#[cfg(target_arch = "wasm32")]
pub fn play(){
    #[cfg(target_arch = "wasm32")]
    let result = web_sys::HtmlAudioElement::new_with_src("sound/stop.mp3");
    #[cfg(target_arch = "wasm32")]
    let _ = result.unwrap().play();
}
//! `bimcraft-web`: BIMCraft no navegador, um envoltório fino sobre `bimcraft-ui-egui`.

fn main() {
    #[cfg(target_arch = "wasm32")]
    web::main();
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("bimcraft-web compila para wasm32-unknown-unknown: use `trunk serve` ou o aplicativo desktop");
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use std::boxed::Box;

    use bimcraft_core::engine::Session;
    use bimcraft_ui_egui::{BimApp, lang};
    use eframe::CreationContext;
    use serde_json::json;
    use wasm_bindgen::JsCast as _;

    type Failure = Box<dyn std::error::Error + Send + Sync>;

    const CANVAS: &str = "bimcraft_canvas";

    pub fn main() {
        console_error_panic_hook::set_once();
        let options = eframe::WebOptions::default();
        wasm_bindgen_futures::spawn_local(async move {
            let Some(canvas) = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id(CANVAS))
                .and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            else {
                web_sys::console::error_1(&format!("BIMCraft: no <canvas id=\"{CANVAS}\"> on the page").into());
                return;
            };
            let runner = eframe::WebRunner::new();
            let creation = Box::new(build);
            if let Err(e) = runner.start(canvas, options, creation).await {
                web_sys::console::error_1(&format!("BIMCraft não pôde iniciar: {e:?}").into());
            }
        });
    }

    /// A janela: um projeto novo na língua do sistema, na página inicial.
    fn build(cc: &CreationContext<'_>) -> Result<Box<dyn eframe::App>, Failure> {
        let language = lang::from_env();
        let mut session = Session::new();
        let _ = session.execute("new", &json!({ "language": language.project().code() }));
        let mut app = BimApp::new(session);
        app.lang = language;
        app.show_start_page();
        if let Some(state) = cc.wgpu_render_state.as_ref() {
            app.use_graphics_card(state);
        }
        Ok(Box::new(app))
    }
}

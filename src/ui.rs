use {egui_miniquad as egui_mq, miniquad::*};

pub struct Backend<A>
where
    A: Fn(&mut dyn RenderingBackend, &egui::Context) + Clone,
{
    egui_mq: egui_mq::EguiMq,
    mq_ctx: Box<dyn RenderingBackend>,
    app: A,
}

impl<A> Backend<A>
where
    A: Fn(&mut dyn RenderingBackend, &egui::Context) + Clone,
{
    pub fn new(app: A) -> Self {
        let mut mq_ctx = window::new_rendering_backend();

        Self {
            egui_mq: egui_mq::EguiMq::new(&mut *mq_ctx),
            mq_ctx,
            app,
        }
    }
}

impl<A> EventHandler for Backend<A>
where
    A: Fn(&mut dyn RenderingBackend, &egui::Context) + Clone,
{
    fn update(&mut self) {}

    fn draw(&mut self) {
        if self.egui_mq.egui_ctx().style().visuals.dark_mode {
            self.mq_ctx.clear(Some((0., 0., 0., 1.)), None, None);
        } else {
            self.mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        }

        // Run the UI code:
        self.egui_mq.run(&mut *self.mq_ctx, self.app.clone());

        // Draw things behind egui here

        self.egui_mq.draw(&mut *self.mq_ctx);

        // Draw things in front of egui here

        self.mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, _repeat: bool) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if keycode == KeyCode::Escape {
                std::process::exit(0);
            }
        }

        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

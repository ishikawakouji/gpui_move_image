use gpui::{
    div, img, prelude::*, App, AppContext, ImageSource, Render, ViewContext, WindowOptions,
    px, MouseDownEvent, Pixels, Point, MouseUpEvent, MouseMoveEvent,
};
use std::path::PathBuf;

struct GifViewer {
    gif_path: PathBuf,
    l: Pixels,
    t: Pixels,
    is_moving: bool,
    last_position: Point<Pixels>,
}

impl GifViewer {
    fn new(gif_path: PathBuf) -> Self {
        let l = px(0.0);
        let t = px(0.0);
        let is_moving = false;
        let last_position = Point {x:px(0.0), y:px(0.0)};
        let img_viewer = Self {gif_path, l, t, is_moving, last_position};
        img_viewer
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, _cx: &mut ViewContext<Self>) {
        self.is_moving = true;
        self.last_position = event.position;
    }

    fn on_mouse_up(&mut self, _event: &MouseUpEvent, _cx: &mut ViewContext<Self>) {
        self.is_moving = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _cx: &mut ViewContext<Self>) {
        if self.is_moving {
            let d_position = event.position - self.last_position;
            self.l += d_position.x;
            self.t += d_position.y;
            self.last_position=event.position;
        }
    }
}

impl Render for GifViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
        .on_mouse_down(gpui::MouseButton::Left, cx.listener(Self::on_mouse_down))
        .on_mouse_up(gpui::MouseButton::Left, cx.listener(Self::on_mouse_up))
        .on_mouse_move(cx.listener(Self::on_mouse_move))
        .size_full().child(
            img(ImageSource::File(self.gif_path.clone().into()))
                .size_full()
                .object_fit(gpui::ObjectFit::Contain)
                .id("gif")
                .w(px(256.0))
                .h(px(256.0))
                .ml(self.l)
                .mt(self.t),
        )
    }
}

fn main() {
    // env_logger::init();
    App::new().run(|cx: &mut AppContext| {
        let cwd = std::env::current_dir().expect("Failed to get current working directory");
        // let gif_path = cwd.join("crates/gpui/examples/image/black-cat-typing.gif");
        let gif_path = cwd.join("black-cat-typing.gif");

        if !gif_path.exists() {
            eprintln!("Image file not found at {:?}", gif_path);
            eprintln!("Make sure you're running this example from the root of the gpui crate");
            cx.quit();
            return;
        }

        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| GifViewer::new(gif_path)),
        )
        .unwrap();
        cx.activate(true);
    });
}

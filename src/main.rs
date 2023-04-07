use doryen_rs::{App, AppOptions, DoryenApi, Engine };

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

const LIMIT_FPS: usize = 20;

struct MyRoguelike {
}

impl Engine for MyRoguelike {
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let _con = api.con();
    }
}


fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "my roguelike".to_owned(),
        font_path: "font.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
        intercept_close_request: false,
        max_fps: LIMIT_FPS,
    });
    app.set_engine(Box::new(MyRoguelike {}));
    app.run();
}
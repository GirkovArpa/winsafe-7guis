use winsafe::gui;
use winsafe::WinResult;

static mut DECASECONDS: i32 = 0;
static mut SLIDER_POS: i32 = 0;

#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain,
    bar: gui::ProgressBar,
    btn: gui::Button,
    lbl: gui::Label,
}

impl MyWindow {
    pub fn new() -> MyWindow {
        let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);
        let bar = gui::ProgressBar::new_dlg(&wnd, 1);
        let btn = gui::Button::new_dlg(&wnd, 5);
        let lbl = gui::Label::new_dlg(&wnd, 2);
        let new_self = Self { wnd, btn, bar, lbl };
        new_self.events();
        new_self
    }

    pub fn run(&self) -> WinResult<()> {
        self.wnd.run_main(None)
    }

    fn events(&self) {
        // https://stackoverflow.com/a/56390740/13378247
        self.wnd.on().wm(winsafe::co::WM::HSCROLL, {
            move |parms| {
                let loword = winsafe::LOWORD(parms.wparam as u32);
                const TB_THUMBPOSITION: u16 = 5;
                let hiword = winsafe::HIWORD(parms.wparam as u32);
                if loword != TB_THUMBPOSITION {
                    return 0;
                }
                unsafe {
                    SLIDER_POS = hiword as i32;
                }
                0
            }
        });

        self.wnd.on().wm_init_dialog({
            let myself = self.clone();
            move |_| {
                myself.wnd.hwnd().SetTimer(1, 100, None).unwrap();
                true
            }
        });

        self.wnd.on().wm_timer(1, {
            let myself = self.clone();
            move || {
                update(true, &myself);
            }
        });

        self.btn.on().bn_clicked({
            let myself = self.clone();
            move || {
                reset(&myself);
            }
        });
    }
}

fn update(increment: bool, my_window: &MyWindow) -> () {
    let a: f32 = 0.;
    let b = unsafe { SLIDER_POS as f32 };
    let y: f32 = 0.;
    let z: f32 = 100.;
    unsafe {
        if increment && ((DECASECONDS / 10) as f32) < b {
            DECASECONDS += 1;
        }
        let seconds = ((DECASECONDS as f32) / 10.0) as f32;
        let mut percentage = map_range(seconds, a, b, y, z) as u32;
        if percentage > 100 {
            percentage = 100;
        }
        my_window.bar.set_position(percentage);
        my_window
            .lbl
            .set_text(&format!("{:.1}s / {}s", seconds, b).replace(".", ","))
            .unwrap();
    }
}

fn reset(my_window: &MyWindow) -> () {
    unsafe {
        DECASECONDS = 0;
        update(false, my_window);
    }
}

fn map_range(n: f32, a: f32, b: f32, y: f32, z: f32) -> f32 {
    return ((n - a) * (z - y) / (b - a) + y).round();
}

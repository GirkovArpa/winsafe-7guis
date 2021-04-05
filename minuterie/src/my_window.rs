use winsafe::gui;
use winsafe::WinResult;

#[derive(Clone)]
pub struct MyWindow {
  wnd: gui::WindowMain,
  btn: gui::Button,
  prg: gui::ProgressCTL1
}

impl MyWindow {
  pub fn new() -> MyWindow {
    let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);
    let btn = gui::Button::new_dlg(&wnd, 5);
    let prg = gui::Progress::new_dlg(&wnd, 1);
    let new_self = Self { wnd, btn, prg };
    new_self.events();
    new_self
  }

  pub fn run(&self) -> WinResult<()> {
    self.wnd.run_main(None)
  }

  fn events(&self) {
    self.btn.on().bn_clicked({
      let myself = self.clone();
      move || {
        
      }
    });
  }
}
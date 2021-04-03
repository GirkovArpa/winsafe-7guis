use winsafe::gui;
use winsafe::WinResult;

#[derive(Clone)]
pub struct MyWindow {
  wnd: gui::WindowMain,
  btn: gui::Button,
  cmb: gui::ComboBox
}

impl MyWindow {
  pub fn new() -> MyWindow {
    let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);
    let btn = gui::Button::new_dlg(&wnd, 2);
    let cmb = gui::ComboBox::new_dlg(&wnd, 3);
    
    let myself = Self { wnd, cmb, btn };
    myself.events();
    myself
  }

  pub fn run(&self) -> WinResult<()> {
    self.wnd.run_main(None)
  }

  fn events(&self) {
    self.wnd.on().wm_init_dialog({
      let myself = self.clone();
        move |_| {
          myself.cmb.items().add(&["one-way flight", "return flight"]).unwrap();
          let i: Option<u32> = Some(0);
          myself.cmb.items().set_selected(i);
          true
        }
    });

  }

}

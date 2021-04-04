use std::cell::RefCell;
use std::rc::Rc;

use winsafe::gui;

#[derive(Clone)]
pub struct MyModal {
  wnd: gui::WindowModal,
  lbl: gui::Label,
  btn: gui::Button,
  input_val: Rc<RefCell<String>>,
}

impl MyModal {
  pub fn new(parent: &dyn gui::Parent, input_text: &str) -> MyModal {
    let wnd = gui::WindowModal::new_dlg(parent, 1001);

    let lbl = gui::Label::new_dlg(&wnd, 0);
    let btn = gui::Button::new_dlg(&wnd, 1);

    let new_self = Self {
      wnd,
      lbl,
      btn,
      input_val: Rc::new(RefCell::new(String::from(input_text))),
    };

    new_self.events();
    new_self
  }

  pub fn show(&self) -> () {
    self.wnd.show_modal().unwrap();
  }

  fn events(&self) {
    self.wnd.on().wm_init_dialog({
      let myself = self.clone();
      move |_| {
        myself.lbl.set_text(&myself.input_val.borrow()).unwrap();
        true
      }
    });

    self.btn.on().bn_clicked({
      let myself = self.clone();
      move || {
        myself.wnd.hwnd().EndDialog(0).unwrap();
      }
    });

  }
}
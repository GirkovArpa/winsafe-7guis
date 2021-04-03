use winsafe::gui;
use winsafe::WinResult;

#[derive(Clone)]
pub struct MyWindow {
  wnd: gui::WindowMain,
  btn: gui::Button,
  edit: gui::Edit,
}

impl MyWindow {
  pub fn new() -> MyWindow {
    let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);
    let btn = gui::Button::new_dlg(&wnd, 1);
    let edit = gui::Edit::new_dlg(&wnd, 0);
    let new_self = Self { wnd, btn, edit };
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
        let chaine: String = myself.edit.text().unwrap();
        let numero: i32 = chaine.parse::<i32>().unwrap() + 1;
        let resultat = format!("{}", numero);
        myself.edit.set_text(&resultat).unwrap();
      }
    });
  }
}

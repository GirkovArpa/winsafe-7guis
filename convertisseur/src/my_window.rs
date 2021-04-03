use winsafe::gui;
use winsafe::WinResult;

static mut SEMAPHORE: bool = false;

#[derive(Clone)]
pub struct MyWindow {
  wnd: gui::WindowMain,
  c: gui::Edit,
  f: gui::Edit,
}

impl MyWindow {
  pub fn new() -> MyWindow {
    let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);
    let c = gui::Edit::new_dlg(&wnd, 0);
    let f = gui::Edit::new_dlg(&wnd, 1);
    let new_self = Self { wnd, c, f };
    new_self.events();
    new_self
  }

  pub fn run(&self) -> WinResult<()> {
    self.wnd.run_main(None)
  }

  fn events(&self) {
    self.c.on().en_change({
      let moi_meme = self.clone();
      move || {
        unsafe {
          if SEMAPHORE {
            SEMAPHORE = false;
            return;
          } else {
            SEMAPHORE = true;
          }
        }
        let chaine_c: String = moi_meme.c.text().unwrap();
        
        let numero_c = chaine_c.parse::<f64>();
        let f_nouveau = match numero_c {
          Ok(c) => c2f(c),
          Err(_) => return
        };

        let chaine_f_nouvelle = format!("{}", f_nouveau);
        moi_meme.f.set_text(&chaine_f_nouvelle).unwrap();
      }
    });

    self.f.on().en_change({
      let moi_meme = self.clone();
      move || {
        unsafe {
          if SEMAPHORE {
            SEMAPHORE = false;
            return;
          } else {
            SEMAPHORE = true;
          }
        }
        let chaine_f: String = moi_meme.f.text().unwrap();
        
        let numero_f = chaine_f.parse::<f64>();
        let c_nouveau = match numero_f {
          Ok(f) => f2c(f),
          Err(_) => return
        };

        let chaine_c_nouvelle = format!("{}", c_nouveau);
        moi_meme.c.set_text(&chaine_c_nouvelle).unwrap();
      }
    });
  }
}

fn c2f(c: f64) -> f64 {
  return c * 9. / 5. + 32.;
}

fn f2c(f: f64) -> f64 {
  return (f - 32.) * 5. / 9.;
}
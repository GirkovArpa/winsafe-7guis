use winsafe::gui;
use winsafe::WinResult;

use chrono::{NaiveDate, Datelike};

mod my_modal;

#[derive(Clone)]
pub struct MyWindow {
  wnd: gui::WindowMain,
  btn: gui::Button,
  cmb: gui::ComboBox,
  one: gui::Edit,
  ret: gui::Edit
}

impl MyWindow {
  pub fn new() -> MyWindow {
    let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);
    let cmb = gui::ComboBox::new_dlg(&wnd, 0);
    let one = gui::Edit::new_dlg(&wnd, 1);
    let ret = gui::Edit::new_dlg(&wnd, 2);
    let btn = gui::Button::new_dlg(&wnd, 3);
    
    let myself = Self { wnd, cmb, btn, one, ret };
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

    self.btn.on().bn_clicked({
      let myself = self.clone();
      move || {
        let selection = myself.cmb.items().selected_text().unwrap();
        let date_string_one = myself.one.text().unwrap();
        let input_text = if selection == "one-way flight" {
          format!("You've booked a one-way flight on {}.", date_string_one)
        } else {
          let date_string_ret = myself.ret.text().unwrap();
          format!("You've booked a return flight from {} to {}.", date_string_one, date_string_ret)
        };
        let my_modal = my_modal::MyModal::new(&myself.wnd, &input_text);
        my_modal.show();
      }
    });

    self.cmb.on().cbn_sel_change({
      let wnd = self.clone();

      move || {
        let selection = wnd.cmb.items().selected_text().unwrap();
        wnd.ret.hwnd().EnableWindow(selection != "one-way flight");

        wnd.btn.hwnd().EnableWindow(validate(wnd.to_owned()));
      }
    });

    self.one.on().en_change({
      let wnd = self.clone();
      move || {
        wnd.btn.hwnd().EnableWindow(validate(wnd.to_owned()));
      }
    });

    self.ret.on().en_change({
      let wnd = self.clone();
      move || {
        wnd.btn.hwnd().EnableWindow(validate(wnd.to_owned()));
      }
    });

  }

}

fn validate(wnd: MyWindow) -> bool {
    let cmb = wnd.cmb;
    let one = wnd.one;
    let ret = wnd.ret;

    // if either date is invalid, return false
    let date_string_one = one.text().unwrap();
    let date_struct_one = match NaiveDate::parse_from_str(&date_string_one, "%d.%m.%Y") {
      Ok(val) => val,
      Err(_) => return false
    };
    let date_days_one = date_struct_one.num_days_from_ce();

    // validation of second edit if return flight is selected
    let selection = cmb.items().selected_text().unwrap();
    if selection == "return flight" {
      let date_string_ret = ret.text().unwrap();
      let date_struct_ret = match NaiveDate::parse_from_str(&date_string_ret, "%d.%m.%Y") {
        Ok(val) => val,
        Err(_) => return false
      };
      let date_days_ret = date_struct_ret.num_days_from_ce();

      // if start date is after return date ;)
      if date_days_one > date_days_ret {
        return false;
      }
    }

    true
}
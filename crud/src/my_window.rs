use winsafe::gui;
use winsafe::WinResult;

#[derive(Clone)]
pub struct MyWindow {
  wnd: gui::WindowMain,
  lst: gui::ListBox,
  crt: gui::Button,
  upd: gui::Button,
  del: gui::Button,
  frst: gui::Edit,
  last: gui::Edit,
}

impl MyWindow {
  pub fn new() -> MyWindow {
    let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);
    let lst = gui::ListBox::new_dlg(&wnd, 2);
    let crt = gui::Button::new_dlg(&wnd, 7);
    let upd = gui::Button::new_dlg(&wnd, 8);
    let del = gui::Button::new_dlg(&wnd, 9);
    let frst = gui::Edit::new_dlg(&wnd, 5);
    let last = gui::Edit::new_dlg(&wnd, 6);
    let new_self = Self { wnd, lst, crt, upd, del, frst, last };
    new_self.events();
    new_self
  }

  pub fn run(&self) -> WinResult<()> {
    self.wnd.run_main(None)
  }

  fn events(&self) {
    self.wnd.on().wm_init_dialog({
      let myself = self.clone();
      move |_| {
        myself.lst.items().add(&[
          "Emil, Hans", 
          "Mustermann, Max", 
          "Tisch, Roman"
        ]);

        myself.lst.hwnd().SendMessage({
          winsafe::msg::lb::SelectString { 
            index: Some(0),
            prefix: "Tisch, Roman"
          }
        }).unwrap();

        true
      }
    });
   
    self.crt.on().bn_clicked({
      let myself = self.clone();
      move || {
        let frst: String = myself.frst.text().unwrap();
        let last: String = myself.last.text().unwrap();
        let name = format!("{}, {}", last, frst);
        myself.lst.items().add(&[&name]);
      }
    });

    self.upd.on().bn_clicked({
      let myself = self.clone();
      move || {
        let lst_hwnd = myself.lst.hwnd();

        let list_box_item = get_selected_item(lst_hwnd).unwrap();

        let index = list_box_item.index;
        lst_hwnd.SendMessage({
          winsafe::msg::lb::DeleteString { index }
        }).unwrap();

        let frst: String = myself.frst.text().unwrap();
        let last: String = myself.last.text().unwrap();
        let name = format!("{}, {}", last, frst);
        myself.lst.items().add(&[&name]);

        lst_hwnd.SendMessage({
          winsafe::msg::lb::SelectString { 
            index: Some(index),
            prefix: &name
          }
        }).unwrap();

      }
    });

    self.del.on().bn_clicked({
      let myself = self.clone();
      move || {
        let lst_hwnd = myself.lst.hwnd();

        let index =  get_selected_item(lst_hwnd).unwrap().index;

        lst_hwnd.SendMessage({
          winsafe::msg::lb::DeleteString { 
            index
          }
        }).unwrap();

        myself.upd.hwnd().EnableWindow(false);
        myself.del.hwnd().EnableWindow(false);
      }
    });

    self.lst.on().lbn_sel_change({
      let myself = self.clone();
      move || {
        let lst_hwnd = myself.lst.hwnd();

        let list_box_item = match get_selected_item(lst_hwnd) {
          Some(list_box_item) => list_box_item,
          None => return
        };

        myself.upd.hwnd().EnableWindow(true);
        myself.del.hwnd().EnableWindow(true);

        myself.frst.set_text(&list_box_item.name.first).unwrap();
        myself.last.set_text(&list_box_item.name.last).unwrap();
      }
    });
  }
}

struct ListBoxItem {
  index: u32,
  name: FullName
}

struct FullName {
  first: String,
  last: String
}

fn get_selected_item(lst_hwnd: winsafe::HWND) -> Option<ListBoxItem> {
  let index = match lst_hwnd.SendMessage({
      winsafe::msg::lb::GetCurSel {}
  }) {
    Some(index) => index,
    None => return None,
  };

  let len = lst_hwnd.SendMessage({
    winsafe::msg::lb::GetTextLen { index }
  }).unwrap();

  let mut text = winsafe::WString::new_alloc_buffer(len as usize + 1);
  lst_hwnd.SendMessage({
      winsafe::msg::lb::GetText { index, text: &mut text }
  }).unwrap();

  let fullname = text.to_string();
  let mut names = fullname.split(", ");

  let name = FullName { 
    last: String::from(names.next().unwrap()), 
    first: String::from(names.next().unwrap()) 
  };
  
  Some(ListBoxItem { index, name })
}
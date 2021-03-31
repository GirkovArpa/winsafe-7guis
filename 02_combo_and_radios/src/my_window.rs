use winsafe::gui;
use winsafe::{HINSTANCE, IdIdiStr, POINT, SIZE, WinResult};

#[derive(Clone)]
pub struct MyWindow {
	wnd:        gui::WindowMain,
	cmb_cities: gui::ComboBox,
	rad_seas:   gui::RadioGroup,
}

impl MyWindow {
	pub fn new() -> MyWindow {
		let hinstance = HINSTANCE::GetModuleHandle(None).unwrap();

		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "Combo and radios".to_owned(),
				class_icon: hinstance.LoadIcon(IdIdiStr::Id(101)).unwrap(),
				size: SIZE::new(300, 150),
				..Default::default()
			},
		);

		let cmb_cities = gui::ComboBox::new(
			&wnd,
			gui::ComboBoxOpts {
				position: POINT::new(20, 10),
				width: 140,
				..Default::default()
			},
		);

		let rad_seas = gui::RadioGroup::new(
			&wnd, &[
				gui::RadioButtonOpts {
					text: "Mediterranean".to_owned(),
					position: POINT::new(20, 50),
					..Default::default()
				},
				gui::RadioButtonOpts {
					text: "Caribbean".into(),
					position: POINT::new(20, 70),
					..Default::default()
				},
				gui::RadioButtonOpts {
					text: "Adriatic".into(),
					position: POINT::new(20, 90),
					..Default::default()
				},
			],
		);

		let new_self = Self { wnd, cmb_cities, rad_seas };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		self.wnd.on().wm_create({ // happens once, right after the window is created
			let self2 = self.clone();
			move |_| {
				self2.cmb_cities.items().add(&["Paris", "Madrid", "Lisbon", "Rome"])
					.unwrap();

				self2.rad_seas[1].set_check(true); // second radio initially selected

				0
			}
		});

		self.cmb_cities.on().cbn_sel_change({ // combo item is selected
			let self2 = self.clone();
			move || {
				let the_city = self2.cmb_cities.items().selected_text().unwrap();
				self2.wnd.hwnd().SetWindowText(&the_city).unwrap()
			}
		});

		self.rad_seas.on().bn_clicked({ // radio item is selected
			let self2 = self.clone();
			move || {
				let selected_radio = self2.rad_seas.checked().unwrap();
				let the_sea = selected_radio.hwnd().GetWindowTextStr().unwrap();
				self2.wnd.hwnd().SetWindowText(&the_sea).unwrap();
			}
		});
	}
}

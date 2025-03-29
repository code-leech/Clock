/* window.rs
 *
 * Copyright 2025 CodeLeech
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};


mod imp {
	use super::*;

	#[derive(Debug, Default, gtk::CompositeTemplate)]
	#[template(resource = "/code/leech/rsclock/window.ui")]
	pub struct RsclockWindow {
		#[template_child]
		pub group: TemplateChild<adw::ToggleGroup>,
		#[template_child]
		pub twelve_hour: TemplateChild<adw::Toggle>,
		#[template_child]
		pub twentyfour_hour: TemplateChild<adw::Toggle>,
		#[template_child]
		pub clock_label: TemplateChild<gtk::Label>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for RsclockWindow {
		const NAME: &'static str = "RsclockWindow";
		type Type = super::RsclockWindow;
		type ParentType = adw::ApplicationWindow;

		fn class_init(klass: &mut Self::Class) {
			klass.bind_template();
		}

		fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
			obj.init_template();
		}
	}

		impl ObjectImpl for RsclockWindow {
			fn constructed(&self) {
				self.parent_constructed();
				let obj = self.obj().clone();

				glib::MainContext::default().spawn_local(glib::clone!(#[weak] obj, async move {
				use chrono::Timelike;
				let mut last_second = None;
				let mut last_format = None;

				loop {
					let now = chrono::Local::now();
					let current_second = now.second();
					let is_24_hour = obj.imp().group.active() == 1;

					match (last_second, last_format) {
						(Some(sec), Some(fmt)) if sec == current_second && fmt == is_24_hour => {}
						_ => {
							last_second = Some(current_second);
							last_format = Some(is_24_hour);
							let time_str = match is_24_hour {
								true => now.format("%H:%M:%S").to_string(),
								false => now.format("%I:%M:%S %p").to_string(),
							};
							obj.imp().clock_label.set_label(&time_str);
						}
					}

					glib::timeout_future(std::time::Duration::from_millis(100)).await;
				}
		}));
	}

	}

	impl WidgetImpl for RsclockWindow {}
	impl WindowImpl for RsclockWindow {}
	impl ApplicationWindowImpl for RsclockWindow {}
	impl AdwApplicationWindowImpl for RsclockWindow {}
}

glib::wrapper! {
	pub struct RsclockWindow(ObjectSubclass<imp::RsclockWindow>)
		@extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
		@implements gio::ActionGroup, gio::ActionMap;
}

impl RsclockWindow {
	pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
		glib::Object::builder()
			.property("application", application)
			.build()
	}
}

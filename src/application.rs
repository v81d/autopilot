/* application.rs
 *
 * Copyright 2026 v81d
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

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::AutopilotWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct AutopilotApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for AutopilotApplication {
        const NAME: &'static str = "AutopilotApplication";
        type Type = super::AutopilotApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for AutopilotApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
        }
    }

    impl ApplicationImpl for AutopilotApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = AutopilotWindow::new(&*application);
                window.upcast()
            });

            // Perform post-construction actions
            if let Some(window_) = window.downcast_ref::<AutopilotWindow>() {
                // TODO: call some more functions here or whatever
                window_.attach_callbacks();
            }

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for AutopilotApplication {}
    impl AdwApplicationImpl for AutopilotApplication {}
}

glib::wrapper! {
    pub struct AutopilotApplication(ObjectSubclass<imp::AutopilotApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AutopilotApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/io/github/v81d/Autopilot")
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("Autopilot")
            .application_icon("io.github.v81d.Autopilot")
            .developer_name("v81d")
            .version(VERSION)
            .developers(vec!["v81d"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(gettext("translator-credits"))
            .copyright("© 2026 v81d")
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/v81d/autopilot")
            .build();

        about.present(Some(&window));
    }
}

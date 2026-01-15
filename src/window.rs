/* window.rs
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
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/v81d/Autopilot/window.ui")]
    pub struct AutopilotWindow {
        // Template widgets
        #[template_child]
        pub main_split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub toggle_sidebar_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub profile_list_box: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub new_profile_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub get_started_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AutopilotWindow {
        const NAME: &'static str = "AutopilotWindow";
        type Type = super::AutopilotWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AutopilotWindow {}
    impl WidgetImpl for AutopilotWindow {}
    impl WindowImpl for AutopilotWindow {}
    impl ApplicationWindowImpl for AutopilotWindow {}
    impl AdwApplicationWindowImpl for AutopilotWindow {}
}

glib::wrapper! {
    pub struct AutopilotWindow(ObjectSubclass<imp::AutopilotWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl AutopilotWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn attach_callbacks(&self) {
        let imp = self.imp();

        // Attach callback for New Profile button
        imp.new_profile_button.connect_clicked(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_| this.add_new_profile()
        ));

        // Attach callback for Get Started button
        imp.get_started_button.connect_clicked(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_| this.add_new_profile()
        ));

        // Attach callback for Toggle Sidebar button
        imp.toggle_sidebar_button.connect_toggled(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_| this.toggle_sidebar()
        ));
    }

    pub fn toggle_sidebar(&self) {
        let imp = self.imp();
        let sidebar_toggled = imp.toggle_sidebar_button.is_active();
        imp.main_split_view.set_show_sidebar(sidebar_toggled);
    }

    pub fn append_sidebar_profile(&self, name: &str, description: &str) {
        // TODO: use a dedicated struct for macros instead of this

        let imp = self.imp();

        let row = gtk::ListBoxRow::new(); // create encapsulating row
        let action_row = adw::ActionRow::builder()
            .title(name)
            .subtitle(description)
            .build();

        row.set_child(Some(&action_row));
        imp.profile_list_box.append(&row);
    }

    pub fn add_new_profile(&self) {
        // TODO: complete this callback
        self.append_sidebar_profile("New Profile", "A brief description of the profile.")
    }
}

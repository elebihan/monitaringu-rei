//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//


use gtk::{self, prelude::*};

pub fn create() -> gtk::HeaderBar {
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_show_close_button(true);
    header_bar.set_title(Some("Monitaringu Rei"));

    let builder = gtk::Builder::from_resource("/com/elebihan/monitaringu-rei-gtk/gtk/menus.ui");
    let menu = builder
        .object::<gio::MenuModel>("application-menu")
        .expect("Can not find menu");

    let menu_button = gtk::MenuButton::new();
    let menu_img = gtk::Image::from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::Menu);
    menu_button.set_image(Some(&menu_img));
    menu_button.set_menu_model(Some(&menu));

    header_bar.pack_end(&menu_button);

    header_bar
}

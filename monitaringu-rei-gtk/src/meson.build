#
# This file is part of monitaringu-rei-gtk
#
# Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
#
# SPDX-License-Identifier: MIT
#

pkginfo_rs = configure_file(input : 'pkginfo.rs.in',
                            output : 'pkginfo.rs',
                            configuration : conf)

run_command('cp', pkginfo_rs, meson.current_source_dir(), check : true)

monitaringu_rei_gtk_sources = files(
  'app.rs',
  'dialogs.rs',
  'error.rs',
  'header_bar.rs',
  'i18n.rs',
  'lib.rs',
  'main.rs',
  'resources.rs',
  'supervisor.rs',
  'window.rs',
  'xdg.rs',
)

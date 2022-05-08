#
# This file is part of monitaringu-rei-gtk
#
# Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
#
# SPDX-License-Identifier: MIT
#

subdir('data')
subdir('src')
subdir('po')

program = 'monitaringu-rei-gtk'

if cargo_target.contains('windows')
  program += '.exe'
endif

monitaringu_rei_gtk = custom_target('monitaringu-rei-gtk',
                                    build_by_default: true,
                                    console: true,
                                    input: [
                                      monitaringu_rei_build_files,
                                      monitaringu_rei_gtk_sources,
                                    ],
                                    output: [program],
                                    install: true,
                                    install_dir: bindir,
                                    command: [
                                      cargo_wrapper,
                                      meson.current_source_dir(),
                                      meson.project_build_root(),
                                      '@OUTPUT@',
                                      cargo_mode,
                                      cargo_target,
                                    ])

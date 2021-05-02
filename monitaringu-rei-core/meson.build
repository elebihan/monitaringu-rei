#
# This file is part of monitaringu-rei
#
# Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
#
# SPDX-License-Identifier: MIT
#

subdir('src')

monitaringu_rei = custom_target('monitaringu-rei',
                                build_by_default: true,
                                console: true,
                                input: [
                                  monitaringu_rei_build_files,
                                  monitaringu_rei_core_sources,
                                ],
                                output: ['monitaringu-rei'],
                                install: true,
                                install_dir: bindir,
                                command: [
                                  cargo_wrapper,
                                  meson.current_source_dir(),
                                  meson.build_root(),
                                  '@OUTPUT@',
                                  'monitaringu-rei',
                                  cargo_mode,
                                ])

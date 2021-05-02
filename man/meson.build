#
# This file is part of monitaringu-rei
#
# Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
#
# SPDX-License-Identifier: MIT
#

custom_target('monitaringu-rei.1',
              output : 'monitaringu-rei.1',
              input : 'monitaringu-rei.1.md',
              command : [pandoc,
                         '--output', '@OUTPUT@',
                         '--standalone',
                         '--from', 'markdown',
                         '--to', 'man',
                         '@INPUT@'],
              install : true,
              install_dir : join_paths(get_option('mandir'), 'man1'))

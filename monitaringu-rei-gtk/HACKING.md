# Hacking on Monitaringu Rei (GTK)

## Translation

To generate the template:

```sh
xtr src/*.rs -o po/code.pot
xgettext --from-code=UTF-8 -L Glade data/gtk/*.ui -o po/ui.pot
msgcat po/code.pot po/ui.pot > po/monitaringu-rei-gtk.pot
```

To create a translation (e.g. french):

```sh
msginit --input po/monitaringu-rei-gtk.pot --output po/fr.pot --locale fr
```

To update the french translation:

```sh
msgmerge --update po/fr.po po/monitaringu-rei-gtk.pot
```


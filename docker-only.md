# Install a dekstop icon 

Follow these simple steps:
1. `git clone git@github.com:Cyber-Mint/pgmodeler-x11.git`
1. `cd pgmodeler-x11`
1. `mkdir -p ~/.local/share/pgmodeler`
1. `cp launcher/resources/pgmodeler-x11 ~/.local/share/pgmodeler/.`
1. `cp launcher/resources/pgmodeler_logo.png ~/.local/share/icons/.`
1. `chmod +x /usr/share/pgmodeler/pgmodeler-x11`
1. `cp launcher/resources/pgmodeler-docker.desktop ~/.local/share/applications/pgmodeler-x11.desktop`
1. `chmod +x ~/.local/share/applications/pgmodeler-x11.desktop`
1. `docker pull cybermint/pgmodeler`

and possibly `gio set .local/share/applications/pgmodeler-x11.desktop "metadata::trusted" TRUE`

And you are good to go ...open the pgmodeler using your launcher.

---

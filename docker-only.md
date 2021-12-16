# Install a dekstop icon 

Follow these simple steps:
1. `git clone git@github.com:Cyber-Mint/pgmodeler-x11.git`
1. `cd pgmodeler-x11`
1. `sudo mkdir -p /usr/share/pgmodeler`
1. `sudo cp launcher/resources/pgmodeler_logo.png /usr/share/pgmodeler/.`
1. `sudo cp docker-compose.yml /usr/share/pgmodeler/.`
1. Now you will need to make a few small changes to your `~/.bashrc` as follows:
    ```
    echo "export USERID=$(id -u)" >> ~/.bashrc
    echo "export WORKING_DIR=/home/$USER" >> ~/.bashrc
    ```
7. `sudo cp launcher/resources/pgmodeler-docker.desktop usr/share/applications/pgmodeler-docker-x11.desktop`
8. `docker pull cybermint/pgmodeler`

And you are good to go ...open the pgmodeler using your launcher.

---

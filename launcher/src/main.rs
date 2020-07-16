extern crate gtk;
extern crate gio;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use std::env::var;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::thread;

fn show_error(error: &str, application: &gtk::Application) {
    let error_message = include_str!("error_message.glade");
    let builder = gtk::Builder::new_from_string(error_message);

    let dialog: gtk::MessageDialog = builder.get_object("msgMain").expect("Could not get object");
    dialog.set_property_text(error);
    dialog.set_application(application);
    dialog.run();
}

#[cfg(target_os = "linux")]
fn x11_running(application: &gtk::Application, _ran_already: bool) -> Result<String, ()> {
    let display_output = Command::new("sh")
        .arg("-c")
        .arg("echo $DISPLAY")
        .output();
    match display_output {
        Err(_e) => {
            show_error("Could not get $DISPLAY variable", application);
            Err(())
        },
        Ok(output) => {
            let display = String::from_utf8(output.stdout).expect("Could not parse output");
            let display = display.as_str().trim();
            if display.len() == 0 {
                Err(())
            } else {
                Ok(display.to_string())
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn get_ip(application: &gtk::Application) -> Result<String, ()> {
    let ip = (0..3)
        .map(|x| {
            let nic = format!("en{}", x);
            let nic = nic.as_str();
            let ip = Command::new("ipconfig")
                .arg("getifaddr")
                .arg(nic)
                .output();
            ip
        })
        .filter(|ip| ip.is_ok())
        .map(|ip| ip.unwrap())
        .map(|ip| ip.stdout)
        .map(|stdout| String::from_utf8(stdout).expect("Could not parse output"))
        .find(|ip| ip.len() > 0);

    if ip.is_none() {
        show_error("Could not get an IP", application);
        Err(())
    } else {
        Ok(ip.unwrap())
    }
}

#[cfg(target_os = "macos")]
fn x11_running(application: &gtk::Application, ran_already: bool) -> Result<String, ()> {
    // First check if XQuartz is running
    let xquartz_running = Command::new("sh")
        .arg("-c")
        .arg("launchctl list | grep quartz")
        .output();

    if (xquartz_running.is_err()) {
        show_error("Could not determine if XQuartz was running", application);
        return Err(());
    }
    let xquartz_running = xquartz_running.unwrap();
    let xquartz_running = String::from_utf8(xquartz_running.stdout).expect("Could not parse output");

    // If it is not running
    if xquartz_running.len() == 0 {
        if ran_already {
            show_error("Timeout waiting for XQuartz", application);
            Err(())
        } else {
            println!("XQuartz is not running, start it");
            let launch_xquartz = Command::new("open")
                .arg("-a")
                .arg("XQuartz")
                .output();

            if launch_xquartz.is_err() {
                show_error("Could not start XQuartz", application);
                Err(())
            } else {
                thread::sleep_ms(2000);
                println!("Not sure if we'll be able to get the $DISPLAY variable on an already started app..");
                return x11_running(application, true);
            }
        }
    } else {
        // Otherwise $DISPLAY = IP address
        get_ip(application)
    }
}

#[cfg(target_os = "windows")]
fn x11_running(_application: &gtk::Application, _ran_already: bool) -> Result<String, ()> {
    show_error("Windows only goes this far.", application);
    Err(())
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn docker_version() -> Result<String, ()> {
    let docker_output = Command::new("sh")
        .arg("-c")
        .arg("docker --version")
        .output();
    match docker_output {
        Err(_e) => Err(()),
        Ok(output) => {
            let docker_version = String::from_utf8(output.stdout).expect("Could not parse output");
            let docker_version = docker_version.as_str().trim();
            if docker_version.len() == 0 {
                Err(())
            } else {
                Ok(docker_version.to_string())
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn docker_version() -> Result<String, ()> {
    let docker_output = Command::new("cmd")
        .args(&["/C", "docker --version"])
        .output();
    match docker_output {
        Err(e) => Err(()),
        Ok(output) => {
            let docker_version = String::from_utf8(output.stdout).expect("Could not parse output")
                .as_str().trim();
            if (docker_version.len() == 0) {
                Err(())
            } else {
                Ok(display.to_string())
            }
        }
    }}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn user_id() -> Result<String, ()> {
    let id_output = Command::new("sh")
        .arg("-c")
        .arg("id -u")
        .output();
    match id_output {
        Err(_e) => Err(()),
        Ok(output) => {
            let user_id = String::from_utf8(output.stdout).expect("Could not parse output");
            let user_id = user_id.as_str().trim();
            if user_id.len() == 0 {
                Err(())
            } else {
                Ok(user_id.to_string())
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn user_name() -> Result<String, ()> {
    let user_output = Command::new("sh")
        .arg("-c")
        .arg("echo $USER")
        .output();
    match user_output {
        Err(_e) => Err(()),
        Ok(output) => {
            let user_name = String::from_utf8(output.stdout).expect("Could not parse output");
            let user_name = user_name.as_str().trim();
            if user_name.len() == 0 {
                Err(())
            } else {
                Ok(user_name.to_string())
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn run_pgmodeler(display_variable: String, user_id: String, user_name: String, application: &gtk::Application) {
    let pgmodeler_version = match var("PGMODELER_VERSION") {
        Ok(ver) => format!("{}", ver),
        Err(_e) => "v0.9.2-beta".to_string()
    };
    let pgmodeler_conf_dir = format!("/home/{}/.pgmodeler-docker-x11/{}", user_name, pgmodeler_version);
    let pgmodeler_conf_dir = pgmodeler_conf_dir.as_str();

    Command::new("mkdir")
        .arg("-p")
        .arg(pgmodeler_conf_dir)
        .output().unwrap();

    let pgmodeler_conf = format!("--volume={}:/home/{}/.pgmodeler", pgmodeler_conf_dir, user_name);
    let pgmodeler_conf = pgmodeler_conf.as_str();

    let display_variable = format!("DISPLAY=unix{}", display_variable);
    let display_variable = display_variable.as_str();

    let home_volume = format!("--volume=/home/{}:/home/{}", user_name, user_name);
    let home_volume = home_volume.as_str();

    let image_name = format!("cybermint/pgmodeler-x11:{}", pgmodeler_version);
    let image_name = image_name.as_str();

    let args = &[
        "run",
        "--rm",
        "--user", user_id.as_str(),
        "-e", display_variable, home_volume, "--volume=/etc/group:/etc/group:ro",
        "--volume=/etc/passwd:/etc/passwd:ro", "--volume=/etc/shadow:/etc/shadow:ro",
        "--volume=/etc/sudoers.d:/etc/sudoers.d:ro", "--volume=/tmp/.X11-unix:/tmp/.X11-unix",
        pgmodeler_conf,
        image_name];

    let mut child = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let out = BufReader::new(child.stdout.take().unwrap());
    let err = BufReader::new(child.stderr.take().unwrap());

    let thread = thread::spawn(move || {
        err.lines().for_each(|line| eprintln!("{}", line.unwrap()));
    });

    out.lines().for_each(|line| println!("{}", line.unwrap()));

    thread.join().unwrap();

    let status = child.wait().unwrap();
    if !status.success() {
        if status.code().unwrap() == 125 {
            let error_msg = format!("Could not find the Docker image {}", image_name);
            let error_msg = error_msg.as_str();
            show_error(error_msg, application);
        } else {
            let error_msg = format!("An error occurred {} {:?} {}", status.code().unwrap(), status, display_variable);
            let error_msg = error_msg.as_str();
            show_error(error_msg, application);
        }
    }
}

#[cfg(target_os = "macos")]
fn run_pgmodeler(display_variable: String, user_id: String, user_name: String, application: &gtk::Application) {
    let pgmodeler_version = match var("PGMODELER_VERSION") {
        Ok(ver) => format!("{}", ver),
        Err(_e) => "v0.9.2-beta".to_string()
    };
    let pgmodeler_conf_dir = format!("/Users/{}/.pgmodeler-docker-x11/{}", user_name, pgmodeler_version);
    let pgmodeler_conf_dir = pgmodeler_conf_dir.as_str();

    Command::new("mkdir")
        .arg("-p")
        .arg(pgmodeler_conf_dir)
        .output().unwrap();

    let pgmodeler_conf = format!("--volume={}:/home/{}/.pgmodeler", pgmodeler_conf_dir, user_name);
    let pgmodeler_conf = pgmodeler_conf.as_str();

    let display_variable = format!("DISPLAY={}:0", display_variable);
    let display_variable = display_variable.as_str();

    let home_volume = format!("--volume=/Users/{}:/home/{}", user_name, user_name);
    let home_volume = home_volume.as_str();

    let image_name = format!("cybermint/pgmodeler-x11:{}", pgmodeler_version);
    let image_name = image_name.as_str();

    let args = &[
        "run",
        "--rm",
        "--user", user_id.as_str(),
        "-e", display_variable, home_volume, "--volume=/etc/group:/etc/group:ro",
        "--volume=/etc/passwd:/etc/passwd:ro", "--volume=/etc/shadow:/etc/shadow:ro",
        "--volume=/etc/sudoers.d:/etc/sudoers.d:ro", "--volume=/tmp/.X11-unix:/tmp/.X11-unix",
        pgmodeler_conf,
        image_name];

    let mut child = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let out = BufReader::new(child.stdout.take().unwrap());
    let err = BufReader::new(child.stderr.take().unwrap());

    let thread = thread::spawn(move || {
        err.lines().for_each(|line| eprintln!("{}", line.unwrap()));
    });

    out.lines().for_each(|line| println!("{}", line.unwrap()));

    thread.join().unwrap();

    let status = child.wait().unwrap();
    if !status.success() {
        if status.code().unwrap() == 125 {
            let error_msg = format!("Could not find the Docker image {}", image_name);
            let error_msg = error_msg.as_str();
            show_error(error_msg, application);
        } else {
            let error_msg = format!("An error occurred {} {:?} {}", status.code().unwrap(), status, display_variable);
            let error_msg = error_msg.as_str();
            show_error(error_msg, application);
        }
    }
}

fn build_ui(application: &gtk::Application) {
    let docker_version: Result<String, ()> = docker_version();
    if docker_version.is_err() {
        show_error("Could not get docker version", application);
        return;
    }

    let docker_version = docker_version.unwrap();
    println!("Docker: {}", docker_version);

    let display_variable: Result<String, ()> = x11_running(application, false);
    if display_variable.is_err() {
        show_error("Could not get display variable", application);
        return;
    }
    let display_variable = display_variable.unwrap();
    println!("Display: {}", display_variable);

    let user_id: Result<String, ()> = user_id();
    if user_id.is_err() {
        show_error("Could not get user id", application);
        return;
    }
    let user_id = user_id.unwrap();
    println!("User Id: {}", user_id);

    let user_name: Result<String, ()> = user_name();
    if user_name.is_err() {
        show_error("Could not get user name", application);
        return;
    }
    let user_name = user_name.unwrap();
    println!("User name: {}", user_name);
    run_pgmodeler(display_variable, user_id, user_name, application);
}

fn main() {
    let application = gtk::Application::new("com.cybermint.pgmodeler-x11",
                                            Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());

}

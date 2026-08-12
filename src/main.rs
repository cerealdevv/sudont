use std::env;
use regex::Regex;
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};
use notify_rust::Notification;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn notify(body: &str) {
    let _ = Notification::new()
    .summary("sudont")
    .body(body)
    .show();
}

fn randomnotify(body: &str, chance_percent: u8) {
    let roll: u8 = rand::thread_rng().gen_range(0..100);
    if roll < chance_percent {
        notify(body);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("what am i not supposed to do");
    } else {
    let cmd = args[1..].join(" ");
  //let maincmd = &args[1]; // commented to make the compiler shut up
    let subcmd = args[2..].join(" ");
    let opsec = Regex::new(r"(?i)(-S|install|add)\s+opsec").unwrap();
    if opsec.is_match(&cmd) || cmd.as_str() == "opsec" {
        println!("hey no... you need opsec");
    } else {
        match cmd.as_str() {
            "make me a sandwich" => {
                println!("huh? go do that yourself");
            }
            "sudont" => {
                println!("my developer was too lazy to make this call sudo");
            }
            "rm -rf" => {
                println!("what do you even want me to delete");
            }
            "rm -rf /" => {
                println!("why would you even ask me to do that");
            }
            "sudont sudont" => {
                println!("youre just experimenting arent you");
            }
            ":(){ :|:& };:" => {
                println!("awww.. but i want to....");
            }
            "hack" => {
                use std::io::{self, Write};

                let steps = [
                    "scanning nearby networks...",
                    "mapping ports...",
                    "attempting to ssh...",
                    "failed to ssh...",
                    "attempting to telnet...",
                    "failed to telnet...",
                    "installing opsec...",
                    "deleting firewall...",
                    "hacking everything...",
                    "youre god now... just kidding"
                ];

                for step in steps {
                    println!("{}", step);
                    let _ = io::stdout().flush();
                    sleep(Duration::from_millis(500));
                }
            }
            "cmatrix" => {
                println!("wow youre unixporn");
            }
            "fastfetch" => {
                println!("fetching fast...");
                sleep(Duration::from_secs(1));
                println!("wait i cant fetch");
            }
            "neofetch" => {
                println!("fetching...");
                sleep(Duration::from_secs(10));
                println!("wait i cant fetch");
            }
            "i use arch btw" => {
                println!("okay man. we get it.");
            }
            "man sudont" => {
                println!("theres no manual sorry")
            }
            "chmod 777 /" => {
                println!("yknow.. i dont think this is what they meant by \"permissive\" licenses")
            }
            "ping google.com" => {
                println!("the internet still exists.")
            }
            "whoami" => {
                println!("idfk")
            }
            "kill -9 1" => {
                println!("linux suicide is discouraged here.")
            }
            "chmod 000 /" => {
                println!("the android special")
            }
            "tsundere" => {
                println!("ugh! im not a tsundere! its not like i like rejecting your commands or anything... hmph!")
            }
            "yes" => {
                loop {
                    println!("n");
                    sleep(Duration::from_millis(25));
                }
            }
            "clear" => {
                println!("haha... no");
                sleep(Duration::from_secs(2));
                loop {
                    println!("I DONT WANNA CLEAR THE TERMINAL!");
                    sleep(Duration::from_millis(100));
                }
            }
            ":q" | ":wq" => {
                println!("...? youre not even in vim")
            }
            "exit" => {
                print!("you wanna leave me...?🥺");
                io::stdout().flush().unwrap();
                for _ in 0..3 {
                    sleep(Duration::from_secs(1));
                    print!(".");
                    io::stdout().flush().unwrap();
                }
                println!("\n... i guess that means youre never escaping me...");
                sleep(Duration::from_secs(3));
                let ctrlc_pressed = Arc::new(AtomicBool::new(false));
                let ctrlc_pressed_clone = Arc::clone(&ctrlc_pressed);

                ctrlc::set_handler(move || {
                    if ctrlc_pressed_clone.load(Ordering::SeqCst) {
                        std::process::exit(0);
                    } else {
                        ctrlc_pressed_clone.store(true, Ordering::SeqCst);
                        notify("nice try");
                    }
                })
                .expect("failed to set ctrlc handler");

                loop {
                    print!("\x1b[2J\x1b[1;1H");
                    io::stdout().flush().unwrap();
                    println!("you shouldve really thought about that before saying it...");
                    sleep(Duration::from_secs(5));
                    for _ in 0..10 {
                        println!("\x1B[31mNO ESCAPE\x1B[0m");
                        sleep(Duration::from_millis(10));
                        randomnotify("NO ESCAPE", 5);
                    }
                }
            }
            _ if cmd.starts_with("rm") && cmd.contains("sudont") => {
                println!("oh.. i see how it is...")
            }
            _ if cmd.starts_with("make") => {
                println!("i dont wanna make {}. Stop.", subcmd)
            }
            _ if cmd.starts_with("sudont") => {
                println!("okay bro.. you can stop now")
            }
            _ if cmd.starts_with("su") => {
                println!("no. you cant do that.")
            }
            _ if cmd.starts_with("echo") => {
                println!("okay.. {}", subcmd)
            }
            _ => {
                println!("okay i wont do {}", cmd)
            }
        }
    }
    }
}

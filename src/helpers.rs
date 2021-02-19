pub mod helpers{
    use std::process::{Command, Stdio};
    use std::str;

    pub fn start_driver(gecko_path:String){
        println!("Starting a new geckodriver instance...");
        let com = Command::new(gecko_path)
            .arg("--connect-existing")
            .arg("--marionette-port")
            .arg("2828")
            .spawn()
            .expect("Cant find ur Geckodriver.");
    }

    pub fn cleaning_service(){
        println!("Starting up cleaning service.");
        let mut com1 = Command::new("pidof")
            .arg("geckodriver")
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        if com1.stdout.len() > 0{
            println!("Gecko found, crushing it.");
            com1.stdout.pop();//remove new line
            let out = str::from_utf8(&com1.stdout).unwrap();
            println!("Pid number {:?}", out);
            let com2 = Command::new("kill")
                .arg(out)
                .spawn()
                .unwrap();
            println!("Gecko utterly crushed.") ;   
        }else{
            println!("No Gecko running around.");
            return
        }
        let mut com3 = Command::new("pidof")
            .arg("firefox")
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        if com3.stdout.len() > 0{
            println!("Closing Firefox, hope you had nothing going on");
            com3.stdout.pop();
            let out2 = str::from_utf8(&com3.stdout).unwrap();
            let com4 = Command::new("kill")
                .arg(out2)
                .spawn()
                .unwrap();
            println!("Firefox is dead.");
        }else{
            println!("No running instance of firefox was found, moving on.");
            return
        }
    }


    pub fn headless_firefox(firefox_path:String, firefox_profile:Option<String>){
        println!("Starting headless firefox, marionette on 2828");
        match firefox_profile{
            Some(path) => {
                println!("Starting firefox, with selected profile.\n");
                let command = Command::new(firefox_path)
                    .arg("-headless")
                    .arg("-marionette")
                    .arg("--profile")
                    .arg(path)
                    .spawn()
                    .expect("Something terrible has happened to firefox!\n");
            },
            None => {
                println!("No path provided, probably using default profile.\n");
                let command = Command::new(firefox_path)
                    .arg("-headless")
                    .arg("-marionette")
                    .spawn()
                    .expect("Something terrible has happened to firefox!\n");
            }
        }
    }

}
use std::io;
use std::time::Duration ; use std::thread::sleep;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

struct Timer{
    time: u64,
    name: String,
}

impl Timer{
    fn tick(&mut self){
        self.time -= 1;
        if self.time <= 0 {
          println!("{} TIMER COMPLETE", self.name.to_uppercase());
          play_audio();
        }
    }

    fn new(time:u64, name: String) -> Self{
        Self {
            time,
            name
        }
    }
}

fn play_audio(){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("src/bell.ogg").unwrap());
    let source = Decoder::new(file).unwrap();
    match stream_handle.play_raw(source.convert_samples()) {
        Ok(_) => {},
        Err(e) => {println!("Something went wrong \n{}", e);},
    }
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    println!("Press Enter to start");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Something went wrong, failed to receive input");
    
    let timer_names = ["Study", "Short Break", "Study", "Long Break"];
    let timer_times = [60*25, 60*2, 60*25, 60*15];

    for i in 0..4 {
        let mut timer = Timer::new(timer_times[i], String::from(timer_names[i]));
        let pb = ProgressBar::new(timer.time);

        println!("{} Timer Started", timer.name);
        while timer.time > 0 {
           sleep(Duration::new(1,0));
           pb.inc(1);
           timer.tick(); 
        }
        
        // give some time for user to realize what's happening
        sleep(Duration::new(3,0));
    } 
}


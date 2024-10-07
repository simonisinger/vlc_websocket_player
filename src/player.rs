use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use serde_json::{json, Value};
use vlc::{Event, EventType, Instance, Media, MediaPlayer, MediaPlayerVideoEx, State};

pub(crate) struct Player {
    mdp: MediaPlayer,
    instance: Instance,
    sender: Sender<String>,
    receiver: Receiver<String>,
}
impl Player {
    pub fn new (sender: Sender<String>, receiver: Receiver<String>) -> Player {
        let instance = Instance::new().unwrap();
        let mdp = MediaPlayer::new(&instance).unwrap();
        mdp.set_fullscreen(true);

        let em = mdp.event_manager();
        let event_sender = sender.clone();
        let _ = em.attach(EventType::MediaStateChanged, move |e, vlc|{
            match e {
                Event::MediaStateChanged(s) => {
                    println!("State : {:?}", s);
                    if s == State::Ended || s == State::Error {
                        let event = json!({
                            "event": "stop".to_string()
                        });
                        event_sender.send(
                            event.to_string()
                        ).unwrap()

                    }
                }
                _ => {}
            }
        });

        Self {mdp, instance, sender, receiver}
    }

    fn handle_message(&mut self) {
        let message = self.receiver.try_recv();
        if message.is_err() {
            return;
        }

        let raw_json = message.unwrap();

        let json_value: Value = serde_json::from_str(&raw_json).unwrap();
        let command: &str = json_value["command"].as_str().unwrap();

        match command {
            "play" => self.play(json_value["path"].as_str().unwrap()),
            "stop" => self.stop(),
            "pause" => self.pause(),
            "seek-forward" => self.seek_forward(json_value["seconds"].as_i64().unwrap()),
            "seek-backward" => self.seek_backward(json_value["seconds"].as_i64().unwrap()),
            _ => {}
        }
    }

    pub fn process_messages(&mut self) {
        loop {
            self.handle_message();

            if self.mdp.is_playing() {
                let event = json!({
                    "event": "status".to_string(),
                    "time": self.mdp.get_time().unwrap()
                });
                self.sender.send(event.to_string()).unwrap();
                // I know sleep is evil but here its most reliable option
                sleep(Duration::from_millis(500));
            }

        }
    }

    fn stop(&mut self) {
        if self.mdp.is_playing() {
            self.mdp.stop();
        }
    }

    fn pause(&mut self) {
        let event: Value;
        if !self.mdp.get_media().is_none() {
            if self.mdp.is_playing() {
                self.mdp.pause();
                event = json!({
                    "event": "pause".to_string()
                });
            } else {
                event = json!({
                    "event": "play".to_string()
                });
                self.mdp.play().unwrap();
            }
            let _ = self.sender.send(event.to_string());
        }
    }

    fn seek_forward(&mut self, seek_seconds: i64) {
        if self.mdp.is_seekable() {
            let current_time = self.mdp.get_time().unwrap();
            self.mdp.set_time(current_time + (seek_seconds * 1000));
        }
    }
    fn seek_backward(&mut self, seek_seconds: i64) {
        if self.mdp.is_seekable() {
            let current_time = self.mdp.get_time().unwrap();
            self.mdp.set_time(current_time - (seek_seconds * 1000));
        }
    }

    fn play(&mut self, path: &str) {
        let md = Media::new_path(&self.instance, path).unwrap();
        self.mdp.set_media(&md);
        // Start playing
        self.mdp.play().unwrap();

        let event = json!({
            "event": "play".to_string(),
            "value": path.to_string()
        });
        self.sender.send(event.to_string()).unwrap();
    }
}
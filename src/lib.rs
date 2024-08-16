use std::collections::HashMap;

mod config;
mod textbox;

// to get the whole script file
static SCRIPT_PATH: &str = std::include_str!("../scripts/script.tdsl");

turbo::init! {
    struct GameState {
        speaking_char: u8,
        lines: Vec<String>,
        current_line: usize,
        wait_timer: u16,
        tweens: HashMap<String, Tween<f32>>,
        tween_done_once: bool,
    } = {
        Self::new()
    }
}

impl GameState {
    fn new() -> Self {
        Self {
            speaking_char: 0,
            lines: SCRIPT_PATH.split("\n")
                .map(|line| line.to_string())
                .collect(),
            current_line: 0,
            wait_timer: 0,
            tweens: HashMap::from([
                ("pop_in_portrait".to_string(), Tween::new(0.)),
                ("fade_in_portrait".to_string(), Tween::new(0.)),
                ]),
            tween_done_once: false,
        }
    }
    
    fn assess_current_line(&mut self) {
        match &self.lines[self.current_line] {
            line if line.starts_with("<<") || line.starts_with("#") || line == "" => {
                // is a passage, send, comment, or blank line, so increment on to next line
                self.current_line += 1;
            },
            line if line.starts_with(">>") => {
                // get divert text value
                let mut divert_text = self.lines[self.current_line].chars();
                divert_text.next();
                divert_text.next();
                // move to divert area!
                let new_knot_index: usize = self.lines
                    .iter()
                    .position(|line| *line == format!("{}{}", "<< ", divert_text.as_str().trim()))
                    .unwrap();
                self.current_line = new_knot_index;
            },
            line if line.starts_with("]>") => {
                // choice logic
                self.evaluate_choice();
            },
            line if line.starts_with("!") => {
                // command block
                self.evaluate_command();
            },
            line if line.starts_with("-- end") => {
                // set character to none
                self.speaking_char = 0;
                //qad finish
                text!("fin", x = 16, y = 174 + 8);
                // wait for button press to restart
                if gamepad(0).select.just_pressed() {
                    *self = GameState::new();
                }
            }
            _ => {
                // regular line
                self.print_current_line();
            },
        }
    }
    
    fn print_current_line(&mut self) {
        // split at the : to get character and line
        let statement: Vec<String> = self.lines[self.current_line]
            .split(":")
            .filter(|&element| element != "")
            .map(|element| element.trim().to_string())
            .collect();
        // draw char portrait
        match statement[0].as_str() {
            "LEFT" => self.speaking_char = 1,
            "RIGHT" => self.speaking_char = 2,
            _ => {},
        }
        
        // draw textbox
        textbox::render_textbox(&statement);
        
        // move this maybe into a bespoke input checker?
        if gamepad(0).start.just_pressed() {
            self.current_line += 1;
            self.tween_done_once = false;
        }
    }
    
    fn evaluate_command(&mut self) {
        // no one talking!
        self.speaking_char = 0;
        // increment wait time, like a local tick basically
        self.wait_timer += 1;

        // split at / for time
        let command_with_arg: Vec<String> = self.lines[self.current_line]
            .split("/")
            .filter(|&element| element != "")
            .map(|element| element.trim().to_string())
            .collect();
        let (command, arg) = (&command_with_arg[0], &command_with_arg[1]);

        match command.as_str() {
            "! WAIT" => {
                // dont increment the line for a period of time
                if self.wait_timer == arg.parse::<u16>().unwrap() * 60 {
                    self.current_line += 1;
                    // reset wait_timer
                    self.wait_timer = 0;

                    self.tween_done_once = false;
                }
            },
            _ => {}
        }
    }

    fn evaluate_choice(&mut self) {
        // split the current line at the ]>
        let choices: Vec<String> = self.lines[self.current_line]
            .split("]>")
            .filter(|&element| element != "")
            .map(|choice| choice.trim().to_string())
            .collect();
        
        // set speaking character to none for choices
        self.speaking_char = 0;
        
        textbox::render_choice_textbox(&choices);
        
        // look forward to next line, split at >> to get diverts
        let diverts: Vec<String> = self.lines[self.current_line + 1]
            .split(">>")
            .filter(|&element| element != "")
            .map(|divert| divert.trim().to_string())
            .collect();
        
        // do input check for left or right
        if gamepad(0).left.just_pressed() {
            if diverts[0] == "NULL" {
                // pass entirely
                return;
            }
            // search the full script to see where << that is, get that index, set current line to that
            let new_knot_index: usize = self.lines
                .iter()
                .position(|line| *line ==  format!("{}{}", "<< ", diverts[0]))
                .unwrap();
            
            self.current_line = new_knot_index;

            self.tween_done_once = false;
        }
        else if gamepad(0).right.just_pressed() {
            if diverts[1] == "NULL" {
                // pass entirely
                return;
            }
            // search the full script to see where << that is, get that index, set current line to that
            let new_knot_index: usize = self.lines
                .iter()
                .position(|line| *line ==  format!("{}{}", "<< ", diverts[1]))
                .unwrap();
            
            self.current_line = new_knot_index;

            self.tween_done_once = false;
        }
        else if gamepad(0).up.just_pressed() {
            if diverts[2] == "NULL" {
                // pass entirely
                return;
            }
            // search the full script to see where << that is, get that index, set current line to that
            let new_knot_index: usize = self.lines
                .iter()
                .position(|line| *line ==  format!("{}{}", "<< ", diverts[2]))
                .unwrap();
            
            self.current_line = new_knot_index;

            self.tween_done_once = false;
        }
        else if gamepad(0).down.just_pressed() {
            if diverts[3] == "NULL" {
                // pass entirely
                return;
            }
            // search the full script to see where << that is, get that index, set current line to that
            let new_knot_index: usize = self.lines
                .iter()
                .position(|line| *line ==  format!("{}{}", "<< ", diverts[3]))
                .unwrap();
            
            self.current_line = new_knot_index;

            self.tween_done_once = false;
        }
    }
}

turbo::go! {
    let mut state = GameState::load();
    
    // static imgs
    sprite!("bg", x = 0, y = 0);
    
    // animated imgs
    sprite!("anim_water_grass", x = 0, y = 77, sw = 384, fps = fps::SLOW);
    sprite!("anim_protag", x = 83, y = 64, sw = 79, fps = fps::SLOW);
    sprite!("anim_antag", x = 215, y = 68, sw = 77, fps = fps::SLOW);
    sprite!("anim_foliage_back", x = 0, y = 0, sw = 384, opacity = 0.65, fps = fps::SLOW);
    sprite!("anim_foliage_front", x = 0, y = 0, sw = 384, fps = fps::SLOW);
    
    // conditional draw of correct portrait and bubble
    match state.speaking_char {
        1 => {

            // parallel tween logic
            if !state.tween_done_once {
                state.tweens.insert("pop_in_portrait".to_string(), Tween::new(1.1).set(1.).duration(15).ease(Easing::EaseInOutSine));
                state.tweens.insert("fade_in_portrait".to_string(), Tween::new(0.).set(1.).duration(15).ease(Easing::EaseInOutSine));
                state.tween_done_once = true;
            }

            // draw portrait one
            sprite!("anim_protag_portrait", 
                x = 12, 
                y = 126. * state.tweens.get_mut("pop_in_portrait").unwrap().get(), 
                sw = 47,
                opacity = state.tweens.get_mut("fade_in_portrait").unwrap().get(),
                fps = fps::REALLY_SLOW);
            // draw bubble one
            sprite!("bubble_protag",
                x = 134, 
                y = 43. * state.tweens.get_mut("pop_in_portrait").unwrap().get(),
                opacity = state.tweens.get_mut("fade_in_portrait").unwrap().get());
        },
        2 => {

            // parallel tween logic
            if !state.tween_done_once {
                state.tweens.insert("pop_in_portrait".to_string(), Tween::new(1.1).set(1.).duration(15).ease(Easing::EaseInOutSine));
                state.tweens.insert("fade_in_portrait".to_string(), Tween::new(0.).set(1.).duration(15).ease(Easing::EaseInOutSine));
                state.tween_done_once = true;
            }

            // draw portrait two
            sprite!("anim_antag_portrait", 
                x = 384 - 47 - 12, 
                y = 126. * state.tweens.get_mut("pop_in_portrait").unwrap().get(), 
                sw = 47,
                opacity = state.tweens.get_mut("fade_in_portrait").unwrap().get(),
                fps = fps::REALLY_SLOW);
            // draw bubble two
            sprite!("bubble_antag", 
                x = 193, 
                y = 50. * state.tweens.get_mut("pop_in_portrait").unwrap().get(),
                opacity = state.tweens.get_mut("fade_in_portrait").unwrap().get());
        },
        _ => {}
    }
    
    GameState::assess_current_line(&mut state);
    
    state.save();
}
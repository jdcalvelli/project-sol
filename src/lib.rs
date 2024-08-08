mod config;
mod textbox;

// to get the whole script file
static SCRIPT_PATH: &str = std::include_str!("script.tdsl");

turbo::init! {
    struct GameState {
        speaking_char: u8,
        lines: Vec<String>,
        current_line: usize,
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
        }
    }
    
    fn assess_current_line(&mut self) {
        match &self.lines[self.current_line] {
            line if line.starts_with("<<") || line.starts_with(">>") || line.starts_with("#") || line == "" => {
                // is a passage, send, comment, or blank line, so increment on to next line
                self.current_line += 1;
            },
            line if line.starts_with("]>") => {
                // choice logic
                self.evaluate_choice();
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
            // search the full script to see where << that is, get that index, set current line to that
            let new_knot_index: usize = self.lines
                .iter()
                .position(|line| *line ==  format!("{}{}", "<< ", diverts[0]))
                .unwrap();
            
            self.current_line = new_knot_index;
        }
        else if gamepad(0).right.just_pressed() {
            // search the full script to see where << that is, get that index, set current line to that
            let new_knot_index: usize = self.lines
                .iter()
                .position(|line| *line ==  format!("{}{}", "<< ", diverts[1]))
                .unwrap();
            
            self.current_line = new_knot_index;
        }
    }
}

turbo::go! {
    let mut state = GameState::load();
    
    // i think i want to kick these out to objects type stuff at some point
    sprite!("bg_0", x = 0, y = 0);
    sprite!("chairs", x = 0, y = 0);
    sprite!("foliage", x = 0, y = 0);
    
    // conditional draw of correct portrait
    // this might be able to move into the game
    match state.speaking_char {
        1 => {
            // draw portrait one
            sprite!("test_portrait_1", x = 0, y = 0);
        },
        2 => {
            // draw portrait two
            sprite!("test_portrait_2", x = 0, y = 0);
        },
        _ => {}
    }
    
    GameState::assess_current_line(&mut state);
    
    state.save();
}
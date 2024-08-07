mod config;
mod textbox;

// to get the whole script file
static SCRIPT_PATH: &str = std::include_str!("script.sol");

turbo::init! {
    struct GameState {
        lines: Vec<String>,
        current_line: usize,
    } = {
        Self {
            lines: SCRIPT_PATH.split("\n")
                .map(|line| line.to_string())
                .collect(),
            current_line: 0,
        }
    }
}

impl GameState {
    fn assess_current_line(&mut self) {
        match &self.lines[self.current_line] {
            line if line.starts_with("<<") || line.starts_with(">>") || line == "" => {
                // knot, divert, or empty, so increment on to next line
                self.current_line += 1;
            },
            line if line.starts_with("]>") => {
                // choice logic
                self.evaluate_choice();
            },
            line if line.starts_with("-- end") => {
                log!("GAME END");
            }
            _ => {
                // regular line
                self.print_current_line();
            },
        }
    }
    
    fn print_current_line(&mut self) {
        // draw textbox
        textbox::render_textbox(&self.lines, &self.current_line);
        // draw text
        //text!(self.lines[self.current_line].as_str(), x = 0, y = 174);
        
        // move this maybe into a bespoke input checker?
        if gamepad(0).start.just_pressed() {
            self.current_line += 1;
        }
    }
    
    fn evaluate_choice(&mut self) {
        // split the current line at the ]>
        let choices: Vec<String> = self.lines[self.current_line]
            .split("]>")
            .filter(|element| *element != "")
            .map(|choice| choice.trim().to_string())
            .collect();
        
        textbox::render_choice_textbox(&choices);
        
        // look forward to next line, split at >> to get diverts
        let diverts: Vec<String> = self.lines[self.current_line + 1]
            .split(">>")
            .filter(|element| *element != "")
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
    sprite!("bg_1", x = 0, y = 0);
    sprite!("foliage", x = 0, y = 0);
    
    GameState::assess_current_line(&mut state);
    
    state.save();
}
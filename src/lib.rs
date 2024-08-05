mod config;

// to get the whole script file
static SCRIPT_PATH: &str = std::include_str!("test.txt");

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
            line if line.starts_with("]*") => self.evaluate_choice(),
            _ => self.print_current_line(),
        }
    }
    
    fn print_current_line(&mut self) {
        text!(self.lines[self.current_line].as_str());
        
        // move this maybe into a bespoke input checker?
        if gamepad(0).start.just_pressed() {
            self.current_line += 1;
        }
    }
    
    fn evaluate_choice(&mut self) {
        // split the current line at the ]*
        let choices: Vec<String> = self.lines[self.current_line]
            .split("]*")
            .map(|choice| choice.trim().to_string())
            .collect();
        // draw choice one in one place, choice two in another
        text!(choices[1].as_str(), x = 0, y = 0);
        text!(choices[2].as_str(), x = 100, y = 0);
        // do input check for left or right
        if gamepad(0).up.just_pressed() {
            // will have to update this for moving to knots eventually
            text!("choice one picked", x = 0, y = 50);
        }
        else if gamepad(0).down.just_pressed() {
            // will have to update this for moving to knots eventually
            text!("choice two picked", x = 0, y = 50);
        }
    }
}

turbo::go! {
    let mut state = GameState::load();
    
    GameState::assess_current_line(&mut state);
    
    state.save();
}
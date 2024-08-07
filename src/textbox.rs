use turbo::*;
use turbo::prelude::Font;

// consts for textboxes related
const TB_WIDTH: u16 = 384;
const TB_HEIGHT: u16 = 174;
const TB_X: u16 = 0;
const TB_Y: u16 = 174;
const TB_PADDING: u16 = 16;

pub fn render_textbox(lines: &Vec<String>, current_line: &usize) -> bool {
	rect!(w = TB_WIDTH, h = TB_HEIGHT, x = TB_X, y = TB_Y, color = 0x000000ff, border_radius = 8);
	
	text!(lines[*current_line].as_str(), x = TB_X + TB_PADDING, y = TB_Y + TB_PADDING);
	
	// its rendering it
	true
}

pub fn render_choice_textbox(choices: &Vec<String>) -> bool {
	rect!(w = TB_WIDTH, h = TB_HEIGHT, x = TB_X, y = TB_Y, color = 0x000000ff, border_radius = 8);
	
	// display first choice
	text!(choices[1].as_str(), x = TB_X + TB_PADDING, y = TB_Y + TB_PADDING);
	// display second choice
	text!(choices[2].as_str(), x = TB_X + (TB_WIDTH / 2) + TB_PADDING, y = TB_Y + TB_PADDING);
	
	true
}
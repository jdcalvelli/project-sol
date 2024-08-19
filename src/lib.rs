use std::collections::HashMap;

mod config;
mod director;

// to get the whole script file
static SCRIPT_PATH: &str = std::include_str!("../scripts/script.tdsl");

turbo::init! {
    struct GameState {
        scene: u8,
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
            scene: 0,
            speaking_char: 0,
            lines: SCRIPT_PATH.split("\n")
                .map(|line| line.to_string())
                .collect(),
            current_line: 0,
            wait_timer: 0,
            tweens: HashMap::from([
                ("pop_in_portrait".to_string(), Tween::new(0.)),
                ("fade_in_portrait".to_string(), Tween::new(0.)),
                ("tween_down_cam".to_string(), Tween::new(0.)),
                ("tween_up_cam".to_string(), Tween::new(0.)),
                ]),
            tween_done_once: false,
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
                state.tweens.insert(
                    "pop_in_portrait".to_string(), 
                    Tween::new(1.1).set(1.).duration(15).ease(Easing::EaseInOutSine)
                );
                state.tweens.insert(
                    "fade_in_portrait".to_string(), 
                    Tween::new(0.).set(1.).duration(15).ease(Easing::EaseInOutSine)
                );
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
                state.tweens.insert(
                    "pop_in_portrait".to_string(), 
                    Tween::new(1.1).set(1.).duration(15).ease(Easing::EaseInOutSine)
                );
                state.tweens.insert("fade_in_portrait".to_string(), 
                    Tween::new(0.).set(1.).duration(15).ease(Easing::EaseInOutSine)
                );
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
    
    match state.scene {
        0 => {
            if gamepad(0).start.just_pressed() {

                state = GameState::new();

                // start the tween down
                state.tweens.insert(
                    "tween_down_cam".to_string(), 
                    Tween::new(-216.).set(108.).duration(120).ease(Easing::EaseInOutSine)
                );
                state.tween_done_once = true;
            }
            if state.tween_done_once {
                set_cam!(x = 192, y = state.tweens.get_mut("tween_down_cam").unwrap().get());

                if state.tweens.get_mut("tween_down_cam").unwrap().done() {
                    state.scene = 1;
                    state.tween_done_once = false;
                }

            }
            else {
                set_cam!(x = 192, y = -216);
            }
        },
        1 => {
            director::assess_current_line(&mut state);
        },
        2 => {
            if !state.tween_done_once {
                state.tweens.insert(
                    "tween_up_cam".to_string(),
                    Tween::new(108.).set(-216.).duration(120).ease(Easing::EaseInOutSine)
                );
                state.tween_done_once = true;
            }
            else {
                set_cam!(x = 192, y = state.tweens.get_mut("tween_up_cam").unwrap().get());
                if state.tweens.get_mut("tween_up_cam").unwrap().done() {
                    state.scene = 0;
                    state.tween_done_once = false;
                }

            }
        }
        _ => panic!("CRITICAL - No scene corresponds to this value.")
    }    
    state.save();
}
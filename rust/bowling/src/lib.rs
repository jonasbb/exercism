#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

struct Frame(Option<u16>, Option<u16>);

impl Frame {
    fn new() -> Self {
        Frame(None, None)
    }

    fn roll(&mut self, value: u16) -> Result<()> {
        if self.get_pins() + value > 10 {
            bail!("Can only roll 10 pins per frame, but tried to roll {} and frame already \
                   contained {} pins.",
                  value,
                  self.get_pins());
        }

        if self.0.is_none() {
            self.0 = Some(value);
            Ok(())
        } else if self.0.is_some() && !self.is_strike() && self.1.is_none() {
            self.1 = Some(value);
            Ok(())
        } else {
            bail!("All rolls are already done for this frame")
        }
    }

    fn is_strike(&self) -> bool {
        if let Some(x) = self.0 { x == 10 } else { false }
    }

    fn is_spare(&self) -> bool {
        if let (Some(x), Some(y)) = (self.0, self.1) {
            x + y == 10
        } else {
            false
        }
    }

    fn is_open(&self) -> bool {
        if let (Some(x), Some(y)) = (self.0, self.1) {
            x + y < 10
        } else {
            false
        }
    }

    fn is_finished(&self) -> bool {
        self.is_strike() || self.is_spare() || self.is_open()
    }

    /// Returns the sum of all rolls within this frame or 0 if no roll has happend yet
    fn get_pins(&self) -> u16 {
        let mut res = 0;
        if let Some(x) = self.0 {
            res += x;
            if let Some(y) = self.1 {
                res += y;
            }
        };
        res
    }

    fn get_first_roll(&self) -> Result<u16> {
        if let Some(x) = self.0 {
            Ok(x)
        } else {
            bail!("No first roll performed yet")
        }
    }

    fn get_second_roll(&self) -> Result<u16> {
        if let Some(x) = self.1 {
            Ok(x)
        } else {
            bail!("No second roll performed yet")
        }
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    is_done: bool,
    fill_balls: u16,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            frames: Vec::with_capacity(12),
            is_done: false,
            fill_balls: 0,
        }
    }

    pub fn roll(&mut self, value: u16) -> Result<()> {
        if self.is_done {
            bail!("The game is alreay finished")
        }
        if value > 10 {
            bail!("Cannot roll more then 10 pins")
        }

        // track fill balls
        if self.fill_balls > 0 {
            self.fill_balls -= 1;
            if self.fill_balls == 0 {
                self.is_done = true;
            }
        }

        // update frames with new frame and roll
        if self.frames.is_empty() || self.frames.last().unwrap().is_finished() {
            self.frames.push(Frame::new());
        }
        self.frames.last_mut().unwrap().roll(value).chain_err(|| "Unable to roll in game")?;

        // calculate fill balls or game end
        if self.frames.len() == 10 && self.frames.last().unwrap().is_finished() {
            if self.frames.last().unwrap().is_spare() {
                self.fill_balls = 1;
            } else if self.frames.last().unwrap().is_strike() {
                self.fill_balls = 2;
            } else {
                self.is_done = true;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u16> {
        if !self.is_done {
            bail!("The game is not yet finished {} {}",
                  self.frames.len(),
                  self.fill_balls)
        }

        let mut score = 0;
        // frame might be real frame or fill frame, so always only count 10 frames
        for i in 0..10 {
            score += self.frames[i].get_pins();

            // count bonus points
            if self.frames[i].is_strike() {
                score += self.frames[i + 1].get_first_roll().unwrap();
                score += match self.frames[i + 1].get_second_roll() {
                    Ok(x) => x,
                    Err(_) => self.frames[i + 2].get_first_roll().unwrap(),
                };
            }
            if self.frames[i].is_spare() {
                score += self.frames[i + 1].get_first_roll().unwrap();
            }
        }

        Ok(score)
    }
}

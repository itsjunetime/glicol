use crate::{Buffer, Input, Message, Node};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct EnvPerc {
    attack: f32,
    decay: f32,
    pos: usize,
    scale: f32,
    sr: usize,
    input_order: Vec<usize>,
}

impl Default for EnvPerc {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvPerc {
    pub fn new() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            pos: 0,
            scale: 1.0,
            sr: 44100,
            input_order: vec![],
        }
    }

    pub fn attack(self, attack: f32) -> Self {
        Self { attack, ..self }
    }
    pub fn decay(self, decay: f32) -> Self {
        Self { decay, ..self }
    }
    pub fn scale(self, scale: f32) -> Self {
        Self { scale, ..self }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }
}

impl<const N: usize> Node<N> for EnvPerc {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        if inputs.len() == 1 {
            let attack_len = (self.attack * self.sr as f32) as usize;
            let decay_len = (self.decay * self.sr as f32) as usize;
            let dur = attack_len + decay_len;
            let buf = &mut inputs[&self.input_order[0]].buffers();

            for (input, out) in buf[0].iter().zip(output[0].iter_mut()) {
                if *input > 0.0 {
                    self.pos = 0;
                    self.scale = *input;
                }
                if self.pos <= attack_len {
                    if attack_len == 0 {
                        *out = 0.0;
                    } else {
                        *out = self.pos as f32 / attack_len as f32;
                    }
                } else if self.pos > attack_len && self.pos <= dur {
                    if decay_len == 0 {
                        *out = 0.0;
                    } else {
                        *out = (dur - self.pos) as f32 / decay_len as f32;
                    }
                } else {
                    *out = 0.0
                }
                // println!("{}", output[0][i]);
                *out *= self.scale;
                self.pos += 1;
            }
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => match pos {
                0 => self.attack = value,
                1 => self.decay = value,
                _ => {}
            },
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}

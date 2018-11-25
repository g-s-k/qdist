#[macro_use]
extern crate vst;
extern crate num_traits;

use vst::buffer::AudioBuffer;
use vst::plugin::{Plugin, Info};

struct QDist {
    threshold: f32,
    bias: f32,
    bits: u8,
}

impl Default for QDist {
    fn default() -> QDist {
        QDist {
            threshold: 1.0,
            bias: 0.0,
            bits: 32,
        }
    }
}

impl Plugin for QDist {
    fn get_info(&self) -> Info {
        Info {
            name: "QDist".to_string(),
            vendor: "g-s-k".to_string(),
            unique_id: 24112018,

            inputs: 2,
            outputs: 2,
            parameters: 3,

            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            1 => self.bias / 2.0 + 0.5,
            2 => f32::from(self.bits) / 32.0,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.threshold = value.max(0.01),
            1 => self.bias = value * 2.0 - 1.0,
            2 => self.bits = (value * 32.0).max(1.0).round() as u8,
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            1 => "DC Offset".to_string(),
            2 => "Bit Depth".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.threshold * 100.0),
            1 => format!("{}", self.bias),
            2 => format!("{}", self.bits),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "".to_string(),
            2 => "bits".to_string(),
            _ => "".to_string(),
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        for (input, output) in buffer.zip() {
            for (input_sample, output_sample) in input.into_iter().zip(output.into_iter()) {
                *output_sample =
                    bit32(thresh(*input_sample + self.bias, self.threshold), self.bits);
            }
        }
    }
}

fn thresh<T>(input: T, threshold: T) -> T
where
    T: num_traits::float::Float + Default,
{
    if input < T::default() {
        input.max(-threshold) / threshold
    } else {
        input.min(threshold) / threshold
    }
}

fn bit32(input: f32, depth: u8) -> f32 {
    let shift = 32 - depth;
    f32::from_bits((input.to_bits() >> shift) << shift)
}

plugin_main!(QDist);

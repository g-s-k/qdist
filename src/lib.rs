#[macro_use] extern crate vst;
extern crate num_traits;

use vst::buffer::AudioBuffer;
use vst::plugin::{Plugin, Info};

struct QDist {
    threshold: f32,
    bias: f32,
}

impl Default for QDist {
    fn default() -> QDist {
        QDist {
            threshold: 1.0,
            bias: 0.0,
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
            parameters: 2,

            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            1 => self.bias,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.threshold = value.max(0.01),
            1 => self.bias = value * 2.0 - 1.0,
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            1 => "DC Offset".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.threshold * 100.0),
            1 => format!("{}", self.bias),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "".to_string(),
            _ => "".to_string(),
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.into_iter().zip(output_buffer.into_iter()) {

                *output_sample = thresh(*input_sample + self.bias, self.threshold);
            }
        }
    }
}

fn thresh<T>(input: T, threshold: T) -> T
where T: num_traits::float::Float + Default {
    if input < T::default() {
        input.min(-threshold) / threshold
    } else {
        input.min(threshold) / threshold
    }
}

plugin_main!(QDist);

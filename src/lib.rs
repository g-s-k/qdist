#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Plugin, Info};

struct QDist {
    threshold: f32
}

impl Default for QDist {
    fn default() -> QDist {
        QDist {
            threshold: 1.0
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
            parameters: 1,

            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.threshold = value.max(0.01),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.threshold * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();

        for (input_buffer, output_buffer) in inputs.iter().zip(outputs) {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {

                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(self.threshold) / self.threshold
                }
                else {
                    *output_sample = input_sample.min(-self.threshold) / self.threshold
                }
            }
        }
    }
}

plugin_main!(QDist);

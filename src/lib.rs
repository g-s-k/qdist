#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Plugin, Info};

#[derive(Default)]
struct QDist;

impl Plugin for QDist {
    fn get_info(&self) -> Info {
        Info {
            name: "QDist".to_string(),
            vendor: "g-s-k".to_string(),
            unique_id: 24112018,
            ..Info::default()
        }
    }
}

plugin_main!(QDist);

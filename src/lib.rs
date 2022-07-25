mod gui;

use std::sync::Arc;
use vst::plugin::PluginParameters;
use vst::prelude::*;

pub struct PestComp {
    pub params: Arc<PestCompParameters>,
    pub editor: Option<gui::PestCompEditor>,
}

unsafe impl Sync for PestComp {}

pub struct PestCompParameters {
    pub engage: AtomicFloat,
}

// default parameters
impl Default for PestCompParameters {
    fn default() -> PestCompParameters {
        PestCompParameters {
            engage: AtomicFloat::new(0.0),
        }
    }
}

impl Plugin for PestComp {
    fn get_info(&self) -> Info {
        Info {
            name: "PestCömp".into(),
            vendor: "Przychlast Records Ltd.".into(),
            unique_id: 6969,
            category: Category::Effect,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            ..Default::default()
        }
    }

    fn new(_host: HostCallback) -> Self {
        let params = Arc::new(Default::default());
        PestComp {
            params: Arc::clone(&params),
            editor: Some(gui::PestCompEditor {
                params,
                window_handle: None,
                is_open: false,
            }),
        }
    }

    // behold the dsp god
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let engage = self.params.engage.get();
        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = if engage < 0.5 { *input_sample } else { 0.0 };
            }
        }
    }

    fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn vst::editor::Editor>)
        } else {
            None
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for PestCompParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.engage.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.engage.set(value),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.engage.get() * 2f32),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Gain",
            _ => "",
        }
        .to_string()
    }
}

vst::plugin_main!(PestComp);

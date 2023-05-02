use std::{num::NonZeroU32, sync::Arc};

use nih_plug::{nih_export_vst3, prelude::*};

#[derive(Params)]
pub struct Parameters {
    #[id = "wet_dry"]
    pub wet_dry: FloatParam,
    #[id = "add_amount"]
    pub add_amount: FloatParam,
    #[id = "mult_amount"]
    pub mult_amount: FloatParam,
    #[id = "residue"]
    pub residue: BoolParam,
}
impl Parameters {
    fn new() -> Parameters {
        fn bool_to_yes(value: bool) -> String {
            if value { "Yes" } else { "No" }.to_string()
        }

        fn percent(name: &'static str, default: f32) -> FloatParam {
            fn formatter(percent: f32) -> String {
                format!("{:.1}", percent * 100.0)
            }
            let range = FloatRange::Linear { min: 0.0, max: 1.0 };
            FloatParam::new(name, default, range)
                .with_unit(" %")
                .with_value_to_string(Arc::new(formatter))
        }
        Parameters {
            wet_dry: percent("Wet Dry Mix", 1.0),
            add_amount: FloatParam::new(
                "Add Amount",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1024.0 * 1024.0 * 4.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            ),
            mult_amount: FloatParam::new(
                "Multiply Amount",
                1.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 1024.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            ),
            residue: BoolParam::new("Difference Only", false)
                .with_value_to_string(Arc::new(bool_to_yes)),
        }
    }
}

pub struct Hardbit {
    params: Arc<Parameters>,
}

impl Plugin for Hardbit {
    type SysExMessage = ();
    type BackgroundTask = ();

    const NAME: &'static str = "Hardbit";
    const VENDOR: &'static str = "a2aaron";
    const URL: &'static str = "https://a2aaron.github.io/";
    const EMAIL: &'static str = "aaronko@umich.edu";
    const VERSION: &'static str = "1.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = false;
    const HARD_REALTIME_ONLY: bool = false;

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_plug::wrapper::setup_logger();
        nih_log!("Initalizing VST...");
        true
    }

    fn params(&self) -> std::sync::Arc<dyn nih_plug::prelude::Params> {
        Arc::clone(&self.params) as Arc<dyn Params>
    }

    fn process(
        &mut self,
        buffer: &mut nih_plug::prelude::Buffer,
        _aux: &mut nih_plug::prelude::AuxiliaryBuffers,
        _context: &mut impl nih_plug::prelude::ProcessContext<Self>,
    ) -> nih_plug::prelude::ProcessStatus {
        let wet_dry = self.params.wet_dry.modulated_normalized_value();
        let add_amt = self.params.add_amount.modulated_plain_value();
        let mult_amt = self.params.mult_amount.modulated_plain_value();
        let residue = self.params.residue.modulated_plain_value();
        for (_, block) in buffer.iter_blocks(128) {
            for buffer in block {
                for value in buffer {
                    let out = *value;
                    let out = out * mult_amt + add_amt;
                    let out = (out - add_amt) / mult_amt;
                    let out = if residue { out - *value } else { out };
                    let out = if !out.is_finite() { *value } else { out };
                    let out = out.clamp(-1.0, 1.0);
                    *value = lerp(*value, out, wet_dry)
                }
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {}

    fn deactivate(&mut self) {}
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    (end - start) * t.clamp(0.0, 1.0) + start
}

impl Default for Hardbit {
    fn default() -> Self {
        Self {
            params: Arc::new(Parameters::new()),
        }
    }
}

impl Vst3Plugin for Hardbit {
    const VST3_CLASS_ID: [u8; 16] = *b"hardbit..a2aaron";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Distortion, Vst3SubCategory::Stereo];
}

impl ClapPlugin for Hardbit {
    const CLAP_ID: &'static str = "Hardbit";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Floating Point Roundoff Error Distortion");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://github.com/a2aaron/hardbit");
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Distortion,
        ClapFeature::AudioEffect,
        ClapFeature::Glitch,
        ClapFeature::Stereo,
    ];
}
// Export symbols for main
nih_export_vst3!(Hardbit);
nih_export_clap!(Hardbit);

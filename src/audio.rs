use std::collections::HashMap;
use std::fs::File;
use web_audio_api::context::{AsBaseAudioContext, AudioContext};
use web_audio_api::media::{MediaElement, OggVorbisDecoder};
use web_audio_api::node::{
    AudioControllableSourceNode, AudioNode, AudioScheduledSourceNode, GainNode,
    MediaElementAudioSourceNode,
};

pub struct Audio {
    context: AudioContext,
    music: HashMap<String, (MediaElementAudioSourceNode, GainNode)>,
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            context: AudioContext::new(),
            music: HashMap::new(),
        }
    }

    pub fn create_music(&mut self, name: &str, file: &str) {
        // setup background music:
        // read from local file
        let file = File::open(file).unwrap();
        // decode file to media stream
        let stream = OggVorbisDecoder::try_new(file).unwrap();
        // wrap stream in MediaElement, so we can control it (loop, play/pause)
        let media = MediaElement::new(stream);
        // register as media element in the audio context
        let background = self.context.create_media_element_source(media);
        // use a gain node to control volume
        let gain = self.context.create_gain();
        // play at low volume
        gain.gain().set_value(0.5);
        // connect the media node to the gain node
        background.connect(&gain);
        // connect the gain node to the destination node (speakers)
        gain.connect(&self.context.destination());
        self.music.insert(name.to_string(), (background, gain));
    }

    pub fn play_music(&mut self, name: &str) {
        match self.music.get(name) {
            Some(background) => {
                background.0.set_loop(true);
                background.0.start()
            }
            None => (),
        }
    }
}

use eframe::egui;
use egui::{
    Align, Color32, CornerRadius, DragValue, FontFamily, FontId, Frame, Layout, RichText, Stroke,
    TextStyle, Vec2,
};
use egui_file_dialog::FileDialog;
use kira::{
    effect::{
        delay::DelayBuilder,
        distortion::{DistortionBuilder, DistortionKind},
        filter::{FilterBuilder, FilterMode},
        reverb::ReverbBuilder,
    },
    sound::static_sound::StaticSoundData,
    track::{TrackBuilder, TrackPlaybackState},
    AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Tween, Value,
};
use std::thread::sleep;
use std::time::Duration;
use std::{ffi::OsStr, path::Path};

// Runs app with default native options
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Signal processing Toolbox",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .expect("Error loading the signal processing toolbox");
}

// App structure
#[derive(Default)]
struct MyEguiApp {
    file_dialog: FileDialog,
    speed_value: f64,
    reverb_value: f64,
    delay_value: u64,
    cutoff_value: f64,
    button_width: f32,
    button_height: f32,
    slider_width: f32,
    slider_height: f32,
}

// Initializing app default values
impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_dialog: FileDialog::new(),
            speed_value: 2.0,
            reverb_value: 0.8,
            delay_value: 1,
            cutoff_value: 100.0,
            button_width: 40.0,
            button_height: 40.0,
            slider_width: 40.0,
            slider_height: 20.0,
        }
    }
}

// Audio processing unit
struct SignalProcessor {
    manager: AudioManager,
    sound_data: StaticSoundData,
}

// Initializing audio processing unit
impl SignalProcessor {
    fn new(path: &str) -> Self {
        Self {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .expect("Couldn't create Audio Manager"),
            sound_data: StaticSoundData::from_file(path).expect("Couldn't load sound from file"),
        }
    }
}

// To play a sound
fn play_normally(signal_processor: &mut SignalProcessor) {
    let sound_handle = signal_processor
        .manager
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    // Block actions while a sound hasn't finished playing
    while sound_handle.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100));
    }
}

/*
fn play_multiple(signal_processor: &mut SignalProcessor) {
    let sound_handle = signal_processor
        .manager
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    sleep(Duration::from_millis(1000));
    let sound_handle2 = signal_processor
        .manager
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    sleep(Duration::from_millis(1000));
    let sound_handle3 = signal_processor
        .manager
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    while sound_handle3.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100)); // Vérifier périodiquement
    }
}*/

// Play a sound with gradually changing speed overtime
fn play_changed_speed(signal_processor: &mut SignalProcessor, speed_rate: f64) {
    let mut sound_handle = signal_processor
        .manager
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    sound_handle.set_playback_rate(
        speed_rate,
        Tween {
            duration: Duration::from_secs(3),
            ..Default::default()
        },
    );
    while sound_handle.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100));
    }
}

// Play a sound with a low/high pass filter
fn play_with_pass(signal_processor: &mut SignalProcessor, mode: FilterMode) {
    let mut track = signal_processor
        .manager
        .add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(FilterBuilder::new().mode(mode));
            builder
        })
        .expect("Couldn't create a mixer sub-track with a filter");
    let track_handle = track
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    while track_handle.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100));
    }
}

// Play a sound with reverberation. 0 is not reverberated and 1 is infinite reverberation
fn play_with_reverb(signal_processor: &mut SignalProcessor, reverb: f64) {
    let mut track = signal_processor
        .manager
        .add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(ReverbBuilder::new().feedback(reverb));
            builder
        })
        .expect("Couldn't create a mixer sub-track with a filter");
    let track_handle = track
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    while track_handle.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100));
    }
}

// Play distorted sound
fn play_with_distortion(signal_processor: &mut SignalProcessor, mode: DistortionKind) {
    let mut distortion_maker = DistortionBuilder::new().kind(mode);
    distortion_maker = distortion_maker.drive(15.0); // Distortion intensity

    let mut track = signal_processor
        .manager
        .add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(distortion_maker);
            builder
        })
        .expect("Couldn't create a mixer sub-track with a filter");
    let track_handle = track
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    while track_handle.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100));
    }
}

// Replay sound after every duration delay sound has passed
fn play_with_delay(signal_processor: &mut SignalProcessor, duration: Duration) {
    let mut delay_maker = DelayBuilder::new().delay_time(duration);
    delay_maker = delay_maker.feedback(2.0); // Amount of feedback

    let mut track = signal_processor
        .manager
        .add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(delay_maker);
            builder
        })
        .expect("Couldn't create a mixer sub-track with a filter");
    let track_handle = track
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    while track_handle.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100));
    }
}

// Cuts frequencies below the cutoff frequency
fn play_with_cutoff(signal_processor: &mut SignalProcessor, cutoff: f64) {
    let mut cutoff_maker = FilterBuilder::new().cutoff(cutoff);
    cutoff_maker = cutoff_maker.mode(FilterMode::LowPass);

    let mut track = signal_processor
        .manager
        .add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(cutoff_maker);
            builder
        })
        .expect("Couldn't create a mixer sub-track with a filter");
    let track_handle = track
        .play(signal_processor.sound_data.clone())
        .expect("Couldn't play sound from file");
    while track_handle.state() == kira::sound::PlaybackState::Playing {
        sleep(Duration::from_millis(100)); // Vérifier périodiquement
    }
}

// Creates a button in the theme
fn create_button(text: &str) -> egui::Button {
    return egui::Button::new(
        egui::RichText::new(text).color(egui::Color32::from_rgb(255, 255, 255)),
    )
    .fill(Color32::from_rgb(173, 216, 230)) // Bleu clair (RGB)
    .corner_radius(CornerRadius::same(10)) // Coins arrondis
    .stroke(Stroke::new(1.0, Color32::from_rgb(100, 149, 237))); // Contour bleu foncé
}

// Creates a frame in the theme
fn create_frame() -> egui::Frame {
    return Frame::new()
        .stroke(Stroke::new(1.5, Color32::WHITE)) // Bordure blanche
        .fill(Color32::from_rgb(65, 141, 240)) // Fond gris foncé
        .corner_radius(CornerRadius::same(10)) // Coins arrondis
        .inner_margin(egui::Margin::same(1));
}

// App UI description
impl eframe::App for MyEguiApp {
    // Main function, displaying the app elements
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Spacing configuration
                let spacing = ui.spacing_mut();
                spacing.item_spacing = egui::vec2(0.0, 20.0);

                // Base style of the app
                let mut style = (*ctx.style()).clone();
                style
                    .text_styles
                    .insert(TextStyle::Body, FontId::new(20.0, FontFamily::Proportional));
                style.text_styles.insert(
                    TextStyle::Heading,
                    FontId::new(30.0, FontFamily::Proportional),
                );
                style.text_styles.insert(
                    TextStyle::Button,
                    FontId::new(35.0, FontFamily::Proportional),
                );
                style.text_styles.insert(
                    TextStyle::Monospace,
                    FontId::new(16.0, FontFamily::Monospace),
                );

                ctx.set_style(style);

                // App elements

                // Title
                ui.label(
                    egui::RichText::new("Welcome to signal processing toolbox :)")
                        .heading()
                        .color(egui::Color32::from_rgb(255, 255, 255))
                        .strong()
                        .underline(),
                );

                // Sub-title
                ui.label(
                    egui::RichText::new("Pick a mp3 file to start the transformation")
                        .color(egui::Color32::from_rgb(255, 255, 255)),
                );

                // File picker
                if ui
                    .button(RichText::new("Search").color(Color32::WHITE))
                    .clicked()
                {
                    self.file_dialog.pick_file();
                }

                if let Some(path) = self.file_dialog.update(ctx).picked() {
                    if path.extension() != Some(OsStr::new("mp3")) {
                        ui.label(
                            RichText::new("Wrong file extension, try again").color(Color32::RED),
                        );
                    } else {
                        if let Some(path_as_str) = path.to_str() {
                            let mut signal_processor = SignalProcessor::new(path_as_str);
                            /*
                            if signal_processor.sound_data.duration() > Duration::new(10, 0) {
                                ui.label(RichText::new("Too big").color(Color32::RED));
                            } */
                            ui.label(
                                egui::RichText::new("File found file under this path :")
                                    .color(egui::Color32::from_rgb(255, 255, 255)),
                            );
                            ui.label(
                                egui::RichText::new(path.display().to_string())
                                    .color(egui::Color32::from_rgb(255, 255, 255)),
                            );

                            // 2 columns with buttons and sliders
                            ui.columns(2, |columns| {
                                let button = create_button("Play normally");
                                if columns[0]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button,
                                    )
                                    .clicked()
                                {
                                    play_normally(&mut signal_processor);
                                }
                                let button2 = create_button("Play with distortion");

                                if columns[1]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button2,
                                    )
                                    .clicked()
                                {
                                    play_with_distortion(
                                        &mut signal_processor,
                                        DistortionKind::HardClip,
                                    );
                                }

                                let button3 = create_button("Play with changed speed :");
                                if columns[0]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button3,
                                    )
                                    .clicked()
                                {
                                    play_changed_speed(&mut signal_processor, self.speed_value);
                                }

                                columns[1].horizontal(|ui| {
                                    let frame = create_frame();
                                    frame.show(ui, |ui| {
                                        ui.add_sized(
                                            [self.slider_width, self.slider_height],
                                            egui::Slider::new(&mut self.speed_value, 0.0..=3.0)
                                                .text("Speed value")
                                                .text_color(Color32::WHITE),
                                        );
                                    });
                                });

                                let button4 = create_button("Play with reverb :");

                                if columns[0]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button4,
                                    )
                                    .clicked()
                                {
                                    play_with_reverb(&mut signal_processor, self.reverb_value);
                                }

                                columns[1].horizontal(|ui| {
                                    let frame = create_frame();
                                    frame.show(ui, |ui| {
                                        ui.add_sized(
                                            [self.slider_width, self.slider_height],
                                            egui::Slider::new(&mut self.reverb_value, 0.0..=1.0)
                                                .text("Reverb value")
                                                .text_color(Color32::WHITE),
                                        );
                                    });
                                });

                                let button5 = create_button("Play with low pass");

                                if columns[0]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button5,
                                    )
                                    .clicked()
                                {
                                    play_with_pass(&mut signal_processor, FilterMode::LowPass);
                                }

                                let button6 = create_button("Play with high pass");
                                if columns[1]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button6,
                                    )
                                    .clicked()
                                {
                                    play_with_pass(&mut signal_processor, FilterMode::HighPass);
                                }

                                let button7 = create_button("Play with delay :");
                                if columns[0]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button7,
                                    )
                                    .clicked()
                                {
                                    play_with_delay(
                                        &mut signal_processor,
                                        Duration::new(self.delay_value, 0),
                                    );
                                }

                                columns[1].horizontal(|ui| {
                                    let frame = create_frame();
                                    frame.show(ui, |ui| {
                                        ui.add_sized(
                                            [self.slider_width, self.slider_height],
                                            egui::Slider::new(&mut self.delay_value, 1..=5)
                                                .text("Delay value")
                                                .text_color(Color32::WHITE),
                                        );
                                    });
                                });

                                let button8 = create_button("Cutoff frequencies");
                                if columns[0]
                                    .add_sized(
                                        Vec2::new(self.button_width, self.button_height),
                                        button8,
                                    )
                                    .clicked()
                                {
                                    play_with_cutoff(&mut signal_processor, self.cutoff_value);
                                }

                                columns[1].horizontal(|ui| {
                                    let frame = create_frame();
                                    frame.show(ui, |ui| {
                                        ui.add_sized(
                                            [self.slider_width, self.slider_height],
                                            egui::Slider::new(
                                                &mut self.cutoff_value,
                                                0.0..=10000.0,
                                            )
                                            .text("Cutoff frequency")
                                            .text_color(Color32::WHITE),
                                        );
                                    });
                                });
                            });
                        }
                    }
                }
            });
        });
    }
}

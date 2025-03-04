use std::sync::Arc;
use windows::Media::Core::MediaSource;
use windows::{
    core::*,
    Foundation::*,
    Media::Playback::*,
    Media::SpeechSynthesis::*
    ,
};

fn run() -> Result<()> {
    let synthesizer = SpeechSynthesizer::new()?;

    let mut cz_voice = None;
    for v in SpeechSynthesizer::AllVoices()? {
        println!("{:?} {:?}", v.DisplayName(), v.Language());
        match v.Language() {
            Ok(lang) if lang == "cs-CZ" => {
                cz_voice = Some(lang);
            }
            _ => {}
        }
    }

    if cz_voice.is_none() {
        eprintln!("CZ voice is not installed");
        return Ok(());
    }

    println!("Using voice: {}", &cz_voice.unwrap());

    let text = "Ahoj, toto je hlas generovaný Windowsem z Rastu a umí i háčky a čárky";
    println!("Speaking: {}", text);

    let stream = synthesizer.SynthesizeTextToStreamAsync(&HSTRING::from(text))?.get()?;
    let media_source = MediaSource::CreateFromStream(&stream, &HSTRING::from(""))?;

    let player = MediaPlayer::new()?;
    player.SetSource(&media_source)?;
    player.Play()?;

    let media_ended = Arc::new(std::sync::Mutex::new(false));
    let media_ended_clone = Arc::clone(&media_ended);

    let token = player.MediaEnded(&TypedEventHandler::new(move |_, _| {
        let mut ended = media_ended_clone.lock().unwrap();
        *ended = true;
        Ok(())
    }))?;

    println!("Waiting for playback to complete...");
    loop {
        if *media_ended.lock().unwrap() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Clean up the event handler
    player.RemoveMediaEnded(token)?;

    println!("Playback completed!");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}
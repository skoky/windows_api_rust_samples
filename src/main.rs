mod volume;

use std::process::exit;
use crate::volume::set_mic_volume;
use windows::core::{h, HSTRING};
use windows::Devices::Geolocation::{CivicAddress, Geocoordinate, GeolocationAccessStatus, Geolocator, Geoposition, PositionStatus};
use windows::Globalization;
use windows::Globalization::Language;
use windows::Media::SpeechRecognition::{SpeechRecognitionCompilationResult, SpeechRecognitionConfidence, SpeechRecognitionResult, SpeechRecognitionResultStatus, SpeechRecognizer, SpeechRecognizerTimeouts};
use windows::Win32::System::Power::SYSTEM_POWER_STATUS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get_mic_volume();
    if let Err(mic_err) = set_mic_volume() {
        println!("Mic set error {:?}", mic_err);
    }
    let _ = futures::executor::block_on(print_geolocation());
    // let wifi = futures::executor::block_on(report_wifi());
    // println!("Connected wifi: {}", wifi.unwrap_or("?,?".to_string()));
    get_power_source();
    if let Err(e) = futures::executor::block_on(speech()) {
        println!("Err {:?}", e);
    }
    Ok(())
}

async fn print_geolocation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Position start");
    let locator = Geolocator::new()?;
    let status: GeolocationAccessStatus = Geolocator::RequestAccessAsync()?.get()?;
    println!("Access {:?}", status);
    if status == GeolocationAccessStatus::Allowed {
        let position: Geoposition = locator.GetGeopositionAsync()?.get()?;
        let acc = locator.DesiredAccuracyInMeters()?;

        let ls: PositionStatus = locator.LocationStatus()?;

        println!("{:?}", ls);

        let address: CivicAddress = position.CivicAddress()?;
        let coordinates: Geocoordinate = position.Coordinate()?;
        println!("Position: {:?}, {:?} / accuracy meters: {:?}", address.City()?, coordinates, acc);
    } else {
        println!("Geo Status: {:?}/Denied", status);
    }
    Ok(())
}

async fn speech() -> Result<(), Box<dyn std::error::Error>> {
    // windows::Media::SpeechSynthesis::
    // let speech: SpeechRecognizer = SpeechRecognizer::new()?;
    // let vector = Vector::<HSTRING>::new()?;

    // let languages = Language::GetMuiCompatibleLanguageListFromLanguageTags("cs-CZ")?;
    // for l in languages {
    //     println!("Language: {:?}", l);
    // }
    let czech_language: Globalization::Language = Language::CreateLanguage(h!("en-us"))?;
    let speech = SpeechRecognizer::Create(&czech_language)?;

    let language: Language = speech.CurrentLanguage()?;
    let language2: Language = SpeechRecognizer::SystemSpeechLanguage()?;
    println!("Languages: {:?} / {:?}", language.DisplayName()?, language2.DisplayName()?);


    let c: SpeechRecognitionCompilationResult = speech.CompileConstraintsAsync()?.get()?;
    // speech.ContinuousRecognitionSession()
    println!("Init status {:?}", c.Status()?);

    let timeouts: SpeechRecognizerTimeouts = speech.Timeouts()?;
    println!("Timeouts silence: {:?} bable: {:?}", timeouts.EndSilenceTimeout()?, timeouts.BabbleTimeout()?);

    loop {
        println!("Listening... Say \"exit\" to stop");

        // let result: IAsyncOperation<SpeechRecognitionResult> = speech.RecognizeWithUIAsync()?;
        let result: SpeechRecognitionResult = speech.RecognizeAsync()?.get()?;

        let sentence: HSTRING = result.Text()?;
        let confidence: SpeechRecognitionConfidence = result.Confidence()?;
        let confidence_text = match confidence {
            SpeechRecognitionConfidence::High => "High",
            SpeechRecognitionConfidence::Medium => "Medium",
            SpeechRecognitionConfidence::Low => "Low",
            _ => "?"
        };
        let status: SpeechRecognitionResultStatus = result.Status()?;
        println!(">>> {:?}({}): {}", status, confidence_text, sentence);
        if sentence.to_string_lossy().to_ascii_lowercase() == "exit" {
            exit(0);
        }
    }
}


fn get_power_source() {
    unsafe {
        let mut system_power_status: SYSTEM_POWER_STATUS = std::mem::zeroed();

        // Get the current power status
        let result = windows::Win32::System::Power::GetSystemPowerStatus(&mut system_power_status);

        if result.is_ok() {
            println!("AC connected: {}", system_power_status.ACLineStatus == 1 );
            println!("Battery below 33%: {}", system_power_status.BatteryFlag == 2);
            println!("Battery charging: {}", system_power_status.BatteryFlag == 8);
            println!("Battery Life remaining (mins): {}", system_power_status.BatteryLifeTime/60
            );
            println!("System Status Flag: {}", system_power_status.SystemStatusFlag);
        } else {
            println!("Failed to get power status.");
        }
    }
}
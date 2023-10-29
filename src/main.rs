use std::process::exit;

use windows::core::HSTRING;
use windows::Devices::Geolocation::{CivicAddress, Geocoordinate, GeolocationAccessStatus, Geolocator, Geoposition, PositionStatus};
use windows::Devices::WiFi::{WiFiAdapter, WiFiAvailableNetwork, WiFiNetworkReport};
use windows::Globalization::Language;
use windows::Media::SpeechRecognition::{SpeechRecognitionCompilationResult, SpeechRecognitionConfidence, SpeechRecognitionResult, SpeechRecognitionResultStatus, SpeechRecognizer, SpeechRecognizerTimeouts};
use windows::Networking::Connectivity::NetworkConnectivityLevel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = futures::executor::block_on(print_geolocation());
    let wifi = futures::executor::block_on(report_wifi());
    println!("Connected wifi: {}", wifi.unwrap_or("?,?".to_string()));
    let _ = futures::executor::block_on(speech());
    Ok(())
}

async fn print_geolocation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Position start");
    let locator = Geolocator::new()?;
    let status: GeolocationAccessStatus = Geolocator::RequestAccessAsync()?.await?;
    println!("Access {:?}", status);
    if status == GeolocationAccessStatus::Allowed {
        let position: Geoposition = locator.GetGeopositionAsync()?.await?;
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
    let speech: SpeechRecognizer = SpeechRecognizer::new()?;

    let language: Language = speech.CurrentLanguage()?;
    let language2: Language = SpeechRecognizer::SystemSpeechLanguage()?;
    println!("Languages: {:?} / {:?}", language.DisplayName()?, language2.DisplayName()?);

    let c: SpeechRecognitionCompilationResult = speech.CompileConstraintsAsync()?.await?;
    // speech.ContinuousRecognitionSession()
    println!("Init status {:?}", c.Status()?);

    let timeouts: SpeechRecognizerTimeouts = speech.Timeouts()?;
    println!("Timeouts silence: {:?} bable: {:?}", timeouts.EndSilenceTimeout()?, timeouts.BabbleTimeout()?);

    loop {
        println!("Listening... Say \"exit\" to stop");

        // let result: IAsyncOperation<SpeechRecognitionResult> = speech.RecognizeWithUIAsync()?;
        let result: SpeechRecognitionResult = speech.RecognizeAsync()?.await?;

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
        if sentence.to_string_lossy() == "exit" {
            exit(0);
        }
    }
}

async fn report_wifi() -> Result<String, Box<dyn std::error::Error>> {
    let nm = WiFiAdapter::RequestAccessAsync()?;
    println!("{:?}", nm.get()?);
    let adapter = WiFiAdapter::FindAllAdaptersAsync()?;

    for a in adapter.get()? {
        let adapter: WiFiAdapter = a;
        let na = adapter.NetworkAdapter()?;
        let np = match na.GetConnectedProfileAsync()?.await {
            Ok(profile) => profile,
            Err(e) => {
                println!("Wifi not connected");
                return Err(Box::new(e))
            }
        };
        let nn = np.GetNetworkNames()?;

        let connected_wifi = if nn.Size()? == 1 {
            nn.GetAt(0).map(|n| n.to_string()).unwrap_or("---".to_string())
        } else if nn.Size()? > 1 {
            nn.GetAt(0).map(|n| n.to_string()).unwrap_or("---".to_string())
        } else {
            println!("No connected wifi found");
            "---".to_string()
        };
        let report: WiFiNetworkReport = adapter.NetworkReport()?;
        let cp = match np.GetNetworkConnectivityLevel()? {
            NetworkConnectivityLevel::InternetAccess => "Internet_connected".to_string(),
            _ => "Internet_NOT_connected".to_string()
        };
        for network in report.AvailableNetworks()? {
            let n: WiFiAvailableNetwork = network;
            let prc = n.SignalBars()? * 25;
            if n.Ssid()? == connected_wifi {
                println!("Connected Wifi: {:?} {}% {}", n.Ssid()?, prc, cp);
                return Ok(format!("{},{}%,{}", connected_wifi, prc, cp))
            }
        }
    }
    Ok("?,?".to_string())
}

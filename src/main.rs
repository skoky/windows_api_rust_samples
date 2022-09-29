use std::process::exit;

use windows::core::HSTRING;
use windows::Devices::WiFi::{WiFiAdapter, WiFiAvailableNetwork, WiFiNetworkReport};
use windows::Foundation::{AsyncStatus, IAsyncOperation};
use windows::Globalization::Language;
use windows::Media::SpeechRecognition::{SpeechRecognitionCompilationResult, SpeechRecognitionConfidence, SpeechRecognitionResult, SpeechRecognizer, SpeechRecognizerTimeouts};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = futures::executor::block_on(report_wifi());
    futures::executor::block_on(speech())
}

async fn speech() -> Result<(), Box<dyn std::error::Error>> {
    let speech: SpeechRecognizer = SpeechRecognizer::new()?;

    let language: Language = speech.CurrentLanguage()?;
    let language2: Language = SpeechRecognizer::SystemSpeechLanguage()?;
    println!("Languages: {:?} / {:?}", language.DisplayName()?, language2.DisplayName()?);
    let c: IAsyncOperation<SpeechRecognitionCompilationResult> = speech.CompileConstraintsAsync()?;
    // speech.ContinuousRecognitionSession()

    let timeouts: SpeechRecognizerTimeouts = speech.Timeouts()?;
    println!("Timeouts silence: {:?} bable: {:?}", timeouts.EndSilenceTimeout()?, timeouts.BabbleTimeout()?);

    // TODO better await
    loop {
        let status: AsyncStatus = c.Status()?;
        if status == AsyncStatus::Completed {
            break;
        }
    }

    loop {
        println!("Listening... Say \"exit\" to stop");

        // let result: IAsyncOperation<SpeechRecognitionResult> = speech.RecognizeWithUIAsync()?;
        let result: IAsyncOperation<SpeechRecognitionResult> = speech.RecognizeAsync()?;

        // TODO better await
        loop {
            let status: AsyncStatus = result.Status()?;
            if status == AsyncStatus::Completed {
                break;
            }
        }
        let x: SpeechRecognitionResult = result.get()?;
        let sentence: HSTRING = x.Text()?;
        let confidence: SpeechRecognitionConfidence = x.Confidence()?;
        let confidence_text = match confidence {
            SpeechRecognitionConfidence::High => "High",
            SpeechRecognitionConfidence::Medium => "Medium",
            SpeechRecognitionConfidence::Low => "Low",
            _ => "?"
        };
        println!(">>> ({}): {}", confidence_text, sentence);
        if sentence.to_string_lossy() == "exit" {
            exit(0);
        }
    }

}

async fn report_wifi() -> Result<(), Box<dyn std::error::Error>> {
    let nm = WiFiAdapter::RequestAccessAsync()?;
    println!("{:?}", nm.get()?);
    let adapter = WiFiAdapter::FindAllAdaptersAsync()?;
    for a in adapter.get()? {
        let adapter: WiFiAdapter = a;
        let report: WiFiNetworkReport = adapter.NetworkReport()?;
        for network in report.AvailableNetworks()? {
            let n: WiFiAvailableNetwork = network;
            println!("Network: {:?} {}kHz Signal: {} Timestamp: {:?}", n.Ssid()?,
                     n.ChannelCenterFrequencyInKilohertz()?, n.SignalBars()?,
                     n.Uptime()?);
        }
    }
    Ok(())
}

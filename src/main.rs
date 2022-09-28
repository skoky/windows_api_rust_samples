use windows::Devices::WiFi::{WiFiAdapter, WiFiAvailableNetwork, WiFiNetworkReport};
use windows::Foundation::{AsyncStatus, IAsyncOperation};
use windows::Media::SpeechRecognition::{SpeechRecognitionCompilationResult, SpeechRecognitionResult, SpeechRecognizer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = futures::executor::block_on(report_wifi());
    futures::executor::block_on(speech())
}

async fn speech() -> Result<(), Box<dyn std::error::Error>> {
    let speech: SpeechRecognizer = SpeechRecognizer::new()?;
    let c: IAsyncOperation<SpeechRecognitionCompilationResult> = speech.CompileConstraintsAsync()?;

    // TODO better await
    loop {
        let status: AsyncStatus = c.Status()?;
        if status == AsyncStatus::Completed {
            break;
        }
    }
    println!("Listening...");

    let result: IAsyncOperation<SpeechRecognitionResult> = speech.RecognizeAsync()?;

    // TODO better await
    loop {
        let status: AsyncStatus = result.Status()?;
        if status == AsyncStatus::Completed {
            break;
        }
    }
    let x: SpeechRecognitionResult = result.get()?;
    println!(">>> {}", x.Text()?);
    Ok(())
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

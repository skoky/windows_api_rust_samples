use windows::Devices::WiFi::{ WiFiAdapter, WiFiAvailableNetwork, WiFiNetworkReport};
use windows::Networking::Connectivity::NetworkAdapter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    futures::executor::block_on(report_wifi())
}

async fn report_wifi() -> Result<(),Box<dyn std::error::Error>>{
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

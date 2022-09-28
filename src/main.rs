use windows::Devices::WiFi::{WiFiAccessStatus, WiFiAdapter, WiFiAvailableNetwork, WiFiNetworkReport};
use windows::Foundation::Collections::IVectorView;
use windows::Foundation::IAsyncOperation;
use windows::Networking::Connectivity::NetworkAdapter;

fn main() {
    futures::executor::block_on(main_async())
}

fn report() {
    println!("Report Wifi")
}

async fn main_async() {
    let nm = WiFiAdapter::RequestAccessAsync().unwrap();
    println!("{:?}", nm.get().unwrap());
    let adapter = WiFiAdapter::FindAllAdaptersAsync().unwrap();
    for a in adapter.get().unwrap() {
        let adapter: WiFiAdapter = a;

        let na: NetworkAdapter = adapter.NetworkAdapter().unwrap();
        println!("{:?}", na);
        // let report: WiFiNetworkReport = adapter.AvailableNetworksChanged(report).unwrap();
        let report: WiFiNetworkReport = adapter.NetworkReport().unwrap();
        for network in report.AvailableNetworks().unwrap() {
            let n: WiFiAvailableNetwork = network;
            println!("Network: {:?} {}kHz Signal: {} Timestamp: {:?}", n.Ssid().unwrap(),
                     n.ChannelCenterFrequencyInKilohertz().unwrap(), n.SignalBars().unwrap(),
            n.Uptime().unwrap());
        }
    }
}

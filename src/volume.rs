use windows::{
    core::*,
    Win32::{
        Media::Audio::*,
        System::Com::*,
    },
};

pub fn set_mic_volume() -> Result<()> {
    unsafe {
        let _r = CoInitializeEx(None, COINIT_MULTITHREADED);

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let device = enumerator.GetDefaultAudioEndpoint(eCapture, eConsole)?;

        //  let endpoint_volume: Endpoints::IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;

        // endpoint_volume.SetMasterVolumeLevelScalar(1.0, std::ptr::null_mut())?;
        // if let Ok(new_vol) = endpoint_volume.GetMasterVolumeLevelScalar() {
        //     println!("Microphone volume set to {:?}%", new_vol * 100f32);
        // }
    }

    Ok(())
}

// pub fn get_mic_volume() -> Result<(f32)> {
//     unsafe {
//         let _r = CoInitializeEx(None, COINIT_MULTITHREADED);
//
//         let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
//
//         let device = enumerator.GetDefaultAudioEndpoint(eCapture, eConsole)?;
//
//         let endpoint_volume: Endpoints::IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;
//
//         match endpoint_volume.GetMasterVolumeLevelScalar() {
//             Ok(volume) => {
//                 println!("Mic volume is {}%", volume*100f32);
//                 Ok(volume)
//             }
//             Err(e) => {
//                 println!("Get volume error {:?}", e);
//                 Err(e)
//             }
//         }
//
//     }
//
// }


use std::ffi::CStr;
use std::os::raw::c_char;
use rodio::{Decoder, OutputStream, Sink};
use std::thread;
use std::os::raw::c_int;

pub const UI_CLICK_SOUND: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/sounds/ui-click.mp3"));
pub const EMPTY_TRASH_SOUND: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/sounds/empty-trash.mp3"));

#[no_mangle]
pub extern "C" fn play_ui_sound(sound_file: *const c_char, muted: c_int) {

    
    

    let sound_file = unsafe { CStr::from_ptr(sound_file).to_str().unwrap() };

    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();


    let sound_data = match sound_file {
        "ui-click.mp3" => UI_CLICK_SOUND,
        "empty-trash.mp3" => EMPTY_TRASH_SOUND,
        _ => {
            eprintln!("sound file {} not signed", sound_file);
            return;
        }
    };
    let source = Decoder::new(std::io::Cursor::new(sound_data)).unwrap();

    sink.append(source);
    sink.play();

    if muted == 0 {
        sink.set_volume(0.0);
    } else {
        sink.set_volume(1.0);
    }

    

    thread::park();

  
}


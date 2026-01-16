use clap::{Args, Parser};
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[command(flatten)]
    onoff: OnOff,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct OnOff {
    #[arg(long)]
    on: bool,

    #[arg(long)]
    off: bool,
}

fn main() {
    let args = Cli::parse();

    let key = match (args.onoff.on, args.onoff.off) {
        (true, _) => VK_IME_ON,
        (_, true) => VK_IME_OFF,
        _ => unreachable!(),
    };

    unsafe {
        SendInput(
            &[
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: key,
                            wScan: 0,
                            dwFlags: KEYBD_EVENT_FLAGS(0),
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: key,
                            wScan: 0,
                            dwFlags: KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ],
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

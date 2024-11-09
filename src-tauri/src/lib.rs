use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use windows::Win32::{
    Foundation::BOOL,
    Graphics::Gdi::{EnumDisplayDevicesW, DISPLAY_DEVICEW},
};

// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaydevicesw?redirectedfrom=MSDN
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaysettingsw?redirectedfrom=MSDN
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw?redirectedfrom=MSDN

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut idevnum = 0;

    loop {
        let display_device: *mut DISPLAY_DEVICEW = &mut DISPLAY_DEVICEW {
            cb: std::mem::size_of::<DISPLAY_DEVICEW>() as u32,
            ..Default::default()
        };

        match unsafe { EnumDisplayDevicesW(None, idevnum, display_device, 0) } {
            BOOL(0) => break,
            _ => unsafe {
                println!("{:?}", *display_device);
            },
        };

        idevnum += 1;
    }

    tauri::Builder::default()
        .setup(|app| {
            let pause_i = MenuItem::with_id(app, "pause", "Pause", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&pause_i, &quit_i])?;

            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "pause" => {
                        println!("pause menu item was clicked");
                    }
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

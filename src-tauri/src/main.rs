
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::SystemTrayEvent;
use tauri::{Manager, Window};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_clipboard;
use tauri_plugin_wallpaper::Wallpaper;

////////////////////////////////////////////////////////////////////////////
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

use rdev::{grab, Event, EventType,Button,simulate, Key, SimulateError,};
#[tauri::command]
fn wheelclick(window: Window){
  tauri::async_runtime::spawn(async move {
    let _callback = move |event: Event| -> Option<Event> {
        if let EventType::ButtonPress(Button::Middle) = event.event_type {
            let s = format!("{:?}",event.event_type);
            window.emit("wheel-click", Payload { message: s.into()}).unwrap();
            None
        }else if let EventType::ButtonRelease(Button::Middle) = event.event_type {
          let s = format!("{:?}",event.event_type);
          window.emit("wheel-click", Payload { message: s.into()}).unwrap();
          None
        }else if let EventType::MouseMove { x:_, y:_} = event.event_type {
          let s = format!("{:?}",event.event_type);
          window.emit("mouse-move", Payload { message: s.into()}).unwrap();
          return Some(event);
        }
        else { Some(event) }
    };
    // let _a = grab(_callback);
    if let Err(error) = grab(_callback) {
      println!("Error: {:?}", error);
    }else{
      // window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
    }
  });
}

////////////////////////////////////////////////////////////////////////////
#[tauri::command]
fn screen(){
    send(&EventType::KeyPress(Key::PrintScreen));
    send(&EventType::KeyRelease(Key::PrintScreen));
}

fn send(event_type: &EventType) {
  let delay = time::Duration::from_millis(20);
  match simulate(event_type) {
      Ok(()) => (),
      Err(SimulateError) => {
          println!("We could not send {:?}", event_type);
      }
  }
  // Let ths OS catchup (at least MacOS)
  thread::sleep(delay);
}

////////////////////////////////////////////////////////////////////////////
// tauri = { version = "1.4", features = [ "system-tray", "api-all", "devtools"] }    cargo.toml
fn main() {

    tauri::Builder::default()
        // .setup(|app| {
        //   #[cfg(debug_assertions)] // only include this code on debug builds
        //   {
        //     let window = app.get_window("main").unwrap();
        //     window.open_devtools();
        //     window.close_devtools();
        //   }
        //   Ok(())
        // })
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .plugin(tauri_plugin_clipboard::init())
        .plugin(Wallpaper::init())
        .system_tray(systemtray())
        .on_system_tray_event(|app,event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
              } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
              }
              SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
              } => {
                println!("system tray received a right click");
              }
              SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
              } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
              }
              SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                  "quit" => {
                    std::process::exit(0);
                  }
                  "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                  }
                  "open"=> {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                  }
                  _ =>{}
                }
              }
              _ => {}
        })
        .invoke_handler(tauri::generate_handler![wheelclick,netspeed,screen,setwallpaper,geticon,systeminfo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

////////////////////////////////////////////////////////////////////////////

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
fn systemtray()->SystemTray{
    // 创建菜单项
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let open = CustomMenuItem::new("open".to_string(), "打开");

    // 挂载菜单项到系统托盘菜单
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(open);
    // 挂在系统托盘菜单到系统托盘
    let tray = SystemTray::new().with_menu(tray_menu);
    return tray;
}

////////////////////////////////////////////////////////////////////////////

use std::{thread, time};
use sysinfo::{NetworkExt, System, SystemExt};
#[tauri::command]
fn netspeed(window: Window){
  tauri::async_runtime::spawn(async move {
    let mut sys = System::new_all();
    
    loop {
      sys.refresh_networks();
      let mut s= String::from(" ");
      for (interface_name, data) in sys.networks() {
        s = format!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
      }
      // println!("{}",s);
      window.emit("netspeed", Payload { message: s.into()}).unwrap();

      let ten_millis = time::Duration::from_millis(1000);
      thread::sleep(ten_millis);
    } 
  });
}
////////////////////////////////////////////////////////////////////////////

#[tauri::command]
fn systeminfo(){
  
}

////////////////////////////////////////////////////////////////////////////

#[tauri::command]
fn setwallpaper(src:String){
    wallpaper::set_from_path(&src).unwrap();
} 

////////////////////////////////////////////////////////////////////////////

// C:\\Program Files (x86)\\NetEase\\CloudMusic\\cloudmusic.exe
use winres_edit::{Resources,Id};
use std::path::Path;
use image::{guess_format,ImageFormat};
#[tauri::command] 
fn geticon(path:String)->Vec<Vec<String>>{
    let mut arr = Vec::new();
    let mut group_icon = Vec::new();
    let mut ico = Vec::new();
    let mut resources = Resources::new(&Path::new(&path));
    resources.load().expect("Unable to load resources");
    resources.open().expect("Unable to open resource file for updates");

    for i in 1..500 {
      let a  = resources.find(Id::Integer(14),Id::Integer(i));
      match a {
          Some(b) => {
            let str:String = format!("{{\"name\":\"{:?}\",\"data\":\"{:?}\"}}",b.name,b.encoded.lock().unwrap());
            group_icon.push(str)
          },
          _=> {}   
      }   
    }   

    for i in 1..500 {
      let a  = resources.find(Id::Integer(3),Id::Integer(i));
      match a {
          Some(b) => {
            let ext = guess_format(&b.encoded.lock().unwrap());

            let ext = match ext {  
                Ok(e) => e,
                Err(_)=>ImageFormat::Ico,
            }; 

            if ext == ImageFormat::Png { 
              let str = format!("{{\"name\":\"{:?}\",\"type\":\"png\",\"data\":\"{:?}\"}}",b.name,b.encoded.lock().unwrap());
              ico.push(str);
            }else{
              let str = format!("{{\"name\":\"{:?}\",\"type\":\"ico\",\"data\":\"{:?}\"}}",b.name,b.encoded.lock().unwrap());
              ico.push(str);
            }
 
          }, 
          _=> {}   
      }
    }  
 
    arr.push(group_icon);
    arr.push(ico);
    resources.close();
    return arr;
}




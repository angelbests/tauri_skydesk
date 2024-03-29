
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_autostart::MacosLauncher;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem,SystemTray,Manager, Window,SystemTrayEvent};
use rdev::{grab, Event, EventType,Button,simulate, Key, SimulateError,};
////////////////////////////////////////////////////////////////////////////
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


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
  use std::{thread, time};
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
        .invoke_handler(tauri::generate_handler![wheelclick,netspeed,screen,setwallpaper,geticon,systeminfo,getlnk,getlnk2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

////////////////////////////////////////////////////////////////////////////


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

/////////////////////////////////////////////////////////////
/// 获取lnk文件关联的exe文件路径
// #[tauri::command]
// fn getlnk(path:String)->String{
//   println!("{:?}",path);
//   let shortcut = lnk::ShellLink::open(path.clone()).unwrap();
//   let mut ico_location = String::from("");
//   let mut local_base_path = String::from("");
//   if let Some(icolocation) = shortcut.icon_location() {
//     ico_location = icolocation.to_string();
//   }
//   if let Some(linkinfo) = shortcut.link_info() {
//     if let Some(exepath) = linkinfo.local_base_path() {
//       local_base_path = exepath.to_string();
//     }
//   }
//   return format!("{{\"ico_location\":{:?},\"local_base_path\":{:?}}}",ico_location,local_base_path);
//   // return  String::from("value");
// }

#[tauri::command]
fn getlnk(path:String)->String{
  println!("{:?}",path);
  let shortcut = lnk::ShellLink::open(path.clone()).unwrap();
  let mut ico_location = String::from("");
  let mut local_base_path = String::from("");
  if let Some(icolocation) = shortcut.icon_location() {
    ico_location = icolocation.to_string();
  }
  if let Some(linkinfo) = shortcut.link_info() {
    if let Some(exepath) = linkinfo.local_base_path() {
      local_base_path = exepath.to_string();
    }
  }
  return format!("{{\"ico_location\":{:?},\"local_base_path\":{:?}}}",ico_location,local_base_path);
}



#[tauri::command]
async fn getlnk2(path:String)->String{
  use lnk_parser::LNKParser;
  let mut target_full_path = String::from("");
  let mut ico_location = String::from("");
  let mut local_base_path = String::from("");
  let lnk_file = LNKParser::from_path(&path);
  match lnk_file {
      Ok(lnkfile) =>{
        if let Some(targetfullpath) = lnkfile.get_target_full_path() {
           println!("exe:{:?}",targetfullpath);
          target_full_path = targetfullpath.to_string();
        }
        if let Some(icolocation) = lnkfile.get_icon_location() {
           println!("ico:{:?}",icolocation.string);
          ico_location = icolocation.string.to_string()
        }
        if let Some(linkinfo) = lnkfile.get_link_info() {
          if let Some(localbasepath) = &linkinfo.local_base_path {
            local_base_path = localbasepath.to_string();
            println!("exe2:{:?}",localbasepath);
          }
        }
      },
      Err(ref e) => {
        println!("{:?}",e);
      }
  }
  return format!("{{\"ico_location\":{:?},\"local_base_path\":{:?},\"target_full_path\":{:?}}}",ico_location,local_base_path,target_full_path);
}


// use parselnk::Lnk;
// use std::fs::File;
// #[tauri::command]
// fn getlnk3(path:String)->String{
//   let mut file = File::open(path).unwrap();
//   let mut str = String::from("");

//   let mut ico_location = String::from("");
//   let mut local_base_path = String::from("");
//   match Lnk::new(&mut file) {
//       Ok(lnk) =>{
//         if let Some(localbasepath) = lnk.link_info.local_base_path {
//           local_base_path = localbasepath;
//           println!("1:{:?}",local_base_path);
//         }

//         if let Some(icolocation) = lnk.string_data.icon_location {
//           ico_location = format!("{:?}",icolocation);
//           println!("2:{:?}",ico_location);
//         }
//         // println!("1:{:?}",lnk.link_info.local_base_path);
//         // println!("2:{:?}",lnk.string_data.icon_location);
//         str = format!("{{\"ico_location\":{:?},\"local_base_path\":{:?}}}",ico_location,local_base_path);
//         println!("{:?}",str);
//       },
//       Err(ref e) => {
//         println!("{:?}",e);
//       }
//   }

//   return str;
// }
////////////////////////////////////////////////////////////////////////////
// 网速

#[tauri::command]
fn netspeed(window: Window){
  use std::{thread, time};
  use sysinfo::{NetworkExt, System, SystemExt, NetworksExt};
  tauri::async_runtime::spawn(async move {
    let mut sys = System::new_all();
    let networks = sys.networks_mut();
    loop {
      networks.refresh_networks_list();
      let mut arr:Vec<String> = Vec::new();
      for (interface_name, data) in networks.iter() {
        let s = format!("{}: {}/{} ||||", interface_name, data.received(), data.transmitted());
        arr.push(s);
      }
      window.emit("netspeed", Payload { message: arr.into_iter().collect()}).unwrap();

      let ten_millis = time::Duration::from_millis(1000);
      thread::sleep(ten_millis);
    } 
  });
}
////////////////////////////////////////////////////////////////////////////

#[tauri::command]
fn systeminfo(window: Window){
  use std::{thread, time};
  use sysinfo::{System, SystemExt,ProcessExt, DiskExt, CpuExt, UserExt};
  tauri::async_runtime::spawn(async move {
    let mut sys = System::new_all();

    loop { 
      sys.refresh_all();
  
      // 磁盘信息
      let mut arr:Vec<String> = Vec::new();
      for disk in sys.disks() {
        let s = format!("{{\"mount_point\":{:?},\"file_system\":{:?},\"kind\":{:?},\"total_space\":{:?},\"available_space\":{:?}}}",disk.mount_point(),disk.file_system(),disk.mount_point(),disk.total_space(),disk.available_space());
        arr.push(s);
      }
      window.emit("disk", &arr).unwrap();
    
      // RAM 物理内存
      let mut s = format!("{{\"totalmemory\":{:?},\"usedmemory\":{:?}}}",sys.total_memory(),sys.used_memory());
      window.emit("memory", s).unwrap();

      // swap 虚拟内存
      s = format!("{{\"totalswap\":{:?},\"usedswap\":{:?}}}",sys.total_swap(),sys.used_swap());
      window.emit("swap", s).unwrap();

      // println!("{:?}",sys.global_cpu_info());
      arr.clear();
      for cpu in sys.cpus() {
        s = format!("{{\"name\":{:?},\"usage\":{:?},\"vender\":{:?},\"brand\":{:?},\"frequery\":{:?}}}",cpu.name(),cpu.cpu_usage(),cpu.vendor_id(),cpu.brand(),cpu.frequency());
        arr.push(s);
      }
      window.emit("cpu", &arr).unwrap();

      //进程列表
      arr.clear();
      for (pid, process) in sys.processes() {
          let mut parentpid = String::from("");
          let mut userid =String::from("");
          match process.parent() {
              Some(pid)=>{
                parentpid = pid.to_string();
              },
              _=>{}
          }
          match process.user_id() {
            Some(user)=>{
              userid = user.to_string();
            },
            _=>{}
          }
          s = format!("{{\"pid\":\"{:?}\",\"name\":\"{:?}\",\"cmd\":\"{:?}\",\"path\":\"{:?}\",\"runpath\":\"{:?}\",\"rootpath\":\"{:?}\",\"memory\":\"{:?}\",\"virtualmemory\":\"{:?}\",\"parentpid\":\"{:?}\",\"status\":\"{:?}\",\"starttime\":\"{:?}\",\"runtime\":\"{:?}\",\"cpu\":\"{:?}\",\"user\":\"{:?}\"}}", 
          pid.to_string(),process.name(),process.cmd(),process.exe(),process.cwd(),process.root(),process.memory(),process.virtual_memory(),parentpid
          ,process.status(),process.start_time(),process.run_time(),process.cpu_usage(),userid);
          arr.push(s);
      }
      window.emit("process", &arr).unwrap();
      
      arr.clear();
      for user in sys.users() {
        s = format!("{{\"name\":\"{:?}\",\"id\":\"{:?}\"}}",user.name(),user.id().to_string());
        arr.push(s);
      }
      window.emit("user", &arr).unwrap();

      // 系统版本
      let mut host_name = String::from("");
      match sys.host_name() {
          Some(ss) =>{
            host_name = ss
          },
          _=>{}
      }

      let mut kernel_version = String::from("");
      match sys.kernel_version() {
          Some(ss)=>{
            kernel_version = ss
          },
          _=>{}
      }

      match sys.long_os_version() {
          Some(ss) =>{
            let s = format!("{{\"version\":{:?}}}",ss+" "+&kernel_version+" "+&host_name);
            window.emit("version", s).unwrap();
          },
          _=>{}
      }

      let ten_millis = time::Duration::from_millis(1000);
      thread::sleep(ten_millis);
    }

  });
}

////////////////////////////////////////////////////////////////////////////
// 设置为背景桌面 
#[tauri::command]
fn setwallpaper(src:String){
    wallpaper::set_from_path(&src).unwrap();
} 

////////////////////////////////////////////////////////////////////////////

// C:\\Program Files (x86)\\NetEase\\CloudMusic\\cloudmusic.exe
// 图标提取
#[cfg(any(windows))]

#[tauri::command] 
fn geticon(path:String)->Vec<Vec<String>>{
    use winres_edit::{Resources,Id};
    use std::path::Path;
    use image::{guess_format,ImageFormat};
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




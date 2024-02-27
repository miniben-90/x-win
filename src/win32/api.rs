#![deny(unused_imports)]

use windows::{
  core::w,
  Win32::System::Variant::{VARIANT, VARIANT_0, VARIANT_0_0, VARIANT_0_0_0, VT_BSTR, VT_I4},
};

use crate::common::{
  api::API,
  x_win_struct::{
    process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use windows::Win32::{
  Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED},
  System::{ProcessStatus::GetProcessMemoryInfo, StationsAndDesktops::EnumDesktopWindows},
  UI::{
    Accessibility::CUIAutomation,
    WindowsAndMessaging::{
      GetWindowInfo, GetWindowPlacement, IsWindow, IsWindowVisible, SW_SHOWMAXIMIZED, WINDOWINFO,
      WINDOWPLACEMENT, WS_ACTIVECAPTION, WS_CAPTION, WS_CHILD, WS_EX_TOOLWINDOW,
    },
  },
};
use windows::{
  core::{PCWSTR, PWSTR},
  Win32::{
    Foundation::HWND,
    Foundation::{CloseHandle, BOOL, LPARAM, RECT},
    Foundation::{HANDLE, MAX_PATH},
    Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW},
    System::{
      Com::*,
      ProcessStatus::PROCESS_MEMORY_COUNTERS,
      Threading::{
        GetProcessId, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
      },
    },
    UI::Accessibility::*,
    UI::WindowsAndMessaging::{
      EnumChildWindows, GetForegroundWindow, GetWindowRect, GetWindowTextW,
      GetWindowThreadProcessId,
    },
  },
};

#[derive(Debug)]
struct LangCodePage {
  pub w_language: u16,
  pub w_code_page: u16,
}

pub struct WindowsAPI {}

/**
 * Impl. for windows system
 */
impl API for WindowsAPI {
  fn get_active_window(&self) -> WindowInfo {
    let hwnd = unsafe { GetForegroundWindow() };
    get_window_information(hwnd)
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    let mut results: Vec<WindowInfo> = Vec::new();
    let mut open_windows: Vec<HWND> = Vec::new();

    let lparam = unsafe {
      std::mem::transmute::<*mut c_void, LPARAM>(&mut open_windows as *mut Vec<HWND> as *mut c_void)
    };
    unsafe {
      let enum_desktop = EnumDesktopWindows(None, Some(enum_desktop_windows_proc), lparam);
      if enum_desktop.is_ok() && open_windows.len().ne(&0) {
        for hwnd in open_windows {
          let window_info = get_window_information(hwnd);
          if window_info.title.eq(&"") && window_info.info.exec_name.to_lowercase().eq(&"explorer")
          {
            continue;
          }
          results.push(window_info);
        }
      }
    }

    results
  }
}

/**
 * Is the window show as maximized
 */
fn is_fullscreen(hwnd: HWND) -> BOOL {
  let mut lpwndpl: WINDOWPLACEMENT = WINDOWPLACEMENT::default();

  unsafe {
    if GetWindowPlacement(hwnd, &mut lpwndpl).is_ok() && SW_SHOWMAXIMIZED.0.eq(&(lpwndpl.showCmd as i32)) {
      return true.into();
    }
  }
  return false.into();
}

/** Functions for callback */
extern "system" fn enum_desktop_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
  let open_windows = unsafe { std::mem::transmute::<LPARAM, &mut Vec<HWND>>(lparam) };

  unsafe {
    if IsWindow(hwnd).as_bool() && IsWindow(hwnd).as_bool() && IsWindowVisible(hwnd).as_bool() {
      let mut pwi: WINDOWINFO = WINDOWINFO::default();
      let _ = GetWindowInfo(hwnd, &mut pwi);
      if ((pwi.dwExStyle & WS_EX_TOOLWINDOW
        == windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0)
        && pwi.dwStyle & WS_CAPTION == WS_CAPTION)
        || pwi.dwWindowStatus == WS_ACTIVECAPTION.0
        || is_fullscreen(hwnd).as_bool())
        && pwi.dwStyle & WS_CHILD == windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE(0)
      {
        let mut clocked_val: i32 = 0;
        let cbattribute = std::mem::size_of::<i32>() as u32;
        let result = DwmGetWindowAttribute(
          hwnd,
          DWMWA_CLOAKED,
          &mut clocked_val as *mut i32 as *mut _,
          cbattribute,
        );
        if result.is_ok() && clocked_val == 0 {
          open_windows.push(hwnd);
        }
      }
    }
  }

  true.into()
}

extern "system" fn enum_child_windows_func(hwnd: HWND, lparam: LPARAM) -> BOOL {
  let process_info: &mut ProcessInfo =
    unsafe { std::mem::transmute::<LPARAM, &mut ProcessInfo>(lparam) };

  let mut process_id: u32 = 0;
  let _id: u32 = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };

  if let Ok(handle) = open_process_handle(process_id) {
    let new_process_info: ProcessInfo = get_process_path_and_name(handle, hwnd, process_id);
    close_process_handle(handle);
    if process_info.path.ne(&new_process_info.path) {
      process_info.exec_name = new_process_info.exec_name;
      process_info.name = new_process_info.name;
      process_info.path = new_process_info.path;
      process_info.process_id = new_process_info.process_id;
      false.into()
    } else {
      true.into()
    }
  } else {
    true.into()
  }
}

/**
 * To know the os
 */
fn os_name() -> String {
  r#"win32"#.to_owned()
}

/**
 * Method to open hnadle
 */
fn open_process_handle(process_id: u32) -> Result<HANDLE, ()> {
  let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) };
  Ok(handle.map_err(|_| ())?)
}

/**
 * Method to close opend handle
 */
fn close_process_handle(handle: HANDLE) -> () {
  unsafe {
    let _ = CloseHandle(handle);
  };
}

/**
 * Function to get Rect data of a window
 */
fn get_rect_window(hwnd: HWND) -> WindowPosition {
  unsafe {
    let mut lprect: RECT = std::mem::zeroed();
    if GetWindowRect(hwnd, &mut lprect).is_ok() {
      WindowPosition {
        height: lprect.bottom - lprect.top,
        width: lprect.right - lprect.left,
        x: lprect.left,
        y: lprect.top,
      }
    } else {
      WindowPosition {
        height: 0,
        width: 0,
        x: 0,
        y: 0,
      }
    }
  }
}

/**
 * Get window title from HWND
 */
fn get_window_title(hwnd: HWND) -> String {
  let title: String;
  let mut v: Vec<u16> = vec![0; 255];
  let title_len = unsafe { GetWindowTextW(hwnd, &mut v) };
  title = String::from_utf16_lossy(&v[0..(title_len as usize)]);
  title
}

/**
 * Get process path from handle
 */
fn get_process_path(phlde: HANDLE) -> Result<PathBuf, ()> {
  let mut lpdwsize: u32 = MAX_PATH;
  let mut lpexename_raw: Vec<u16> = vec![0; MAX_PATH as usize];
  let lpexename: PWSTR = windows::core::PWSTR::from_raw(lpexename_raw.as_mut_ptr());

  let process_path: String = unsafe {
    let failed =
      QueryFullProcessImageNameW(phlde, PROCESS_NAME_WIN32, lpexename, &mut lpdwsize).is_err();
    if failed {
      return Err(());
    }
    lpexename.to_string().map_err(|_| ())?
  };
  Ok(Path::new(&process_path).to_path_buf())
}

/**
 * Get process name with help of the process path
 */
fn get_process_name_from_path(process_path: &PathBuf) -> Result<String, ()> {
  let lptstrfilename: windows::core::HSTRING = process_path.as_os_str().into();
  let dwlen: u32 = unsafe { GetFileVersionInfoSizeW(&lptstrfilename, Some(std::ptr::null_mut())) };
  if dwlen == 0 {
    return Err(());
  }
  let mut lpdata: Vec<u8> = vec![0u8; dwlen.try_into().unwrap()];
  let version_info_success =
    unsafe { GetFileVersionInfoW(&lptstrfilename, 0, dwlen, lpdata.as_mut_ptr().cast()).is_ok() };
  if !version_info_success {
    return Err(());
  }
  let mut lplpbuffer: *mut c_void = std::ptr::null_mut();
  let mut pulen: u32 = 0;

  let ver_query_success: BOOL = unsafe {
    VerQueryValueW(
      lpdata.as_ptr().cast(),
      w!("\\VarFileInfo\\Translation"),
      &mut lplpbuffer,
      &mut pulen,
    )
  };

  if !ver_query_success.as_bool() {
    return Err(());
  }

  let lang: &[LangCodePage] =
    unsafe { std::slice::from_raw_parts(lplpbuffer as *const LangCodePage, 1) };

  if lang.len() == 0 {
    return Err(());
  }

  let mut query_len: u32 = 0;

  let lang = lang.get(0).unwrap();
  let lang_code = format!(
    "\\StringFileInfo\\{:04x}{:04x}\\FileDescription",
    lang.w_language, lang.w_code_page
  );
  let lang_code_string: String = lang_code.to_string();
  let lang_code_ptr: *const u16 = lang_code_string
    .encode_utf16()
    .chain(Some(0))
    .collect::<Vec<_>>()
    .as_ptr();

  let lang_code: PCWSTR = PCWSTR::from_raw(lang_code_ptr);

  let mut file_description_ptr = std::ptr::null_mut();

  let file_description_query_success: BOOL = unsafe {
    VerQueryValueW(
      lpdata.as_ptr().cast(),
      lang_code,
      &mut file_description_ptr,
      &mut query_len,
    )
  };

  if !file_description_query_success.as_bool() {
    return Err(());
  }

  let file_description =
    unsafe { std::slice::from_raw_parts(file_description_ptr.cast(), query_len as usize) };
  let file_description = String::from_utf16_lossy(file_description);
  let file_description = file_description.trim_matches(char::from(0)).to_owned();

  return Ok(file_description);
}

/**
 * Return process info with pid, name and path (search deep in cas of using ApplicationFrameHost)
 */
fn get_process_path_and_name(phlde: HANDLE, hwnd: HWND, process_id: u32) -> ProcessInfo {
  let mut process_info = ProcessInfo {
    process_id,
    name: "".to_string(),
    path: "".to_string(),
    exec_name: "".to_string(),
  };

  if let Ok(process_path) = get_process_path(phlde) {
    process_info.exec_name = process_path
      .file_stem()
      .unwrap_or(std::ffi::OsStr::new(""))
      .to_str()
      .unwrap_or("")
      .to_owned();
    process_info.path = process_path
      .clone()
      .into_os_string()
      .into_string()
      .unwrap()
      .to_owned();
    process_info.name = process_info.exec_name.clone();

    if process_info
      .exec_name
      .to_lowercase()
      .eq(r#"applicationframehost"#)
    {
      let lparam = unsafe {
        std::mem::transmute::<*mut c_void, LPARAM>(
          &mut process_info as *mut ProcessInfo as *mut c_void,
        )
      };
      unsafe { EnumChildWindows(hwnd, Some(enum_child_windows_func), lparam) };
    } else if let Ok(process_name) = get_process_name_from_path(&process_path) {
      process_info.name = process_name;
    }
  }

  process_info
}

/**
 * Function that construct windowInfo
 */
fn get_window_information(hwnd: HWND) -> WindowInfo {
  let mut window_info: WindowInfo = WindowInfo {
    id: 0,
    os: os_name(),
    title: "".to_string(),
    position: WindowPosition {
      x: 0,
      y: 0,
      width: 0,
      height: 0,
    },
    info: ProcessInfo {
      process_id: 0,
      path: "".to_string(),
      name: "".to_string(),
      exec_name: "".to_string(),
    },
    usage: UsageInfo { memory: 0 },
    url: "".to_string(),
  };
  let mut lpdwprocessid: u32 = 0;
  unsafe { GetWindowThreadProcessId(hwnd, Some(&mut lpdwprocessid)) };

  if let Ok(handle) = open_process_handle(lpdwprocessid) {
    let position: WindowPosition = get_rect_window(hwnd);
    let id = unsafe { GetProcessId(handle) };
    let parent_process: ProcessInfo = get_process_path_and_name(handle, hwnd, lpdwprocessid);

    let mut process_memory_counters = PROCESS_MEMORY_COUNTERS::default();

    unsafe {
      let _ = GetProcessMemoryInfo(
        handle,
        &mut process_memory_counters as *mut _,
        std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
      );
    };
    close_process_handle(handle);
    let exec_name = parent_process.exec_name.to_lowercase();
    if exec_name.ne(&"searchhost") {
      let mut url: String = "".to_owned();
      if is_browser(&exec_name.as_str()) {
        url = get_browser_url(hwnd, exec_name).to_owned();
      }
      window_info = WindowInfo {
        id,
        os: os_name(),
        title: get_window_title(hwnd),
        position,
        info: parent_process,
        usage: UsageInfo {
          memory: process_memory_counters.WorkingSetSize as u32,
        },
        url,
      };
    }
  }

  window_info
}

fn get_browser_url(hwnd: HWND, exec_name: String) -> String {
  unsafe {
    if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_ok() {
      let automation: Result<IUIAutomation, _> = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL);
      if automation.is_ok() {
        let automation: IUIAutomation = automation.unwrap();
        let element: Result<IUIAutomationElement, _> = automation.ElementFromHandle(hwnd);
        if element.is_ok() {
          let element: IUIAutomationElement = element.unwrap();
          /* Chromium part to get url from search bar */
          return match &exec_name.to_lowercase() {
            x if x.contains(&"firefox") => get_url_from_automation_id(&automation, &element, "urlbar-input".to_owned()),
            x if x.contains(&"msedge") => get_url_from_automation_id(&automation, &element, "view_1020".to_owned()),
            _ => get_url_for_chromium(&automation, &element)
        };
        }
      }
    }
  }
  return "".to_string();
}

/**
 * Get value from automationId
 */
fn get_url_from_automation_id(
  automation: &IUIAutomation,
  element: &IUIAutomationElement,
  automation_id: String,
) -> String {
  unsafe {
    let mut variant1: VARIANT_0_0_0 = VARIANT_0_0_0::default();
    variant1.bstrVal = ::std::mem::ManuallyDrop::new(::windows::core::BSTR::from(automation_id));
    let mut variant2: VARIANT_0_0 = VARIANT_0_0::default();
    variant2.vt = VT_BSTR;
    variant2.Anonymous = variant1.clone();
    let mut variant3 = VARIANT_0::default();
    variant3.Anonymous = ::std::mem::ManuallyDrop::new(variant2.into());
    let variant = VARIANT {
      Anonymous: variant3.clone(),
    };

    let condition = automation
      .CreatePropertyCondition(UIA_AutomationIdPropertyId, variant)
      .unwrap();

    let test = element.FindFirst(TreeScope_Subtree, &condition);
    if test.is_ok() {
      let test = test.unwrap();
      let variant = test.GetCurrentPropertyValue(UIA_ValueValuePropertyId);
      if variant.is_ok() {
        let variant = variant.unwrap();
        if !variant.Anonymous.Anonymous.Anonymous.bstrVal.is_empty() {
          return variant.Anonymous.Anonymous.Anonymous.bstrVal.to_string();
        }
      }
    }
  }
  return "".to_string();
}

/**
 * Loop to find out the url
 */
fn get_url_for_chromium(automation: &IUIAutomation, element: &IUIAutomationElement) -> String {
  unsafe {
    let mut variant1: VARIANT_0_0_0 = VARIANT_0_0_0::default();
    variant1.lVal = 0xC36E;
    let mut variant2 = VARIANT_0_0::default();
    variant2.vt = VT_I4;
    variant2.Anonymous = variant1.clone();
    let mut variant3 = VARIANT_0::default();
    variant3.Anonymous = ::std::mem::ManuallyDrop::new(variant2.into());
    let variant = VARIANT {
      Anonymous: variant3.clone(),
    };

    let condition = automation
      .CreatePropertyCondition(UIA_ControlTypePropertyId, variant)
      .unwrap();

    let test = element.FindFirst(TreeScope_Children, &condition);

    if test.is_ok() {
      let test = test.unwrap();
      let variant = test.GetCurrentPropertyValue(UIA_ValueValuePropertyId);
      if variant.is_ok() {
        let variant = variant.unwrap();
        if !variant.Anonymous.Anonymous.Anonymous.bstrVal.is_empty() {
          return variant.Anonymous.Anonymous.Anonymous.bstrVal.to_string();
        }
      }
    }
  }
  return get_url_for_chromium_from_ctrlk(automation, element);
}

/** Fallback to search url from ctrl+l keyboard access */
fn get_url_for_chromium_from_ctrlk(automation: &IUIAutomation, element: &IUIAutomationElement) -> String {
  unsafe {
    let mut variant1: VARIANT_0_0_0 = VARIANT_0_0_0::default();
    variant1.lVal = 0xC354;
    let mut variant2 = VARIANT_0_0::default();
    variant2.vt = VT_I4;
    variant2.Anonymous = variant1.clone();
    let mut variant3 = VARIANT_0::default();
    variant3.Anonymous = ::std::mem::ManuallyDrop::new(variant2.into());
    let variant = VARIANT {
      Anonymous: variant3.clone(),
    };

    let condition1 = automation
      .CreatePropertyCondition(UIA_ControlTypePropertyId, variant)
      .unwrap();

    let mut variant1: VARIANT_0_0_0 = VARIANT_0_0_0::default();
    variant1.bstrVal = ::std::mem::ManuallyDrop::new(::windows::core::BSTR::from("Ctrl+L"));
    let mut variant2: VARIANT_0_0 = VARIANT_0_0::default();
    variant2.vt = VT_BSTR;
    variant2.Anonymous = variant1.clone();
    let mut variant3 = VARIANT_0::default();
    variant3.Anonymous = ::std::mem::ManuallyDrop::new(variant2.into());
    let variant = VARIANT {
      Anonymous: variant3.clone(),
    };

    let condition2 = automation
    .CreatePropertyCondition(UIA_AccessKeyPropertyId, variant)
    .unwrap();

    let condition = automation.CreateAndCondition(&condition1, &condition2).unwrap();

    let test = element.FindFirst(TreeScope_Subtree, &condition);

    if test.is_ok() {
      let test = test.unwrap();
      let variant = test.GetCurrentPropertyValue(UIA_ValueValuePropertyId);
      if variant.is_ok() {
        let variant = variant.unwrap();
        if !variant.Anonymous.Anonymous.Anonymous.bstrVal.is_empty() {
          return variant.Anonymous.Anonymous.Anonymous.bstrVal.to_string();
        }
      }
    }
  }
  return "".to_owned();
}


fn is_browser(browser_name: &str) -> bool {
  match browser_name {
    "chrome" | "msedge" | "opera" | "opera_gx" | "brave" | "vivaldi" | "iron" | "epic"
    | "chromium" | "ucozmedia" | "blisk" | "maxthon" | "beaker" | "beaker browser" | "firefox" => {
      true
    }
    _ => false,
  }
}

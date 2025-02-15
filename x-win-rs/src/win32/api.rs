#![deny(unused_imports)]

use base64::Engine;

use windows::{
  core::{w, VARIANT},
  Win32::{
    Foundation::{FALSE, TRUE},
    Graphics::Gdi::{
      DeleteDC, DeleteObject, GetObjectW, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
      DIB_RGB_COLORS,
    },
    UI::{
      Shell::ExtractIconExW,
      WindowsAndMessaging::{DestroyIcon, FindWindowW, GetIconInfo, HICON, ICONINFO},
    },
  },
};

use crate::common::{
  api::{empty_entity, empty_icon, os_name, Api},
  x_win_struct::{
    icon_info::IconInfo, process_info::ProcessInfo, usage_info::UsageInfo, window_info::WindowInfo,
    window_position::WindowPosition,
  },
};
use std::{ffi::c_void, os::windows::ffi::OsStrExt};
use std::{
  ffi::OsStr,
  path::{Path, PathBuf},
};
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
impl Api for WindowsAPI {
  fn get_active_window(&self) -> WindowInfo {
    let hwnd = unsafe { GetForegroundWindow() };
    get_window_information(hwnd)
  }

  fn get_open_windows(&self) -> Vec<WindowInfo> {
    let mut results: Vec<WindowInfo> = Vec::new();

    enum_desktop_windows(|hwnd| {
      let window_info = get_window_information(hwnd);
      if !(window_info.title.is_empty()
        && window_info.info.exec_name.to_lowercase().eq(&"explorer"))
      {
        results.push(window_info);
      }
      true
    });

    results
  }

  fn get_app_icon(&self, window_info: &WindowInfo) -> IconInfo {
    if !window_info.info.path.is_empty() {
      let lpszfile: Vec<u16> = std::path::Path::new(&window_info.info.path)
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();

      let mut phiconlarge = HICON::default();
      let mut phiconsmall = HICON::default();

      let value = unsafe {
        ExtractIconExW(
          PCWSTR(lpszfile.as_ptr()),
          0,
          Some(&mut phiconlarge as *mut HICON),
          Some(&mut phiconsmall as *mut HICON),
          1,
        )
      };

      if value.ne(&0) && (!phiconlarge.0.is_null() || !phiconsmall.0.is_null()) {
        let mut piconinfo: ICONINFO = ICONINFO::default();
        let phicon = {
          if !phiconlarge.0.is_null() {
            phiconlarge
          } else {
            phiconsmall
          }
        };
        let icon_info = unsafe { GetIconInfo(phicon, &mut piconinfo as *mut ICONINFO as _) };
        if icon_info.is_ok() {
          let hbm = piconinfo.hbmColor;

          let mut cbitmap = BITMAP::default();

          let objectw = unsafe {
            GetObjectW(
              hbm,
              std::mem::size_of::<BITMAP>() as i32,
              Some(&mut cbitmap as *mut _ as _),
            )
          };

          if objectw > 0 {
            let mut lpbmi = BITMAPINFO::default();
            lpbmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
            lpbmi.bmiHeader.biWidth = cbitmap.bmWidth;
            lpbmi.bmiHeader.biHeight = -cbitmap.bmHeight;
            lpbmi.bmiHeader.biPlanes = 1;
            lpbmi.bmiHeader.biBitCount = 32;
            lpbmi.bmiHeader.biCompression = BI_RGB.0;

            let hdc = unsafe { windows::Win32::Graphics::Gdi::CreateCompatibleDC(None) };
            let mut buffer: Vec<u8> = vec![0u8; (cbitmap.bmHeight * cbitmap.bmWidth * 4) as usize];
            let height = unsafe {
              windows::Win32::Graphics::Gdi::GetDIBits(
                hdc,
                hbm,
                0,
                cbitmap.bmHeight as u32,
                Some(buffer.as_mut_ptr().cast()),
                &mut lpbmi,
                DIB_RGB_COLORS,
              )
            };

            let mut data: String = String::new();

            if height.eq(&cbitmap.bmHeight) {
              //Reverse table to have rgba value from bgra buffer
              for chunk in buffer.chunks_mut(4) {
                let [b, _, r, _] = chunk else { unreachable!() };
                std::mem::swap(b, r);
              }
              let mut png_data = Vec::new();
              {
                let cursor = std::io::Cursor::new(&mut png_data);
                let mut encoder =
                  png::Encoder::new(cursor, cbitmap.bmWidth as u32, cbitmap.bmHeight as u32);
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header().unwrap();
                writer.write_image_data(&buffer).unwrap();
              }
              data = base64::prelude::BASE64_STANDARD.encode(png_data);
            }

            unsafe {
              let _ = DeleteDC(hdc);
              let _ = DeleteObject(hbm);
            };
            return IconInfo {
              data: format!("data:image/png;base64,{}", data).to_owned(),
              height: cbitmap.bmHeight as u32,
              width: cbitmap.bmWidth as u32,
            };
          }
        }
        unsafe {
          if !phiconlarge.0.is_null() {
            DestroyIcon(phiconlarge).unwrap();
          }
          if !phiconsmall.0.is_null() {
            DestroyIcon(phiconsmall).unwrap();
          }
        };
      }
    }

    empty_icon()
  }

  fn get_browser_url(&self, window_info: &WindowInfo) -> String {
    let mut url: String = String::from("");
    if !window_info.info.exec_name.is_empty() && is_browser(window_info.info.exec_name.as_str()) {
      let hwnd = unsafe {
        let data: Vec<u16> = OsStr::new(&window_info.title.to_owned())
          .encode_wide()
          .chain(Some(0))
          .collect();
        let window_title = windows::core::PCWSTR(data.as_ptr());
        FindWindowW(None, window_title)
      };
      if hwnd.is_ok() {
        url = get_browser_url(hwnd.unwrap(), window_info.info.exec_name.clone()).clone();
      }
    }
    url
  }
}

/** Functions for callback */
unsafe extern "system" fn enum_desktop_windows_proc<Callback: FnMut(HWND) -> bool>(
  hwnd: HWND,
  lparam: LPARAM,
) -> BOOL {
  let callback = lparam.0 as *mut Callback;
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
          // If problem with callback stop loop
          if !((*callback)(hwnd)) {
            return FALSE;
          }
        }
      }
    }

    TRUE
  }
}

extern "system" fn enum_child_windows_func(hwnd: HWND, lparam: LPARAM) -> BOOL {
  let process_info = lparam.0 as *mut ProcessInfo;
  let mut process_id: u32 = 0;
  let _id: u32 = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };
  if let Ok(handle) = open_process_handle(process_id) {
    let new_process_info: ProcessInfo = get_process_path_and_name(handle, hwnd, process_id);
    close_process_handle(handle);
    unsafe {
      if (*process_info).path.ne(&new_process_info.path) {
        (*process_info).exec_name = new_process_info.exec_name;
        (*process_info).name = new_process_info.name;
        (*process_info).path = new_process_info.path;
        (*process_info).process_id = new_process_info.process_id;
        FALSE
      } else {
        TRUE
      }
    }
  } else {
    TRUE
  }
}

/** Function with callback as parameter to get open windows */
fn enum_desktop_windows<Callback: FnMut(HWND) -> bool>(callback: Callback) {
  unsafe {
    let lparam = LPARAM(&callback as *const _ as isize);
    let _ = EnumDesktopWindows(None, Some(enum_desktop_windows_proc::<Callback>), lparam);
  }
}

/**
 * Is the window show as maximized
 */
fn is_fullscreen(hwnd: HWND) -> BOOL {
  let mut lpwndpl: WINDOWPLACEMENT = WINDOWPLACEMENT::default();

  unsafe {
    if GetWindowPlacement(hwnd, &mut lpwndpl).is_ok()
      && SW_SHOWMAXIMIZED.0.eq(&(lpwndpl.showCmd as i32))
    {
      return TRUE;
    }
  }
  FALSE
}

/**
 * Method to open hnadle
 */
fn open_process_handle(process_id: u32) -> Result<HANDLE, ()> {
  let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) };
  handle.map_err(|_| ())
}

/**
 * Method to close opend handle
 */
fn close_process_handle(handle: HANDLE) {
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
        is_full_screen: is_fullscreen(hwnd).as_bool(),
      }
    } else {
      WindowPosition {
        height: 0,
        width: 0,
        x: 0,
        y: 0,
        is_full_screen: false,
      }
    }
  }
}

/**
 * Get window title from HWND
 */
fn get_window_title(hwnd: HWND) -> String {
  let mut v: Vec<u16> = vec![0; 255];
  let title_len = unsafe { GetWindowTextW(hwnd, &mut v) };
  String::from_utf16_lossy(&v[0..(title_len as usize)])
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
fn get_process_name_from_path(process_path: &Path) -> Result<String, ()> {
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

  if lang.is_empty() {
    return Err(());
  }

  let mut query_len: u32 = 0;

  let lang = lang.first().unwrap();
  let lang_code = format!(
    "\\StringFileInfo\\{:04x}{:04x}\\FileDescription",
    lang.w_language, lang.w_code_page
  );
  let lang_code_string: String = lang_code.to_string();
  let lang_code_ptr = lang_code_string
    .encode_utf16()
    .chain(Some(0))
    .collect::<Vec<_>>();
  let lang_code_ptr = lang_code_ptr.as_ptr();

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

  Ok(file_description)
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
    process_info.exec_name = match process_path.file_stem() {
      Some(process_path) => process_path.to_str().unwrap_or("").to_string(),
      None => String::from(""),
    };

    process_info.path = match process_path.as_path().to_str().to_owned() {
      Some(path) => path.to_string(),
      None => String::from(""),
    };

    if process_info
      .exec_name
      .to_lowercase()
      .eq(r#"applicationframehost"#)
    {
      let lparam = LPARAM(&mut process_info as *const ProcessInfo as isize);
      let _ = unsafe { EnumChildWindows(hwnd, Some(enum_child_windows_func), lparam) };
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
  let mut window_info: WindowInfo = empty_entity();
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
      window_info = WindowInfo {
        id,
        os: os_name(),
        title: get_window_title(hwnd),
        position,
        info: parent_process,
        usage: UsageInfo {
          memory: process_memory_counters.WorkingSetSize as u32,
        },
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
            x if x.contains("firefox") => {
              get_url_from_automation_id(&automation, &element, "urlbar-input".to_owned())
            }
            x if x.contains("msedge") => {
              let mut value =
                get_url_from_automation_id(&automation, &element, "view_1022".to_owned());
              if value.is_empty() {
                value = get_url_from_automation_id(&automation, &element, "view_1020".to_owned());
              }
              value.to_owned()
            }
            _ => get_url_for_chromium(&automation, &element),
          };
        }
      }
    }
  }

  "".to_string()
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
    let variant = VARIANT::from(::windows::core::BSTR::from(automation_id));
    let condition = automation
      .CreatePropertyCondition(UIA_AutomationIdPropertyId, &variant)
      .unwrap();
    let test = element.FindFirst(TreeScope_Subtree, &condition);
    if test.is_ok() {
      let test = test.unwrap();
      let variant = test.GetCurrentPropertyValue(UIA_ValueValuePropertyId);
      if variant.is_ok() {
        let variant = variant.unwrap();
        if !variant.is_empty() {
          return variant.to_string();
        }
      }
    }
  }

  "".to_string()
}

/**
 * Get url from element id 0xC36E
 */
fn get_url_for_chromium(automation: &IUIAutomation, element: &IUIAutomationElement) -> String {
  unsafe {
    let variant = VARIANT::from(0xC36E);
    let condition = automation
      .CreatePropertyCondition(UIA_ControlTypePropertyId, &variant)
      .unwrap();
    let test = element.FindFirst(TreeScope_Children, &condition);
    if test.is_ok() {
      let test = test.unwrap();
      let variant = test.GetCurrentPropertyValue(UIA_ValueValuePropertyId);
      if variant.is_ok() {
        let variant = variant.unwrap();
        if !variant.is_empty() {
          return variant.to_string();
        }
      }
    }
  }

  // If not found fallback to use ctrl+l to get it
  get_url_for_chromium_from_ctrlk(automation, element)
}

/** Fallback to recover url from ctrl+l keyboard access */
fn get_url_for_chromium_from_ctrlk(
  automation: &IUIAutomation,
  element: &IUIAutomationElement,
) -> String {
  unsafe {
    let variant = VARIANT::from(0xC354);
    let condition1 = automation
      .CreatePropertyCondition(UIA_ControlTypePropertyId, &variant)
      .unwrap();
    let variant = VARIANT::from(::windows::core::BSTR::from("Ctrl+L"));
    let condition2 = automation
      .CreatePropertyCondition(UIA_AccessKeyPropertyId, &variant)
      .unwrap();
    let condition = automation
      .CreateAndCondition(&condition1, &condition2)
      .unwrap();
    let test = element.FindFirst(TreeScope_Subtree, &condition);
    if test.is_ok() {
      let test = test.unwrap();
      let variant = test.GetCurrentPropertyValue(UIA_ValueValuePropertyId);
      if variant.is_ok() {
        let variant = variant.unwrap();
        if !variant.is_empty() {
          return variant.to_string();
        }
      }
    }
  }

  String::from("")
}

fn is_browser(browser_name: &str) -> bool {
  matches!(
    browser_name,
    "chrome"
      | "msedge"
      | "opera"
      | "opera_gx"
      | "brave"
      | "vivaldi"
      | "iron"
      | "epic"
      | "chromium"
      | "ucozmedia"
      | "blisk"
      | "maxthon"
      | "beaker"
      | "beaker browser"
      | "firefox"
  )
}

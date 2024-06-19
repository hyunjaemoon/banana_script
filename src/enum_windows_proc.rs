extern crate winapi;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::um::winuser::{
    mouse_event, ClientToScreen, EnumWindows, GetClientRect, GetWindowTextA, GetWindowTextLengthA,
    IsWindowVisible, SetCursorPos, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
};

extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> i32 {
    unsafe {
        if IsWindowVisible(hwnd) == 0 {
            return 1; // Continue enumerating
        }

        let length = GetWindowTextLengthA(hwnd) + 1;
        if length == 1 {
            return 1; // Continue enumerating
        }

        let mut buffer: Vec<c_char> = Vec::with_capacity(length as usize);
        buffer.set_len(length as usize - 1); // Set length to length - 1 for null termination

        GetWindowTextA(hwnd, buffer.as_mut_ptr(), length);

        let window_title = CStr::from_ptr(buffer.as_ptr())
            .to_string_lossy()
            .into_owned();

        if window_title.contains("Banana") {
            let hwnd_ptr = lparam as *mut HWND;
            *hwnd_ptr = hwnd;
            return 0; // Stop enumerating
        }
    }
    1 // Continue enumerating
}

fn find_window_by_title_substring(_substring: &str) -> Option<HWND> {
    let mut hwnd: HWND = null_mut();
    let hwnd_ptr: *mut HWND = &mut hwnd;

    unsafe {
        EnumWindows(Some(enum_windows_proc), hwnd_ptr as isize);
    }

    if hwnd.is_null() {
        None
    } else {
        Some(hwnd)
    }
}

fn click_at_position(x: i32, y: i32) {
    unsafe {
        SetCursorPos(x, y);
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}

pub fn run(hours: u32, minutes: u32, seconds: u32, delay_time: u64, is_infinite: bool) {
    match find_window_by_title_substring("Banana") {
        Some(hwnd) => {
            let mut rect: RECT = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            unsafe {
                GetClientRect(hwnd, &mut rect);
            }

            let mut point = POINT {
                x: (rect.right - rect.left) / 2,
                y: (rect.bottom - rect.top) / 2,
            };

            unsafe {
                ClientToScreen(hwnd, &mut point);
            }

            let request_exit: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
            let start_time = SystemTime::now();
            let wait_time = (hours * 60 * 60) + (minutes * 60) + seconds;
            let mut hour_count = 0;
            loop {
                match request_exit.load(Ordering::SeqCst) {
                    true => break,
                    false => click_at_position(point.x, point.y),
                }
                if is_infinite {
                    println!("Hours Passed: {}", hour_count);
                    sleep(Duration::from_secs(3601));
                    hour_count += 1;
                    continue;
                }
                if start_time.elapsed().unwrap().as_secs() >= wait_time.into() {
                    request_exit.store(true, Ordering::SeqCst);
                }
                sleep(Duration::from_millis(delay_time));
            }
        }
        None => eprintln!("Window with title containing 'Banana' not found!"),
    }
}

use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};

use rfd::MessageDialog;
use serde_json::Value;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Fullscreen, WindowBuilder},
};
use wry::WebViewBuilder;

#[derive(Debug, Clone)]
enum UserEvent {
    CloseAfterWin,
    ExitNow,
    Reopen,
}

fn main() -> wry::Result<()> {
    #[cfg(target_os = "windows")]
    {
        ensure_autostart_prompt_once();
    }
    let event_loop: EventLoop<UserEvent> = tao::event_loop::EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    #[cfg(target_os = "windows")]
    unsafe {
        keyboard::init_with_proxy(proxy.clone());
    }

    let window = WindowBuilder::new()
        .with_title("Sans Gate")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .build(&event_loop)
        .expect("failed to create window");

    // Force native fullscreen (borderless) at launch
    window.set_fullscreen(Some(Fullscreen::Borderless(None)));

    let won_flag = Arc::new(AtomicBool::new(false));
    let won_flag_ipc = won_flag.clone();
    let proxy_ipc = proxy.clone();

    let init_js = r#"
        (() => {
          const send = (obj) => {
            try { window.ipc.postMessage(JSON.stringify(obj)); } catch (_) {}
          };

          // One-time reload helper for hotkeys
          let __hasReloaded = false;
          const reloadOnce = () => { if (!__hasReloaded) { __hasReloaded = true; try { location.reload(); } catch (_) {} } };

          // Konami code: Up, Up, Down, Down, Left, Right, Left, Right, B, A
          (function(){
            const seq = ['ArrowUp','ArrowUp','ArrowDown','ArrowDown','ArrowLeft','ArrowRight','ArrowLeft','ArrowRight','b','a'];
            let idx = 0;
            const matches = (expected, key) => expected.length === 1 ? expected === key.toLowerCase() : expected === key;
            window.addEventListener('keydown', (e) => {
              try {
                const key = (e.key || '').toString();
                if (matches(seq[idx], key)) {
                  idx++;
                  if (idx === seq.length) {
                    send({ event: 'konami' });
                    idx = 0;
                  }
                } else {
                  // reset if mismatch, but allow restarting from first on immediate match
                  idx = matches(seq[0], key) ? 1 : 0;
                }
              } catch(_) {}
            }, { capture: true });
          })();

          // Removed auto 'Z' press at startup

          // Reload after 10 minutes if still running
          const reloadTimer = setTimeout(() => {
            try { location.reload(); } catch (_) {}
          }, 10 * 60 * 1000);

          // Hook console.log to detect win/loss
          (function() {
            const orig = console.log;
            console.log = function(...args) {
              try {
                const text = args.map(a => {
                  try { return typeof a === 'string' ? a : JSON.stringify(a); } catch(_) { return String(a); }
                }).join(' ');
                if (text.includes('Won')) {
                  try { clearTimeout(reloadTimer); } catch(_) {}
                  send({ event: 'won' });
                } else if (text.includes('Loss')) {
                  // No reload on loss
                }
              } catch (_) {}
              return orig.apply(this, args);
            };
          })();

          // Force browser fullscreen for the page content and reapply if lost.
          (function(){
            const isFull = () => !!(document.fullscreenElement || document.webkitFullscreenElement || document.msFullscreenElement);
            const requestFull = () => {
              try {
                const el = document.documentElement;
                const req = el.requestFullscreen || el.webkitRequestFullscreen || el.msRequestFullscreen;
                if (req) req.call(el);
              } catch (_) {}
            };

            let reloadAttempts = 0;
            let pending = false; // debounce reload check
            const enforce = () => {
              if (!isFull()) {
                requestFull();
                if (!pending) {
                  pending = true;
                  setTimeout(() => {
                    try {
                      if (!isFull() && reloadAttempts < 3) {
                        reloadAttempts++;
                        location.reload();
                      }
                    } catch (_) {}
                    pending = false;
                  }, 1500);
                }
              }
            };

            // Initial try and per-frame enforcement
            setTimeout(enforce, 500);
            const tick = () => { try { enforce(); } catch (_) {} requestAnimationFrame(tick); };
            requestAnimationFrame(tick);
            document.addEventListener('fullscreenchange', () => { if (!isFull()) enforce(); });
            document.addEventListener('webkitfullscreenchange', () => { if (!isFull()) enforce(); });
          })();

          // Soft handling of close-ish hotkeys inside the page context.
          // Note: cannot intercept Ctrl+Alt+Del; Alt+F4 is OS-handled and may still close.
          window.addEventListener('keydown', (e) => {
            try {
              const key = (e.key || '').toString();
              const k = key.length ? key.toLowerCase() : '';
              // Ctrl+W (common close-tab in browsers)
              if (e.ctrlKey && (k === 'w')) {
                e.preventDefault();
                try { reloadOnce(); } catch(_) {}
                return;
              }
              // Alt+F4 (best-effort; OS may close before this runs)
              if (e.altKey && (key === 'F4' || k === 'f4')) {
                e.preventDefault();
                try { reloadOnce(); } catch(_) {}
                return;
              }
            } catch (_) {}
          }, { capture: true });
        })();
    "#;

    let _webview = WebViewBuilder::new(&window)
        .with_url("https://benp1236691.github.io/BadtimePage/")
        .with_initialization_script(init_js)
        .with_ipc_handler(move |req| {
            let msg = req.body();
            if let Ok(v) = serde_json::from_str::<Value>(msg) {
                if v.get("event").and_then(|e| e.as_str()) == Some("won") {
                    if !won_flag_ipc.swap(true, Ordering::SeqCst) {
                        // First time we saw a win: schedule close after 3 seconds
                        let _ = proxy_ipc.send_event(UserEvent::CloseAfterWin);
                    }
                } else if v.get("event").and_then(|e| e.as_str()) == Some("konami") {
                    // Ignore Konami exit in hardened mode
                }
            }
        })
        .build()?;

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                // Nothing extra on init lol
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::CloseAfterWin) => {
                // Sleep 3 seconds, then optionally prompt
                let proxy2 = proxy.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(3));

                    // First-win prompt: offer to close now
                    let _ = MessageDialog::new()
                        .set_title("Victory!")
                        .set_description("You beat Sans. Close the app now?")
                        .set_buttons(rfd::MessageButtons::YesNo)
                        .show();

                    // No forced exit; keep running
                });
            }
            Event::UserEvent(UserEvent::ExitNow) => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::Reopen) => {
                // Relaunch a fresh instance and exit this one to avoid duplicates
                let _ = spawn_new_instance();
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Focused(false), .. } => {
                // On focus loss, relaunch to regain attention
                let _ = spawn_new_instance();
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                // Reopen if the user tries to close
                let _ = spawn_new_instance();
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

#[cfg(target_os = "windows")]
fn spawn_new_instance() -> std::io::Result<()> {
    let exe = std::env::current_exe()?;
    std::process::Command::new(exe).spawn()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn spawn_new_instance() -> std::io::Result<()> { Ok(()) }

#[cfg(target_os = "windows")]
fn ensure_autostart_prompt_once() {
    use std::fs;
    use std::path::PathBuf;

    const RUN_VALUE: &str = "SansGate";
    let mut prompted = false;
    if let Some(appdata) = std::env::var_os("APPDATA") {
        let mut p = PathBuf::from(appdata);
        p.push("SansGate");
        let _ = fs::create_dir_all(&p);
        p.push("autostart_prompted.flag");
        if p.exists() {
            prompted = true;
        } else {
            if is_autostart_configured(RUN_VALUE).unwrap_or(false) {
                let _ = fs::write(&p, b"1");
                prompted = true;
            } else {
                let result = rfd::MessageDialog::new()
                    .set_title("Enable Autostart")
                    .set_description("Run this app automatically when you sign in?")
                    .set_buttons(rfd::MessageButtons::YesNo)
                    .show();
                match result {
                    rfd::MessageDialogResult::Yes => { let _ = set_autostart(RUN_VALUE); }
                    _ => {}
                }
                let _ = fs::write(&p, b"1");
                prompted = true;
            }
        }
    }
    if !prompted {
        if !is_autostart_configured("SansGate").unwrap_or(true) {
            let _ = set_autostart("SansGate");
        }
    }
}

#[cfg(target_os = "windows")]
fn is_autostart_configured(value_name: &str) -> windows::core::Result<bool> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{RegCloseKey, RegGetValueW, RegOpenKeyExW, HKEY, HKEY_CURRENT_USER, RRF_RT_REG_SZ, KEY_READ};
    use windows::Win32::Foundation::ERROR_SUCCESS;

    let subkey = to_wide("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let name = to_wide(value_name);
    unsafe {
        let mut hkey: HKEY = HKEY::default();
        let open = RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), 0, KEY_READ, &mut hkey);
        if open != ERROR_SUCCESS { return Ok(false); }
        let mut size: u32 = 0;
        let status = RegGetValueW(hkey, PCWSTR(std::ptr::null()), PCWSTR(name.as_ptr()), RRF_RT_REG_SZ, None, None, Some(&mut size));
        let _ = RegCloseKey(hkey);
        Ok(status == ERROR_SUCCESS)
    }
}

#[cfg(target_os = "windows")]
fn set_autostart(value_name: &str) -> windows::core::Result<()> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{RegCloseKey, RegOpenKeyExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ};
    use windows::Win32::Foundation::ERROR_SUCCESS;

    let subkey = to_wide("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let name = to_wide(value_name);
    let exe = std::env::current_exe().unwrap_or_default();
    let exe_str = format!("\"{}\"", exe.display());
    let data = to_wide(&exe_str);
    unsafe {
        let mut hkey: HKEY = HKEY::default();
        let open = RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), 0, KEY_SET_VALUE, &mut hkey);
        if open != ERROR_SUCCESS { return Ok(()); }
        let bytes = std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2);
        let _ = RegSetValueExW(hkey, PCWSTR(name.as_ptr()), 0, REG_SZ, Some(bytes));
        let _ = RegCloseKey(hkey);
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn to_wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

#[cfg(target_os = "windows")]
mod keyboard {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::OnceLock;
    use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_ESCAPE, VK_F4, VK_LWIN, VK_RWIN, VK_SHIFT, VK_SPACE, VK_TAB};
    use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, SetWindowsHookExW, HHOOK, KBDLLHOOKSTRUCT, LLKHF_ALTDOWN, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN};
    use tao::event_loop::EventLoopProxy;
    use super::UserEvent;

    static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
    static PROXY: OnceLock<EventLoopProxy<UserEvent>> = OnceLock::new();
    static REOPEN_PENDING: AtomicBool = AtomicBool::new(false);

    #[no_mangle]
    pub unsafe extern "system" fn low_level_keyboard_proc(nCode: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if nCode >= 0 {
            let kb: &KBDLLHOOKSTRUCT = &*(l_param.0 as *const KBDLLHOOKSTRUCT);
            let vk = kb.vkCode as u32;
            let alt_down = (kb.flags & LLKHF_ALTDOWN) == LLKHF_ALTDOWN;
            let is_keydown = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;

            let matches_vk = |key: VIRTUAL_KEY| vk == key.0 as u32;
            let win_down = (GetAsyncKeyState(VK_LWIN.0 as i32) as u16 & 0x8000) != 0
                || (GetAsyncKeyState(VK_RWIN.0 as i32) as u16 & 0x8000) != 0;
            let ctrl_down = (GetAsyncKeyState(VK_CONTROL.0 as i32) as u16 & 0x8000) != 0;
            let shift_down = (GetAsyncKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0;

            // Block common task-switch/system combos (best-effort; OS may still handle some)
            if is_keydown {
                let block =
                    // Alt+Tab and Alt+Esc
                    (alt_down && (matches_vk(VK_TAB) || matches_vk(VK_ESCAPE))) ||
                    // Alt+F4
                    (alt_down && matches_vk(VK_F4)) ||
                    // Alt+Space
                    (alt_down && matches_vk(VK_SPACE)) ||
                    // Windows keys directly (prevents Win key menu) and Win+Tab
                    matches_vk(VK_LWIN) || matches_vk(VK_RWIN) || (win_down && matches_vk(VK_TAB)) ||
                    // Ctrl+Shift+Esc (Task Manager) and Ctrl+Esc (Start Menu)
                    ((ctrl_down && shift_down) && matches_vk(VK_ESCAPE)) ||
                    (ctrl_down && matches_vk(VK_ESCAPE));

                if block {
                    if let Some(p) = PROXY.get() {
                        if !REOPEN_PENDING.swap(true, Ordering::SeqCst) {
                            let _ = p.send_event(UserEvent::Reopen);
                            std::thread::spawn(|| {
                                std::thread::sleep(std::time::Duration::from_millis(1000));
                                REOPEN_PENDING.store(false, Ordering::SeqCst);
                            });
                        }
                    }
                    return LRESULT(1);
                }
            }
        }
        CallNextHookEx(HHOOK(std::ptr::null_mut()), nCode, w_param, l_param)
    }

    pub unsafe fn init_with_proxy(proxy: EventLoopProxy<UserEvent>) {
        let _ = PROXY.set(proxy);
        if HOOK_INSTALLED.swap(true, Ordering::SeqCst) {
            return;
        }
        // Install a global low-level keyboard hook.
        let _hhook: HHOOK = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), HINSTANCE(std::ptr::null_mut()), 0)
            .expect("failed to install keyboard hook");
        // Note: we purposely do not unhook on exit since the app is single-process and exits entirely.
    }
}

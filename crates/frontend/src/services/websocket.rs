use futures::SinkExt;
use futures::StreamExt;
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::ErrorEvent;
use web_sys::MessageEvent;
use web_sys::WebSocket;

pub fn start<C>(callback: C, in_rx: futures::channel::mpsc::UnboundedReceiver<common::AppToServer>)
where
    C: Fn(common::SimulationStatus) + 'static,
{
    log::info!("Starting websocket service");

    let location = web_sys::window().expect("no global `window` exists").location();
    let protocol = location.protocol().expect("should have a protocol");
    let hostname = location.hostname().expect("should have a hostname");

    let ws_protocol = match protocol.as_str() {
        "https:" => "wss",
        _ => "ws",
    };

    let port = match location.port() {
        Ok(port) if !port.is_empty() => format!(":{}", port),
        _ => String::new(),
    };

    let ws_url = format!("{}://{}{}/api/ws", ws_protocol, hostname, port);

    // Generate regular heartbeat messages from app to server
    let (mut heartbeat_tx, heartbeat_rx) = futures::channel::mpsc::channel(1);
    spawn_local(async move {
        let heartbeat_interval = yew::platform::time::interval(Duration::from_secs(2));
        futures::pin_mut!(heartbeat_interval);
        while heartbeat_interval.next().await.is_some() {
            let msg = common::AppToServer::HeartBeat;
            heartbeat_tx.send(msg).await.unwrap();
        }
    });

    // App to Server
    let mut all_rx = futures::stream::select(in_rx, heartbeat_rx).map(|msg| msg.ser());

    spawn_local({
        async move {
            match WebSocket::new(&ws_url) {
                Err(err) => {
                    log::error!("Failed to create WebSocket: {err:?}");
                    yew::platform::time::sleep(Duration::from_secs(5)).await;
                }
                Ok(ws) => {
                    let onmessage = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                            let text: String = text.into();
                            let ws_msg = common::ServerToApp::deser(&text);
                            match ws_msg {
                                common::ServerToApp::SimulationStatus(value) => {
                                    log::info!("Received simulation status");
                                    callback(value);
                                }
                                common::ServerToApp::HeartBeat => {
                                    log::trace!("Received heartbeat");
                                }
                            }
                        }
                    });
                    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                    onmessage.forget();

                    let onerror = Closure::<dyn FnMut(_)>::new(|err: ErrorEvent| {
                        log::error!("Websocket error: {err:?}");
                    });
                    ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                    onerror.forget();

                    let onclose = Closure::<dyn FnMut()>::new(|| {
                        log::error!("Websocket closed");
                    });
                    ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                    onclose.forget();

                    let onopen = Closure::<dyn FnMut()>::new(move || {
                        log::info!("WebSocket connected");
                    });
                    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                    onopen.forget();

                    while let Some(msg) = all_rx.next().await {
                        if let Err(err) = ws.send_with_str(&msg) {
                            log::error!("Failed to send message: {:?}", err);
                        }
                    }
                }
            }
        }
    });
}

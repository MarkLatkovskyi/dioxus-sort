use std::collections::HashMap;
use std::time::Duration;
use crate::canvas_updater::CanvasUpdater;
use components::*;
use consts::*;
use dioxus::prelude::*;
use futures::channel::mpsc::{Receiver, TryRecvError};
use futures::channel::oneshot::Sender as OneSender;
use futures_timer::Delay;
use crate::checked_list::ListAction;

mod canvas_updater;
mod checked_list;
mod checked_value;
mod components;
mod consts;
mod sorts;

#[allow(unused)]
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
#[allow(unused)]
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
#[allow(unused)]
const TAILWIND: Asset = asset!("/assets/tailwind.js");

fn main() {
    launch(App);
}

pub enum CoroutineAction {
    NewSort(Receiver<ListAction>, OneSender<Vec<Num>>),
    ChangeLen(usize),
    Stop,
    Start,
    ChangeDelay(Duration),
    Shuffle,
}

async fn coroutine(mut rx: UnboundedReceiver<CoroutineAction>) {
    let list = vec![1, 2, 3, 4, 5, 6, 7];
    let mut canvas_updater = CanvasUpdater::new(list);
    let mut delay = Duration::from_millis(100);
    let mut receiver: Option<Receiver<_>> = None;
    let mut is_running = false;
    loop {
        let ok_message = match rx.try_recv() {
            Ok(v) => v,
            Err(TryRecvError::Empty) => {
                let Some(receiver) = receiver.as_mut() else {
                    Delay::new(delay).await;
                    continue;
                };
                if is_running {
                    let Ok(action) = receiver.recv().await else {
                        is_running = false;
                        continue
                    };
                    canvas_updater.proceed(action);
                }

                Delay::new(delay).await;
                continue
            }
            Err(TryRecvError::Closed) => {
                loop {
                    match rx.recv().await.unwrap() {
                        msg @ CoroutineAction::NewSort(..) => {
                            break msg;
                        }
                        _ => continue,
                    }
                }
            }
        };
        match ok_message {
            CoroutineAction::Stop => {
                is_running = false;
            }
            CoroutineAction::ChangeDelay(d) => delay = d,
            CoroutineAction::Shuffle => {
                canvas_updater.shuffle_list();
                is_running = false;
                receiver = None;
            },
            CoroutineAction::NewSort(new_receiver, sender) => {
                is_running = false;
                sender.send(canvas_updater.get_list().clone()).unwrap();
                receiver = Some(new_receiver)
            }
            CoroutineAction::Start => {
                is_running = true;
            }
            CoroutineAction::ChangeLen(l) => {
                *canvas_updater.modify_list() = (1..=l as Num).collect();
            }
        }
    }
}

#[component]
fn App() -> Element {

    use_coroutine(async |rx| {
        // Delay::new(Duration::from_secs(5)).await;
        coroutine(rx).await
    });

    rsx! {

        // head {
        //     script {
        //         src: TAILWIND,
        //     }
        // }
        // Stylesheet { href: TAILWIND_CSS }
        Stylesheet { href: MAIN_CSS }
        canvas {
            id: "list-canvas",
            width: WIDTH,
            height: HEIGHT,
        }
        Panel {}
    }
}

use crate::consts::*;
use crate::sorts::SortType;
use crate::tools::checked_list::{CheckedList, ListAction};
use crate::Num;
use dioxus::core::Task;
use dioxus::prelude::*;
use futures::channel::mpsc;
use futures::channel::mpsc::Receiver;
use futures_timer::Delay;
use rodio::mixer::Mixer;
use rodio::source::SineWave;
use rodio::Source;
use std::cell::LazyCell;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Copy, Clone, Debug, Default)]
pub enum Highlight {
    #[default]
    None,
    Read,
    Write,
}

impl Highlight {
    fn get_color(self) -> &'static str {
        match self {
            Highlight::None => NEUTRAL_COLOR,
            Highlight::Read => READ_COLOR,
            Highlight::Write => WRITE_COLOR,
        }
    }
}

pub struct App {
    canvas_id: &'static str,
    list: Vec<Num>,
    max: Num,
    len: usize,
    entries_to_clear: HashSet<usize>,
    is_running: bool,
    delay: Duration,
    receiver: Option<Receiver<ListAction<Num>>>,
    sort_handle: Option<Task>,
    display_state: DisplayState,
    current_sort: Option<SortType>,
    sound_on: bool,
    sound_mixer: LazyCell<Option<&'static Mixer>>,
}

impl App {
    pub fn new(display_state: DisplayState) -> Self {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let f = || {
            let sink = rodio::DeviceSinkBuilder::open_default_sink().ok()?;

            let leak = Box::leak(Box::new(sink));

            Some(leak.mixer())
        };
        let sound_handle = LazyCell::new(f as _);

        let mut s = Self {
            canvas_id: "list-canvas",
            max: *list.iter().max().unwrap(),
            len: list.len(),
            list,
            entries_to_clear: HashSet::new(),
            is_running: false,
            delay: Duration::from_millis(100),
            receiver: None,
            sort_handle: None,
            display_state,
            current_sort: None,
            sound_on: false,
            sound_mixer: sound_handle,
        };
        s.update_all();
        s
    }

    pub fn get_list(&self) -> &Vec<Num> {
        &self.list
    }

    pub fn shuffle_list(&mut self) {
        fastrand::shuffle(&mut self.list);
        self.update_all();
    }
}

impl App {
    pub fn clear(&self) {
        document::eval(&format!(
            r#"
            let canvas = document.getElementById("{canvas_id}");
            let ctx = canvas.getContext("2d");
            ctx.clearRect(0, 0, canvas.width, canvas.height);
        "#,
            canvas_id = self.canvas_id,
        ));
    }

    pub fn update_entry(&self, idx: usize, new_highlight: Highlight) {
        let _eval = document::eval(&format!(
            r#"
            let canvas = document.getElementById("{canvas_id}");
            let ctx = canvas.getContext("2d");
            ctx.imageSmoothingEnabled = false;
            ctx.fillStyle = "{highlight}";
            ctx.clearRect({idx}, 0, 1, {start_height});
            ctx.fillRect({idx}, {start_height}, 1, {this_height});
        "#,
            canvas_id = self.canvas_id,
            highlight = new_highlight.get_color(),
            start_height = self.max - self.list[idx],
            this_height = self.list[idx],
        ));
    }

    pub fn update_all(&mut self) {
        self.clear();
        self.len = self.list.len();
        self.max = *self.list.iter().max().unwrap_or(&1);

        for idx in 0..self.list.len() {
            self.update_entry(idx, Highlight::None)
        }
    }

    pub fn update_entries_for_one(&mut self, entries: &[(usize, Highlight)]) {
        for &(idx, new_highlight) in entries {
            self.update_entry(idx, new_highlight);
        }
        let items = entries.iter().map(|(idx, _)| idx).copied();
        self.play_sounds(items.clone());
        self.entries_to_clear.extend(items);
    }

    pub fn proceed(&mut self, action: ListAction<Num>) {
        for &i in &self.entries_to_clear {
            self.update_entry(i, Highlight::None);
        }
        self.entries_to_clear.clear();

        let entries = match action {
            ListAction::Swap(a, b) => {
                self.list.swap(a, b);
                vec![
                    (a, Highlight::Write),
                    (b, Highlight::Write)
                ]
            }
            ListAction::Compare(a, b) => {
                [a, b]
                    .into_iter()
                    .flatten()
                    .map(|x| (x, Highlight::Read))
                    .collect()
            }
            ListAction::Write(i, v) => {
                self.list[i] = v;
                vec![(i, Highlight::Write)]
            }
        };
        self.update_entries_for_one(&entries);
    }

    pub fn play_sounds(&self, indices: impl IntoIterator<Item = usize>) {
        let freq = |idx| MIN_FREQ * (MAX_FREQ / MIN_FREQ).powf(self.list[idx] as f32 / self.max as f32);

        // println!("Playing: {freq}");
        let dur = self.delay
            .max(Duration::from_millis(500));
        // let volume = 100.0*self.delay.as_secs_f32().powi(2) / dur.as_secs_f32();

        let init_source = |freq| SineWave::new(freq);
        // let pass_freq_mul = 10.0;

        let source = |freq| init_source(freq)
            .amplify(10.0)
            // .amplify(volume)
            .take_duration(dur)
            // .fade_in(dur/2);
            .fade_out(dur/2);

        let sound = |idx| source(freq(idx));
        if let Some(mixer) = *self.sound_mixer && self.sound_on {
            for idx in indices {
                mixer.add(sound(idx));
            }
        }
    }
}

pub enum AppMessage {
    NewSort(SortType),
    StartOrStop,
    SwitchSound,
    Shuffle,
    ChangeLen(usize),
    ChangeDelay(Duration),
    UpdateAll,
}

impl App {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::StartOrStop => {
                self.is_running = !self.is_running;
                self.display_state.is_running.set(self.is_running);
            }
            AppMessage::Shuffle => {
                self.shuffle_list();
                if let Some(sort) = self.current_sort {
                    self.update(AppMessage::NewSort(sort))
                }
            }
            AppMessage::NewSort(sort) => {
                if let Some(handle) = self.sort_handle {
                    handle.cancel()
                }
                self.current_sort = Some(sort);

                self.display_state.sort.set(Some(sort));

                let (sender, receiver) = mpsc::channel(0);
                self.receiver = Some(receiver);

                let mut checked_list = CheckedList::new(self.list.clone(), sender);

                self.sort_handle = Some(spawn(async move {
                    sort.start(checked_list.as_mut()).await
                }));
            }
            AppMessage::ChangeDelay(d) => {
                self.display_state.delay.set(d.as_millis() as _);
                self.delay = d;
            }
            AppMessage::ChangeLen(l) => {
                self.sort_handle = None;
                self.is_running = false;
                self.display_state.is_running.set(false);
                self.receiver = None;
                self.list = (1..=l as Num).collect();
                self.len = l;
                self.max = l as Num;

                self.display_state.length.set(l);
                self.display_state.height.set(l);
            }
            AppMessage::UpdateAll => {
                self.update_all();
            }
            AppMessage::SwitchSound => {
                self.sound_on = !self.sound_on;
                self.display_state.sound_on.set(self.sound_on);
            }
        }
    }

    pub async fn idle(&mut self) -> bool {
        let Some(receiver) = self.receiver.as_mut() else {
            return true;
        };
        if self.is_running {
            let Ok(action) = receiver.recv().await else {
                self.is_running = false;
                self.display_state.is_running.set(false);
                return false;
            };
            self.proceed(action);
        }

        Delay::new(self.delay).await;
        false
    }
}

#[derive(Clone, Copy)]
pub struct DisplayState {
    pub is_running: Signal<bool>,
    pub sort: Signal<Option<SortType>>,
    pub length: Signal<usize>,
    pub height: Signal<usize>,
    pub delay: Signal<u64>,
    pub sound_on: Signal<bool>,
}

impl Default for DisplayState {
    fn default() -> Self {
        Self {
            is_running: Signal::new(false),
            sort: Signal::new(None),
            length: Signal::new(10),
            height: Signal::new(10),
            delay: Signal::new(100),
            sound_on: Signal::new(false),
        }
    }
}

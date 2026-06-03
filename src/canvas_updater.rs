use crate::checked_list::ListAction;
use crate::consts::*;
use crate::Num;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

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

pub struct CanvasUpdater {
    canvas_id: &'static str,
    list: Vec<Num>,
    max: Num,
    len: usize,
    width: Num,
    height: Num,
    entries_to_clear: HashSet<usize>,
}

impl CanvasUpdater {
    pub fn new(list: Vec<Num>) -> Self {
        Self {
            canvas_id: "list-canvas",
            max: *list.iter().max().unwrap(),
            len: list.len(),
            list,
            width: WIDTH,
            height: HEIGHT,
            entries_to_clear: HashSet::new(),
        }
    }

    pub fn get_list(&self) -> &Vec<Num> {
        &self.list
    }

    pub fn modify_list(&mut self) -> ListRef {
        ListRef {
            canvas_updater: self,
        }
    }

    pub fn shuffle_list(&mut self) {
        // TODO: Write better shuffler
        let map: HashMap<_, _> = self.list.drain(..).enumerate().collect();
        self.list.extend(map.into_values());
        self.update_all();
    }
}

impl CanvasUpdater {

    pub fn clear(&self) {
        document::eval(&format!(
            r#"
            let canvas = document.getElementById("{canvas_id}");
            let ctx = canvas.getContext("2d");
            ctx.clearRect(0, 0, {width}, {height});
        "#,
            canvas_id = self.canvas_id,
            width = self.width,
            height = self.height,
        ));
    }

    pub fn update_entry(&self, idx: usize, new_highlight: Highlight) {
        document::eval(&format!(
            r#"
            let canvas = document.getElementById("{canvas_id}");
            let ctx = canvas.getContext("2d");
            ctx.fillStyle = "{highlight}";
            ctx.clearRect({this_offset}, 0, {this_width}, {height});
            ctx.fillRect({this_offset}, {this_height}, {this_width}, {height})
        "#,
            canvas_id = self.canvas_id,
            highlight = new_highlight.get_color(),
            // The + 1 is to get rid of the annoying gaps
            this_width = self.width/self.len as Num + 1,
            this_offset = self.width*idx as Num/self.len as Num,
            this_height = self.height - self.height*self.list[idx]/self.max,
            height = self.height,
        ));
    }

    pub fn update_all(&self) {
        for (idx, &val) in self.list.iter().enumerate() {
            self.update_entry(idx, Highlight::None)
        }
    }

    pub fn update_entry_for_one(&mut self, idx: usize, new_highlight: Highlight) {
        self.update_entry(idx, new_highlight);
        self.entries_to_clear.insert(idx);
    }

    pub fn proceed(&mut self, action: ListAction) {

        for &i in &self.entries_to_clear {
            self.update_entry(i, Highlight::None);
        }

        match action {
            ListAction::Swap(a, b) => {
                self.list.swap(a, b);
                self.update_entry_for_one(a, Highlight::Write);
                self.update_entry_for_one(b, Highlight::Write);
            }
            ListAction::Compare(a, b) => {
                if let Some(a) = a {
                    self.update_entry_for_one(a, Highlight::Read);
                }
                if let Some(b) = b {
                    self.update_entry_for_one(b, Highlight::Read);
                }
            }
        }
    }
}

pub struct ListRef<'a> {
    canvas_updater: &'a mut CanvasUpdater
}

impl<'a> Deref for ListRef<'a> {
    type Target = Vec<Num>;

    fn deref(&self) -> &Self::Target {
        &self.canvas_updater.list
    }
}

impl<'a> DerefMut for ListRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.canvas_updater.list
    }
}

impl<'a> Drop for ListRef<'a> {
    fn drop(&mut self) {
        let inner = &mut self.canvas_updater;
        inner.max = *inner.list.iter().max().unwrap();
        inner.len = inner.list.len();
        inner.update_all()
    }
}
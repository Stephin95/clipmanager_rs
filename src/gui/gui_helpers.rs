
use std::ops::{Bound, RangeBounds};
use crate::clip_db::ClipboardEntry;
trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

fn slice_button_text<'a>(clip_text: String) -> String {
    let string_length = clip_text.len();
    let max_slice_length=50;
    if string_length > max_slice_length {
        format!("{}{}",clip_text.replace("\n", " ").slice(..max_slice_length).to_owned(),"...")
    } else {
        clip_text.replace("\n", " ")
    }
}
pub fn shorten_entry(clip_entry:&ClipboardEntry)->ClipboardEntry{
        ClipboardEntry {
            clip_text: slice_button_text(clip_entry.clip_text.clone()),
            id: clip_entry.id.clone(),
            clip_bin:clip_entry.clip_bin.clone()
        }
}
// pub fn format_button_text(clipboard_entry: &Vec<ClipboardEntry>) -> Vec<ClipboardEntry> {
//     clipboard_entry
//         .iter()
//         .map(|clip_entry| shorten_entry(clip_entry))
//         .collect()
// }

// pub fn create_button_text(clipboard_entry: &Vec<ClipboardEntry>) -> Vec<ClipboardEntry> {
//     clipboard_entry
//         .iter().rev()
//         .map(|clip_entry| {let mut button_clip_entry=clip_entry.clone();button_clip_entry.clip_text=button_clip_entry.clip_text.replace("\n", "\\n");button_clip_entry})
//         .collect()}
#[derive(Debug, Clone)]
pub enum Message {
    Entry(i32),
    ToastClose(usize),
    EventOccurred(iced_native::Event),
}

diesel::table! {
    pub clipboard_entries (id){
        id -> Integer,
        clip_text -> Text,
        
    }
}
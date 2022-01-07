table! {
    handler_events (id) {
        id -> Nullable<Integer>,
        timestamp -> Timestamp,
        handler -> Text,
        host -> Nullable<Text>,
        uri -> Nullable<Text>,
        src_ip -> Nullable<Text>,
        info -> Nullable<Text>,
    }
}

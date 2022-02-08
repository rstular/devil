table! {
    handler_events (id) {
        id -> Nullable<Integer>,
        timestamp -> Timestamp,
        handler -> Text,
        host -> Nullable<Text>,
        uri -> Nullable<Text>,
        src_ip -> Nullable<Inet>,
        payload -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        details -> Nullable<Text>,
        x_forwarded_for -> Nullable<Text>,
    }
}

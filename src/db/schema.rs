table! {
    handler_events (id) {
        id -> Integer,
        timestamp -> Timestamp,
        handler -> Text,
        subhandler -> Nullable<Text>,
        host -> Nullable<Text>,
        uri -> Nullable<Text>,
        src_ip -> Nullable<Inet>,
        payload -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        handler_data -> Nullable<Text>,
        x_forwarded_for -> Nullable<Text>,
    }
}

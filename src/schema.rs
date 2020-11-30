table! {
    main_table (id) {
        id -> Int4,
        reference_time -> Timestamptz,
        insert_time -> Timestamptz,
        data -> Jsonb,
        tags -> Jsonb,
        bucket_name -> Text,
    }
}

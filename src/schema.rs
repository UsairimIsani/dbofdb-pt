table! {
    main_table (insert_time) {
        reference_time -> Timestamptz,
        insert_time -> Timestamptz,
        data -> Jsonb,
        tags -> Jsonb,
        bucket_name -> Text,
    }
}

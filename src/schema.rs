table! {
    measurements (id) {
        id -> Unsigned<Bigint>,
        temp_celsius -> Double,
        humidity -> Double,
        comment -> Nullable<Tinytext>,
    }
}

// @generated automatically by Diesel CLI.
diesel::table! {
    albums (id){
        id->Integer,
        artist->Text,
        year->Integer,
        name ->Text,
        is_released -> Bool
    }
    
}

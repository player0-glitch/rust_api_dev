use diesel::prelude::*;
use crate::schema::albums::{self, dsl::albums as all_albums};

#[derive(Queryable,Selectable)]
#[diesel(table_name=crate::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Album{
    pub id:i32,
    pub artist:String,
    pub name:String,
    pub year:i32,
    pub is_released:bool,
}

//This is the new struct we'll use as a new db entry
#[derive(Insertable,Selectable)]
#[diesel(table_name=crate::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAlbum{
    pub artist:String,
    pub name:String,
    year:i32,
    pub is_released:bool,
}

//Functionality of our database
impl Album {

    //will just show a specific 'albums' in the database
    pub fn show_album(id:i32,db_connection:&mut SqliteConnection)->Vec<Self>{
            all_albums
            .select(Album::as_select())
            .find(id)
            .load::<Album>(db_connection)    //I need to learn about load function in Rust
            .expect("Failed To Load ALl Entries From SQLite database")
    }

    //Will show all the 'albums' in the database
    pub fn show_all(db_connection:&mut SqliteConnection)->Vec<Self>{
        all_albums
            .order(albums::id.desc())
            .select(Album::as_select())
            .load::<Self>(db_connection)
            .expect("Failed To show all Albums")
    }

    pub fn update_by_id(id:i32,album:NewAlbum,db_connection:&mut SqliteConnection)->bool{
        //create 'use' for a easier new entryuseK
        use crate::schema::albums::dsl::{artist as a,name as n,year as y,is_released as r};

        //Destructuring
        let NewAlbum {artist, name, year, is_released }=album;
       
        //using all the books we have in the database  to find the right albumbs
        diesel::update(all_albums.find(id))
            .set((a.eq(artist),n.eq(name),y.eq(year),r.eq(is_released)))
            .execute(db_connection)
            .is_ok()//returns a true value is the operation is success or a flase value when

    }
    
}

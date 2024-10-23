
use diesel::prelude::*; //since we're using sqlite we don't care too much aboutanything else
use schema::albums; //name of the table
use schema::albums::dsl::albums as all_allbums; 

#[derive(self::Queryable,Selectable)]    //so that we can query it in and out of out data table
#[diesel(table_name=albums)]     //this is the table we'll be using 
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]//The database type we're using is SQlite3
/*but it  */
pub struct Album{ 
    pub id:i32,
    pub artist: String,
    pub release_year: i32,
    pub sales:i32,
    pub released:bool,
}


//this macro will allow us to insert this struct entry into 
//the database
#[derive(self::Insertable)]
#[diesel(table_name=albums)]
pub struct NewAlbum{

    pub artist:String,
    pub year: u32,
    pub sales: i32,
    pub released:bool,
}

//Method used to show entries of our database
impl Album {
    pub fn show(id:i32,connection:&SqliteConnection)->Vec<Album> {
        //using the schema we referenced
        all_allbums
            .find(id)
            .load::<Album>(connection)
            .expect("Error Loading database table (Albums)");
    }
   
}

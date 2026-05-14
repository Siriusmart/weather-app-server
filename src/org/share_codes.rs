use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, Selectable,
    prelude::{Identifiable, Insertable, Queryable},
    sqlite::Sqlite,
};
use rand::{RngExt, rng};

use crate::DatabaseConnection;

diesel::table! {
    share_codes(share_code) {
        share_code -> Text,
        metoffice_key -> Text
    }
}

#[derive(Selectable, Queryable, Insertable, Identifiable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = share_codes)]
#[diesel(primary_key(share_code))]
pub struct ShareCode {
    share_code: String,
    metoffice_key: String,
}

impl ShareCode {
    pub fn share_code(&self) -> &str {
        &self.share_code
    }

    pub fn metoffice_key(&self) -> &str {
        &self.metoffice_key
    }

    pub fn create_metoffice(
        conn: &mut DatabaseConnection,
        metoffice_key: &str,
    ) -> Result<Self, diesel::result::Error> {
        let mut rng = rng();

        let code = loop {
            let code = (0..6)
                .map(|_| rng.random_range(b'A'..=b'Z') as char)
                .collect::<String>();

            if Self::get(conn, &code)?.is_none() {
                break code;
            }
        };

        diesel::insert_into(share_codes::table)
            .values(Self {
                share_code: code,
                metoffice_key: metoffice_key.to_string(),
            })
            .execute(conn)?;

        Self::get_by_metoffice_key(conn, metoffice_key).map(Option::unwrap)
    }

    pub fn get(
        conn: &mut DatabaseConnection,
        share_code: &str,
    ) -> Result<Option<Self>, diesel::result::Error> {
        share_codes::table
            .filter(share_codes::share_code.eq(share_code))
            .select(share_codes::all_columns)
            .get_result(conn)
            .optional()
    }

    pub fn get_by_metoffice_key(
        conn: &mut DatabaseConnection,
        metoffice_key: &str,
    ) -> Result<Option<Self>, diesel::result::Error> {
        share_codes::table
            .filter(share_codes::metoffice_key.eq(metoffice_key))
            .select(share_codes::all_columns)
            .get_result(conn)
            .optional()
    }

    /// returns true if anything remove, false otherwise
    pub fn remove_by_metoffice_key(
        conn: &mut DatabaseConnection,
        metoffice_key: &str,
    ) -> Result<bool, diesel::result::Error> {
        Ok(diesel::delete(share_codes::table)
            .filter(share_codes::metoffice_key.eq(metoffice_key))
            .execute(conn)?
            != 0)
    }
}

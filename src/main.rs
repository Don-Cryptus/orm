#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{DatabaseConnection, DbErr};

mod connection;

fn main() {
    println!("Hello, world!");
}

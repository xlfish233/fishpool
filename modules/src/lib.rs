mod jwt;

mod user;

mod response;
mod result;

pub mod db;

pub mod salt;

use salvo::prelude::*;
use salvo::session::SessionDepotExt;

use crate::app::models::dashboard::Credit;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CREDITS: Vec<Credit> = vec![
        Credit::new("Tjac", "Helping with debugging backend routes, providing a python server and design inspiration for the dashboard", Some("https://github.com/tjac/tsto_server".to_owned())),
        Credit::new("BodNJenie", "Helping out with protocol buffers and providig a c++ server", Some("https://github.com/bodnjenie14/Tsto---Simpsons-Tapped-Out---Private-Server".to_owned())),
        Credit::new("Whoops I Paniced", "Providing a nodejs server which greatly inspired the code structure of this server", Some("https://github.com/TappedOutReborn/GameServer-Reborn".to_owned())),
        Credit::new("tehfens", "Providing the apk-patcher and giving advice on how to extract the correct protocol buffers from the apk", Some("https://github.com/d-fens/tstoapkpatcher".to_owned())),
        Credit::new("Ethan", "Providing the shared login API which is currently in use by eg. Rudeboy", Some("https://api.tsto.app".to_owned())),
        Credit::new("Joee", "Working hard on the DLCs and helping on debugging backend routes", None),
        Credit::new("Rudeboy™", "Helping on debugging backend routes and initial setup of a test server to get into the waters", None),
        Credit::new("Dractiums", "Helping on debugging backend routes and initial setup of a test server to get into the waters", None),
    ];
}

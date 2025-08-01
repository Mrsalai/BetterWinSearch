slint::include_modules!();
use std::ffi::OsStr;
use std::os::windows::fs::MetadataExt;
use std::time::SystemTime;
use std::{fs, path};
use std::fs::{File};
use std::path::{PathBuf};
use std::thread;
use rusqlite::{Connection, ToSql};
use slint::{Image, Weak};
use chrono::prelude::*;
use chrono::offset::MappedLocalTime;

pub fn main()
{

    let ui = MainWindow::new().unwrap();
    let ui_handle: Weak<MainWindow> = ui.as_weak();
    let ui: MainWindow = ui_handle.unwrap();


    let dbcon  = Connection::open("index.db3").unwrap(); //Db init
    dbcon.execute("
    CREATE TABLE IF NOT EXISTS apps (
        name TEXT,
        location TEXT,
        image BLOB
    )",()).unwrap();
    //dbcon.execute("
    //CREATE TABLE IF NOT EXISTS folders (
    //path TEXT
    //)",()).unwrap();
    dbcon.execute("CREATE TABLE lastscandate(date, TEXT)",());
    //deep_scan();
    sync(&dbcon);
    ui.run();
}

pub fn deep_scan()
{
    thread::spawn(||
    {
        let scannedpath = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs";
        directorysearch(scannedpath);
        println!("Programs done")

    });  //return applistresult
    thread::spawn(||
    {
        let scannedpath  = "C:\\Program Files";
        directorysearch(scannedpath);
        println!("Program files done")

    });
    thread::spawn(||
    {
        let scannedpath = "C:\\Program Files (X86)";
        directorysearch(scannedpath);
        println!("Programfiles x86 done")

    });

}    
pub fn directorysearch(scannedpath: &str,)
{
    let dbcon = rusqlite::Connection::open("index.db3").unwrap();
    for file in fs::read_dir(scannedpath).unwrap()
    {        
        let pathresult = file.as_ref().unwrap().path();
        if pathresult.extension().is_some_and(|ext: &OsStr| ext == "lnk" || ext =="exe")
        {
            writetodb(None,pathresult.as_os_str(),None,&dbcon)
        }



        else 
        {
            if pathresult.is_dir()
            {
                scanchildprc(&pathresult,&dbcon);
                dbcon.execute("INSERT INTO folders(path) VALUES (?1)",[&pathresult.to_str()]);
            }

        }
    }
    
}

    //return applistresult
pub fn scanchildprc(pathresult:&PathBuf,dbcon:&Connection)
{
    if let Ok(entries) = fs::read_dir(pathresult)
    {    
        for file in entries
        {   
            let subpathresult = file.as_ref().unwrap().path();
            if subpathresult.extension().is_some_and(|ext: &OsStr| ext == "lnk" || ext=="exe")
                {
                    writetodb(None,subpathresult.as_os_str(),None,&dbcon)
                    
                }
            else 
                {
                    scanchildprc(&subpathresult,&dbcon); 
                }
        }   
    }

}

pub fn writetodb(name: Option<&str>,location: &OsStr,blob: Option<&OsStr>,dbcon:&Connection)
{
 
    dbcon.execute("INSERT INTO apps (name, location, image) VALUES (?1, ?2, ?3)",("none",location.to_str().unwrap(),"none"),);


}




pub fn sync(dbcon:&Connection)
{
    let customtime = chrono::offset::Local::now();

    let scannedpath = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs";
    for file in fs::read_dir(scannedpath).unwrap()
    {   
        let creationtime = file.unwrap().metadata().unwrap().created();
        //println!("{}",creationtime.unwrap().);

    }  //return applistresult
    dbcon.execute("INSERT INTO lastscandate (date) VALUES (?1)", [customtime.to_string()]);
    println!("{}",customtime.to_string());

}
//check if new folder is found or old one is not found and check for shortcuts, no deep scans
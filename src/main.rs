slint::include_modules!();
use std::ffi::OsStr;
use std::{fs};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::thread;
use rusqlite::Connection;

use slint::{Weak};


pub fn main() -> Result<(), slint::PlatformError> {
    let dbcon  = Connection::open("index.db3").unwrap();
    let ui = MainWindow::new()?;
    let ui_handle: Weak<MainWindow> = ui.as_weak();
    let ui: MainWindow = ui_handle.unwrap();

    
    get_shorts();
    ui.run()

}

pub fn get_shorts()
{
    thread::spawn(||
    {
        let scannedpath = "C:\\Program Files (X86)";
        directorysearch(scannedpath);

        println!("Programs done")

    });  //return applistresult
    thread::spawn(||
    {
        let scannedpath  = "C:\\Program Files (X86)";
        directorysearch(scannedpath);
        println!("Programs done")

    });
    thread::spawn(||
    {
        let scannedpath = "C:\\Program Files (X86)";
        directorysearch(scannedpath);
        println!("Programs done")

    });

}    
pub fn directorysearch(scannedpath: &str)
{
    for file in fs::read_dir(scannedpath).unwrap()
    {        
        let pathresult = file.as_ref().unwrap().path();
        if pathresult.extension().is_some_and(|ext| ext == "lnk")
        {
            todo!("db")

        }


        if pathresult.extension().is_some_and(|ext| ext =="exe")
        {
            todo!("db")
        }


        else 
        {
            if file.unwrap().metadata().unwrap().is_dir()
            {
                scanchildprc(&pathresult);

            }
        }
    }
}

    //return applistresult
pub fn scanchildprc(pathresult:&PathBuf)
{
    let startprogramdirs: File;
    if let Ok(entries) = fs::read_dir(pathresult)
    {    
        for file in entries
        {   
                let subpathresult = file.as_ref().unwrap().path();
                if &subpathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "lnk"
                {
                    todo!("db")

                    
                }
                if &subpathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "exe"
                {
                    
                    todo!("db")

                }
                else 
                {
                    scanchildprc(&subpathresult);
                }
        }   
    }


}   
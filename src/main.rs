slint::include_modules!();
use std::{fs};
use std::fs::{OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
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

pub fn get_shorts()// -> Vec<slint::SharedString>
{


    thread::spawn(||
    {
        let mut startprogramdirs = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./index/startprogramdirs.json")
        .unwrap();

        for file in fs::read_dir("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs").unwrap()
        {        
            let pathresult = file.as_ref().unwrap().path();
            if &pathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "lnk"
            {
                startprogramdirs.write(&pathresult.to_owned().into_os_string().as_encoded_bytes()).unwrap();
                startprogramdirs.write("\n".as_bytes()).unwrap();
            }


            if &pathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "exe"
            {
                startprogramdirs.write(&pathresult.to_owned().into_os_string().as_encoded_bytes()).unwrap();
                startprogramdirs.write("\n".as_bytes()).unwrap();
            }


            else 
            {
                if file.unwrap().metadata().unwrap().is_dir()
                {
                    scanchildprc(&pathresult);

                }
            }
        }
        println!("Programs done")

    });

    thread::spawn(||
    {
        let mut startprogramdirs = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./index/startprogramdirs.json")
        .unwrap();

        for file in fs::read_dir("C:\\Program Files (X86)").unwrap()
        {        
            let pathresult = file.as_ref().unwrap().path();
            if &pathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "lnk"
            {
                startprogramdirs.write(pathresult.to_owned().into_os_string().as_encoded_bytes()).unwrap();
                startprogramdirs.write("\n".as_bytes()).unwrap();
            }

            if &pathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "exe"
            {
                startprogramdirs.write(pathresult.to_owned().into_os_string().as_encoded_bytes()).unwrap();
                startprogramdirs.write("\n".as_bytes()).unwrap();
            }

            else 
            {
                if file.unwrap().metadata().unwrap().is_dir()
                {
                    scanchildprc(&pathresult);

                }
            }
        }
        println!("Programfiles x86 done")
    });


    thread::spawn(||
    {
        let mut startprogramdirs = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./index/startprogramdirs.json")
        .unwrap();

        for file in fs::read_dir("C:\\Program Files").unwrap()
        {      
            let pathresult = file.as_ref().unwrap().path();
            if &pathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "lnk"
            {
                startprogramdirs.write(pathresult.to_owned().into_os_string().as_encoded_bytes()).unwrap();
                startprogramdirs.write("\n".as_bytes()).unwrap();
            }

            if &pathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "exe"
            {
                startprogramdirs.write(pathresult.to_owned().into_os_string().as_encoded_bytes()).unwrap();
                startprogramdirs.write("\n".as_bytes()).unwrap();
            }


            else 
            {
                if file.unwrap().metadata().unwrap().is_dir()
                {
                    scanchildprc(&pathresult);

                }
            }
        }
        println!("Programfiles done")

    });
 
    //return applistresult
}    


    //return applistresult
pub fn scanchildprc(pathresult:&PathBuf)
{
        let mut startprogramdirs = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./index/startprogramdirs.json")
        .unwrap();
    //for file in fs::read_dir(pathresult).unwrap_or_default()
    if let Ok(entries) = fs::read_dir(pathresult)
    {    
        for file in entries
        {   
                let subpathresult = file.as_ref().unwrap().path();
                if &subpathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "lnk"
                {
                    startprogramdirs.write(&subpathresult.to_string_lossy().to_string().as_bytes()).unwrap();
                    startprogramdirs.write("\n".as_bytes()).unwrap();
                    
                }
                if &subpathresult.extension().unwrap_or_default().to_string_lossy().to_string() == "exe"
                {
                    startprogramdirs.write(&subpathresult.to_string_lossy().to_string().as_bytes()).unwrap();
                    startprogramdirs.write("\n".as_bytes()).unwrap();
                    
                }
                else 
                {
                    scanchildprc(&subpathresult);
                }
        }   
    }
}   
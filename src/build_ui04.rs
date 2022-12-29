extern crate gtk;
extern crate exif;
// extern crate chrono;
extern crate regex;
// extern crate walkdir;
extern crate gdk_pixbuf;
use gtk::prelude::*;
// use gtk::gdk;
use gtk::glib;

// use gtk::gdk_pixbuf::{Pixbuf};
use std::path::{Path};
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
// use walkdir::WalkDir;

// use chrono::prelude::*;
use gtk::{   
    ProgressBar,
//    prelude::ProgressBarExt,
    Label,
    FileChooserDialog,
    FileChooserAction,
    Notebook,
    Button,
    ComboBoxText,
    Entry,
    Grid,
    prelude::GridExt,
    prelude::WidgetExt,
//    prelude::CssProviderExt,
    prelude::GtkWindowExt,
};

const STYLE: &str = "
button.text-button {
    /* If we don't put it, the yellow background won't be visible */
    border-style: solid;
    border-width: 5px;
    border-color: #888888;
    background-image: none;
}
notebook tab:checked {
    border-style: solid;
    border-width: 2px;
    border-color: blue;
}
#MessTitle {
    font-size: large;
}
#MessValue {
    border-style: solid;
    border-width: 2px;
    border-color: #888888;
    min-height: 30px;
}
#tab1 {
    font-weight: bold;   
    border-style: solid;
    border-width: 2px;
    border-color: #888888;
} 
#tab2 {
    font-weight: bold;   
    border-style: solid;
    border-width: 2px;
    border-color: #888888;
} 
#tab3 {
    font-weight: bold;   
    border-style: solid;
    border-width: 2px;
    border-color: #888888;
} 
/*  progress bar height */
#bar1, progress, trough {
   color: black;
   font-weight: bold;   
   min-height: 15px;
}";

pub fn build_ui(application: &gtk::Application) {

      let provider = gtk::CssProvider::new();
      provider.load_from_data(STYLE.as_bytes());
      gtk::StyleContext::add_provider_for_display(
              &gtk::gdk::Display::default().expect("Could not connect to a display"),
              &provider,
              gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
      );      

    let window = gtk::ApplicationWindow::new(application);
    let wtitle = format!("Mega-ls conversion: {}.{}.{}",gtk::major_version(), gtk::minor_version(), gtk::micro_version());

    window.set_title(Some(&wtitle));
//    window.set_position(WindowPosition::Center);
    window.set_size_request(800, 500);
    
    let messageclear_button = Button::with_label("Clear");
    let messagetitle_label = Label::new(Some("Message: "));
    gtk::traits::WidgetExt::set_widget_name(&messagetitle_label, "MessTitle");
    let messageval_label = Label::new(Some("Message area"));
    gtk::traits::WidgetExt::set_widget_name(&messageval_label, "MessValue");
    
    let progressreset_button = Button::with_label("Reset");
    let progress_progressbar = ProgressBar::new();
    progress_progressbar.set_show_text(true);
    gtk::traits::WidgetExt::set_widget_name(&progress_progressbar, "bar1");

    let descline1_label = Label::new(Some("Need to run the following terminal commands before using this app:"));
    let descline2_label = Label::new(Some("rclone lsf --files-only -R --csv --format pst /pathofdirectory | sort > outputfile1"));
    let descline3_label = Label::new(Some("mega-ls -l -r --time-format=ISO6081_WITH_TIME /cloudpathofdirectory > outputfile2"));
    let descline4_label = Label::new(Some("Run this app using outputfile2 as input file"));
    let descline5_label = Label::new(Some("Need to run the following terminal command after using this app:"));
    let descline6_label = Label::new(Some("sort targetfile > outputfile3"));
    let descline7_label = Label::new(Some("compare outputfile1 with outputfile3. I use Meld."));

    let cdirectory1_button = Button::with_label("mega-ls -lr input file");
    let cdirectory1_combobox = ComboBoxText::new();
    cdirectory1_combobox.set_hexpand(true);

    let cdirectory_o_button = Button::with_label("Target Directory");
    let cdirectory_o_combobox = ComboBoxText::new();
    cdirectory_o_combobox.set_hexpand(true);
    let cnumrows_label = Label::new(Some("number of rows:"));
    let cnumrows_entry = Entry::new();
    let cgetrows_button = Button::with_label("get total rows");
    let ctarget_label = Label::new(Some("Target file name:"));
    let ctarget_entry = Entry::new();
    ctarget_entry.set_text("megaxx.txt");
    let cutc_label = Label::new(Some("UTC offset"));
    let cutc_entry = Entry::new();
    cutc_entry.set_text("0");

    let cexconv_button = Button::with_label("Execute Conversion");

    let vbox1 = Grid::new();
    vbox1.set_column_spacing(5);
    vbox1.set_row_spacing(5);
//    item, column, row, column length, row length
    vbox1.attach(&cdirectory1_button, 1, 1 , 2, 1);
    vbox1.attach(&cdirectory1_combobox, 3, 1 , 3, 1);
    vbox1.attach(&cdirectory_o_button, 6, 1 , 2, 1);
    vbox1.attach(&cdirectory_o_combobox, 8, 1 , 2, 1);
    vbox1.attach(&cnumrows_label, 2, 3 , 1, 1);
    vbox1.attach(&cgetrows_button, 1, 4 , 1, 1);    
    vbox1.attach(&cnumrows_entry, 2, 4 , 1, 1);
    vbox1.attach(&ctarget_label, 6, 3 , 1, 1);
    vbox1.attach(&ctarget_entry, 8, 3 , 2, 1);
    vbox1.attach(&cutc_label, 6, 4 , 1, 1);
    vbox1.attach(&cutc_entry, 8, 4 , 2, 1);
    vbox1.attach(&cexconv_button, 9, 10 , 1, 1); 
    let vnotebook = Notebook::new();
    let tab1_label = Label::new(Some("  mega-ls convert  "));
    gtk::traits::WidgetExt::set_widget_name(&tab1_label, "tab1");
    vnotebook.append_page(&vbox1, Some(&tab1_label));

    let vbox = Grid::new();
    vbox.set_column_spacing(5);
    vbox.set_row_spacing(5);
    
    vbox.attach(&messageclear_button, 0, 0, 1, 1);
    vbox.attach(&messagetitle_label, 1, 0, 1, 1);
    vbox.attach(&messageval_label, 2, 0, 8, 1);
    vbox.attach(&vnotebook, 0, 2, 10, 10);
    vbox.attach(&progressreset_button, 0, 13, 1, 1);
    vbox.attach(&progress_progressbar, 1, 13, 10, 1);
    vbox.attach(&descline1_label, 1, 14, 10, 1);
    vbox.attach(&descline2_label, 1, 15, 10, 1);
    vbox.attach(&descline3_label, 1, 16, 10, 1);
    vbox.attach(&descline4_label, 1, 17, 10, 1);
    vbox.attach(&descline5_label, 1, 18, 10, 1);
    vbox.attach(&descline6_label, 1, 19, 10, 1);
    vbox.attach(&descline7_label, 1, 20, 10, 1);

    window.set_child(Some(&vbox));
    window.set_destroy_with_parent(true);
    window.show(); 

//----------------- clear message area  button start -----------------------------------
    messageclear_button.connect_clicked(glib::clone!(@weak messageval_label => move|_| {
        
            messageval_label.set_text("message area cleared");
     
    }));
//----------------- clear message area  button end -----------------------------------

//----------------- reset progress area  button start -----------------------------------
   progressreset_button.connect_clicked(glib::clone!(@weak progress_progressbar, @weak messageval_label => move|_| {
        
            progress_progressbar.set_fraction(0.0);
            messageval_label.set_text("progress bar reset");
     
    }));
//----------------- clear message area  button end -----------------------------------

//----------------- source file  button start -----------------------------------
    cdirectory1_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory1_combobox, @weak messageval_label => move|_| {
   
        messageval_label.set_text("getting directory");

        let dialog = FileChooserDialog::new(
            Some("Choose a XML file"),
            Some(&window),
            FileChooserAction::Open,
            &[("Open", gtk::ResponseType::Accept), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Accept {
                if let Some(filename) = d.file() {
                    if let Some(filepath) = filename.path() {
                        cdirectory1_combobox.prepend_text(&filepath.display().to_string());
                        cdirectory1_combobox.set_active(Some(0));
                        messageval_label.set_text("XML file selected");
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* xml file: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* xml file: ERROR GETTING FILR **********</span>");
                }
            }
            if messageval_label.text() == "getting directory 1" {
                messageval_label.set_markup("<span color=\"#FF000000\">********* xml file: ERROR  OPEN  button not selected **********</span>");
            }
            d.close();
        });
        dialog.show();

      
    }));
//----------------- source file  button end -----------------------------------

//----------------- target directory button start -----------------------------------
    cdirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory_o_combobox, @weak messageval_label => move|_| {
    
        messageval_label.set_text("getting directory");

        let dialog = FileChooserDialog::new(
            Some("Choose output Directory 1"),
            Some(&window),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.file() {
                    if let Some(folderpath) = foldername.path() {
                        cdirectory_o_combobox.prepend_text(&folderpath.display().to_string());
                        cdirectory_o_combobox.set_active(Some(0));
                        messageval_label.set_text("output directory selected");
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* output directory: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* output directory: ERROR GETTING FOLDER **********</span>");
                }
            }
            if messageval_label.text() == "getting directory 1" {
                messageval_label.set_markup("<span color=\"#FF000000\">********* output directory: ERROR  OPEN  button not selected **********</span>");
            }
            d.close();
        });
        dialog.show();
    }));
//----------------- target directory button end -----------------------------------
//----------------- get rows button start -----------------------------------
    cgetrows_button.connect_clicked(glib::clone!(@weak cdirectory1_combobox, @weak cnumrows_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {
        if let Some(filename) = cdirectory1_combobox.active_text() {
            let str_filename = filename.to_string();
            if Path::new(&str_filename).exists() {
                let mut bolok = true;
                let file = File::open(str_filename).unwrap();
                let mut reader = BufReader::new(file);
                let mut line = String::new();
                let mut linenum: i64 = 0;
                let mut count = 0;
                let mut incrcount = 100000;
                loop {
                   match reader.read_line(&mut line) {
                      Ok(bytes_read) => {
                          // EOF: save last file address to restart from this address for next run
                          if bytes_read == 0 {
                              break;
                          }
                          linenum = linenum + 1;
                          count = count + 1;
                          if count > incrcount {
                              incrcount = incrcount + 100000;
                              let progressfr: f64 = count as f64 / 100000000 as f64;
                              progress_progressbar.set_fraction(progressfr);
                              while glib::MainContext::pending(&glib::MainContext::default()) {
                                 glib::MainContext::iteration(&glib::MainContext::default(),true);
                              }
                          }
                      }
                      Err(_err) => {
                          messageval_label.set_markup("<span color=\"#FF000000\">* error reading xml file: do file i and iconv **********</span>");
                          bolok = false;   
                          break;
                      }
                   };
                }
                if bolok {       
                    let numrowtext = format!("{}",linenum);
                    cnumrows_entry.set_text(&numrowtext);
                    messageval_label.set_text("number of rows has been set");
                    progress_progressbar.set_fraction(1.0);
                    while glib::MainContext::pending(&glib::MainContext::default()) {
                        glib::MainContext::iteration(&glib::MainContext::default(),true);
                    }
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* source file does not exist **********</span>");
            }
                
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING FROM DIRECTORY IN COMBOBOX **********</span>");
        }
    }));
//----------------- get rows button end -----------------------------------
    
//----------------- convert button start -----------------------------------
    cexconv_button.connect_clicked(glib::clone!(@weak cdirectory1_combobox, @weak cdirectory_o_combobox, @weak ctarget_entry, @weak cnumrows_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {
// files must be in utf-8 format
// linux command file -i filename will show format
// linux command iconv -f format -t UTF-8 filename -o outputfile    will convert file to UTF-8   
        let mut bolok = true;
        let mut numrows: i64 = 1;
        let mut targetfullname = format!("");
        progress_progressbar.set_fraction(0.5);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }
        if let Some(dirname) = cdirectory_o_combobox.active_text() {
            let str_dirname = dirname.to_string();
            if Path::new(&str_dirname).exists() {
                let strtarget = ctarget_entry.text();
                if strtarget.len() < 4 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* target name less than 4 characters **********</span>");
                    bolok = false;
                } else {
                    if !strtarget.contains(".") { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* target name does not have a file type (ie xx.xxx) **********</span>");
                        bolok = false;
                    } else {
                        let lrperpos = strtarget.rfind(".").unwrap();
                        if (strtarget.len() - lrperpos) < 4 {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* target name does not have a valid type (must be at least 3 characters **********</span>");
                            bolok = false;
                        } else {
                            let lfperpos = strtarget.find(".").unwrap();
                            if lfperpos < 3 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* target name is least than 3 characters **********</span>");
                                bolok = false;
                            } else {
                                targetfullname = format!("{}/{}", str_dirname, strtarget);
                                if Path::new(&targetfullname).exists() {
                                    messageval_label.set_markup("<span color=\"#FF000000\">********* target name already exists **********</span>");
                                    bolok = false;
                                }
                            }
                        }
                    }
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* target directory does not exist **********</span>");
                bolok = false;
            }
                
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING TARGET DIRECTORY IN COMBOBOX **********</span>");
            bolok = false;
        }
        if bolok {
            let strnumrows = cnumrows_entry.text();
            numrows = strnumrows.parse().unwrap_or(-99);
            if numrows < 10 {
                messageval_label.set_markup("<span color=\"#FF000000\">********* INVALID NUMBER IN NUMBER OF ROWS ENTRY **********</span>");
                bolok = false;
            }
        }
        if bolok {         
          if let Some(filename) = cdirectory1_combobox.active_text() {
            let str_filename = filename.to_string();
            let str_filenamex = str_filename.clone();
            if Path::new(&str_filename).exists() {
                // Open the file in read-only mode (ignoring errors).
                let file = File::open(str_filenamex).unwrap(); 
                let mut reader = BufReader::new(file);
                let mut targetfile = File::create(targetfullname).unwrap();
                let mut line = String::new();
                let mut linenum = 0;
                let mut sdir = "--".to_string();
                let mut topdir = String::new();
                loop {
                   match reader.read_line(&mut line) {
                      Ok(bytes_read) => {
                          // EOF: save last file address to restart from this address for next run
//                          writeln!(&mut targetfile,"line num:{}", linenum).unwrap();
                          if bytes_read == 0 {
                              break;
                          }
                          linenum = linenum + 1;
                          if !line.starts_with("d") {
                              if line.starts_with("---- ") || line.starts_with("-ep- ") {
         			              let sizval = line.get(9..20).unwrap().to_string();
//                                  writeln!(&mut targetfile,"line num:{} size value: -{}-", linenum, sizval).unwrap();
         				          let test_int: i64 = sizval.trim().parse().unwrap_or(-99);
                                  let ssize;
         				          if test_int >= 0 {
         					          ssize = format!("{}",test_int);
         			              } else {
         					          ssize = format!("invalid size value: -{}-", sizval.trim());
         				          }
                                  let sdate = line.get(21..31).unwrap().to_string();
                                  let stime = line.get(32..40).unwrap().to_string();
                                  let sfile = line.get(41..(bytes_read-1)).unwrap().to_string();
                                  let stroutput;
                                  if sdir == "" {
                                      stroutput = format!("{},{},{} {}", sfile, ssize, sdate, stime);
                                  } else {
                                      stroutput = format!("{}/{},{},{} {}", sdir, sfile, ssize, sdate, stime);
                                  }
                                  writeln!(&mut targetfile, "{}", stroutput).unwrap();
                              } else {
                                  if line.contains(":") {
//                                      println!("sdir:-{}- topdir:-{}- line: {} value: {}", sdir, topdir, linenum, line.get(0..(bytes_read-1)).unwrap());
                                      let lcurrpos = line.find(":").unwrap();
                                      if sdir == "--" {
                                          sdir = "".to_string();
                                          topdir = line.get(..lcurrpos).unwrap().to_string();
                                      } else {
                                          let lcurrtop = line.find(&topdir).unwrap();
                                          sdir = line.get((lcurrtop+topdir.len()+1)..lcurrpos).unwrap().to_string();
                                      }
//                                      writeln!(&mut targetfile,"line num:{} cursor pos: {} dir -{}-", linenum, lcurrpos, sdir).unwrap();
                                  } else {
//                                      writeln!(&mut targetfile,"line num:{} bytes_read: {} unused line: {}", linenum, bytes_read, line.get(0..).unwrap()).unwrap();
                                  }
                              }
                          }
                          let progressfr: f64 = linenum as f64 / numrows as f64;
                          progress_progressbar.set_fraction(progressfr);
                          while glib::MainContext::pending(&glib::MainContext::default()) {
                                glib::MainContext::iteration(&glib::MainContext::default(),true);
                          }
                          if linenum > numrows {
                               break;
                          }
                          line.clear();
                      }
                      Err(_err) => {
                          messageval_label.set_markup("<span color=\"#FF000000\">* error reading mega-ls file: do file i and iconv **********</span>");
                          bolok = false;   
                          break;
                      }
                   }
                }
                if bolok {
                    messageval_label.set_text("source file exists and read");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* source file does not exist **********</span>");
            }
                
          } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING mega-ls FILE IN COMBOBOX **********</span>");
          }
        }
    }));

//----------------- convert button end -----------------------------------

//-------------------- connects end
}

use std::time::Duration;

use gtk::*;
use gtk::builders::GridBuilder;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::pango::{AttrList, AttrFontDesc, FontDescription};
use gtk::prelude::*;
extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());
    application.connect_activate(build_ui);
    gtk::init();
    gtk::set_debug_flags(1);
    application.run();
}

fn build_ui(application: &gtk::Application) {

    // Init window and basic skeleton
    let glade_src = include_str!("glade.ui");
    let builder = Builder::from_string(glade_src);
    let window: ApplicationWindow = builder.object("window").expect("Coauldn't get window");
    window.fullscreen();
    window.set_application(Some(application));


    let scrolled_window: ScrolledWindow = builder.object("scrolled_window").expect("Couldn't get window");

    // clone some varibles because closures don't want to share >:(
    let builder_clone = builder.clone();
    let window1 = window.clone();
    let window2 = window.clone();
    let scrolled_window1 = scrolled_window.clone();
    let application_clone = application.clone();

    // txt2img update
    let refresh_txt2img_images = move || {

        let grid: Grid = builder_clone.object("gride").expect("Couldn't get grid");
        // some varibles to properly count gui offsets
        let mut i = 0;
        let mut off = 0;

        // loop over every file in "./images/"
        for e in WalkDir::new("./images/").into_iter().filter_map(|e| e.ok()) {
            // ignore folders
            if e.metadata().unwrap().is_file() {
                // only if file is .jpg
                if e.path().extension().unwrap().to_str()  == Some("jpg") {
                    i = i + 1;

                    let image: Image = Image::new();

                    // Make a new label from prompt of the image
                    let label: Label = Label::new(Some(&e.path().to_str().unwrap().split("/").nth(2).unwrap().replace(",","_").replace("__","_").replace("_"," ")));
                    // load image into pixbuf and downscale to 360x360
                    let pixbuf1 = Pixbuf::from_file_at_scale(
                        e.path(),
                        360,
                        360,
                        false).unwrap();
                
                    // set image to pixbuf
                    image.set_pixbuf(Some(&pixbuf1));

                    // set some attributes
                    image.set_size_request(384, 360);
                    // Śmieszny sposob na ustawienie czcionki
                    let  attributes = AttrList::new();
                    attributes.insert(AttrFontDesc::new(&FontDescription::from_string("Minecraft Regular 24")));
                    label.set_attributes(Some(&attributes));
                    
                    label.set_size_request(360, 128);
                    label.set_wrap(true);
                    label.set_line_wrap(true);
                    label.set_width_chars(1);
                    label.set_max_width_chars(5);
                    label.set_margin_bottom(12);
                    label.set_wrap_mode(gtk::pango::WrapMode::Char);
                    // nawet nie pytaj
                    if i % 5 == 0{
                        //grid.add(&image);
                        grid.attach(&image, (i-1)*384, (360*off)+24*off, 384, 360);
                        grid.attach(&label, (i-1)*384, 360*off+360+24*off, 360, label.allocated_height());


                        //grid.attach_next_to(&label, Some(&image), PositionType::Bottom,1,1);
                        off = off + 1;
                        i = 0;
                    }else{
                        //grid.add(&image);
                        grid.attach(&image, (i-1)*384, (360*off)+24*off, 384, 360);
                        grid.attach(&label, (i-1)*384, 360*off+360+24*off, 360, label.allocated_height());


                        //grid.attach(&fixed,384*(i-1), 360*off+360+24*off, 384, 24);
                        //grid.attach_next_to(&label, Some(&image), PositionType::Bottom,320,24);
                    }
                    
                }
            }
        }

        // update window
        window1.show_all();

        // allow clock to continue
        glib::Continue(true)
    };

    let refresh_img2img_images = move || {

        // Img2Img Gallery
        let grid: Grid = builder.object("gridd").expect("Couldn't get grid");

        // some varibles to properly count gui offsets
        let mut i = 0;
        let mut off = 0;

        // loop over every file in "./images-img2img/outputs/"
        for e in WalkDir::new("./images-img2img/outputs/").into_iter().filter_map(|e| e.ok()) {
            // ignore folders
            if e.metadata().unwrap().is_file() {
                // only if file is .jpg
                if e.path().extension().unwrap().to_str()  == Some("jpg") {
                    i = i + 1;
                    
                    // this time init 2 images
                    let image: Image = Image::new();
                    let image2: Image = Image::new();
                    
                    // filter out prompt from file path
                    let prompt: &str =  e.path().to_str().unwrap().split("/").nth(3).unwrap();
                    
                    // set label to that prompt
                    let label: Label = Label::new(Some(&prompt.replace(",","_").replace("__","_").replace("_"," ")));
                    
                    // load image into pixbuf and downscale to 360x360
                    let pixbuf1 = Pixbuf::from_file_at_scale(
                        e.path(),
                        360,
                        360,
                        false).unwrap();
                    
                    // figure out second image path
                    let second_image_path = String::from("./images-img2img/inputs/") + prompt + &String::from("/") + e.path().file_name().unwrap().to_str().unwrap();
                    
                    // and set pixbuf to that path
                    let pixbuf2 = Pixbuf::from_file_at_scale(
                        second_image_path,
                            360,
                            360,
                            false).unwrap();

                    // set images to pixbufs
                    image.set_pixbuf(Some(&pixbuf1));
                    image2.set_pixbuf(Some(&pixbuf2));

                    // setup some look and feel attributes
                    image.set_margin_start(12);
                    image.set_margin_end(148);
                    label.set_margin_bottom(48);
                    label.set_size_request(360, 24);
                    label.set_line_wrap(true);
                    label.set_wrap_mode(gtk::pango::WrapMode::Word);

                    let attributes = AttrList::new();
                    
                    attributes.insert(AttrFontDesc::new(&FontDescription::from_string("Minecraft Regular 24")));
                    
                    label.set_attributes(Some(&attributes));

                    // nawet nie pytaj
                    if i % 2 == 0{

                        grid.attach(&image2, 1200, 360*off+24*off, 360, 360);

                        // One arrow can't be in 2 places at once
                        let arrow: Label = Label::new(Some("=>"));
                        arrow.set_margin_start(16);
                        let  attributes = AttrList::new();
                        attributes.insert(AttrFontDesc::new(&FontDescription::from_string("Monocraft Bold 36")));
                        arrow.set_attributes(Some(&attributes));
                        
                        // continue creating the grid layout
                        grid.attach_next_to(&arrow,Some(&image2),PositionType::Right,64,360);
                        grid.attach_next_to(&image,Some(&arrow),PositionType::Right,360,360);
                        grid.attach_next_to(&label, Some(&image2), PositionType::Bottom,752,24);
                        
                        off = off + 1;
                        i = 0;
                    }else{

                        image2.set_margin_start(73);
                        grid.attach(&image2, 0, 360*off+24*off, 360, 360);
                        
                        // same story as upper arrow
                        let arrow:Label = Label::new(Some("=>"));
                        arrow.set_margin_start(16);
                        let  attributes = AttrList::new();
                        attributes.insert(AttrFontDesc::new(&FontDescription::from_string("Monocraft Bold 36")));
                        arrow.set_attributes(Some(&attributes));
                        
                        // continue creating the grid layout
                        grid.attach_next_to(&arrow,Some(&image2),PositionType::Right,64,360);
                        grid.attach_next_to(&image,Some(&arrow),PositionType::Right,360,360);
                        grid.attach_next_to(&label, Some(&image2), PositionType::Bottom,752,24);

                    }
                }
            }
        }

        // update window
        window2.show_all();

        // allow clock to continue
        glib::Continue(true)
    };



    window.show_all();

    // to detect if we stopped moving
    let mut last_pos: f64 = 0.0;

    // go in reverse?
    let mut tylem: bool = false;

    let auto_scroll_tick = move || {
        let scroll: bool = scrolled_window1.hscrollbar_policy() == PolicyType::External;
        if scroll {
            let adj = scrolled_window1.clone().vadjustment();
            last_pos = adj.value();
            
            if tylem{

                adj.set_value(adj.value() - 1.0);

            }else{

                adj.set_value(adj.value() + 1.0 );

            }

            scrolled_window1.clone().set_vadjustment(Some(&adj));

            if last_pos == adj.value() {
                tylem = !tylem
            }
        }
        glib::Continue(true)
    };

    // executes the closure once every 16 miliseconds
    glib::timeout_add_local(Duration::from_millis(16), auto_scroll_tick);

    // init images for the first time (no need to wait 10 seconds)
    refresh_txt2img_images();
    refresh_img2img_images();

    // executes the closure once every 10 seconds
    glib::timeout_add_local(Duration::from_secs_f64(10.0), refresh_txt2img_images);
    glib::timeout_add_local(Duration::from_secs_f64(10.0), refresh_img2img_images);
    
    // Handle keypresses
    scrolled_window.clone().connect_key_press_event(move |_, k| {
        let gstring = k.clone().keyval().name().unwrap();
        let val = gstring.as_str();
        match val {
            "q" => application_clone.quit(), // quit
            "space" => { // start/stop scroll

                // This is really stupid but it works
                if scrolled_window.hscrollbar_policy() == PolicyType::External {
                    
                    scrolled_window.set_hscrollbar_policy(PolicyType::Always);
               
                }else{
                 
                    scrolled_window.set_hscrollbar_policy(PolicyType::External);
                
                }
                
            },
            _ => {}
        }
        Inhibit(false)
    });

}
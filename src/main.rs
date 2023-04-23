use std::io::Read;
use std::net::TcpListener;
use std::thread;

use gtk::gdk_pixbuf::{Pixbuf, InterpType};
use gtk::gio::MemoryInputStream;
use gtk::glib::{MainContext, PRIORITY_DEFAULT};
use gtk::pango::{AttrList, AttrFontDesc, FontDescription};
use gtk::prelude::*;
use gtk::*;

use serde::{Deserialize, Serialize};
extern crate walkdir;

extern crate base64;

fn main() {
    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());
    application.connect_activate(build_ui);
    gtk::init();
    gtk::set_debug_flags(1);
    application.run();
}

#[derive(Serialize, Deserialize, Debug)]
struct Rating {
    score: String,
    image: String,
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
    let builder_clone2 = builder.clone();
    let window1 = window.clone();
    let application_clone = application.clone();
    
    let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:2137").unwrap();

        for stream in listener.incoming() {
            
            let mut stream = stream.unwrap();
            let mut messages = String::new();
            stream.read_to_string(&mut messages);

            let ratings: Vec<Rating> = serde_json::from_str(&messages).unwrap();

            tx.send(ratings);
        }

    });

    let (tx2, rx2) = MainContext::channel(PRIORITY_DEFAULT);
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:2138").unwrap();

        for stream in listener.incoming() {
            
            let mut stream = stream.unwrap();
            let mut currentWebCamImage = String::new();
            stream.read_to_string(&mut currentWebCamImage);

            tx2.send(currentWebCamImage);
        }

    });

    window.show_all();
    
    rx2.attach(None, move |images| {
        let webCam: Image = builder_clone2.object("currentWebCamImg").expect("Couldn't get currentWebCamImg");
        let decoded = base64::decode(&images).unwrap();
        let bytes = glib::Bytes::from(&decoded);
        let stream = MemoryInputStream::from_bytes(&bytes);
        let mut pixbuf = Pixbuf::from_stream(&stream, None::<&gio::Cancellable>).unwrap();
        pixbuf = pixbuf.scale_simple(480, 480, InterpType::Bilinear).unwrap();
        webCam.set_pixbuf(Some(&pixbuf));
        Continue(true)
    });

    rx.attach(None, move |images| {
        let grid: Grid = builder_clone.object("gride").expect("Couldn't get grid");
        let children: Vec<Widget> = grid.children();
        for child in children {
            grid.remove(&child);
        }
        // some varibles to properly count gui offsets
        let mut i = 0;

        for data in images.iter() {
            i += 1;
            let image: Image = Image::new();
            let label: Label = Label::new(Some(&data.score));


            let decoded = base64::decode(&data.image).unwrap();
            let bytes = glib::Bytes::from(&decoded);
            let stream = MemoryInputStream::from_bytes(&bytes);
            let mut pixbuf = Pixbuf::from_stream(&stream, None::<&gio::Cancellable>).unwrap();
            pixbuf = pixbuf.scale_simple(360, 360, InterpType::Bilinear).unwrap();
            image.set_pixbuf(Some(&pixbuf));
            
            let attributes = AttrList::new();
            attributes.insert(AttrFontDesc::new(&FontDescription::from_string("Minecraft Regular 24")));

            label.set_attributes(Some(&attributes));
            label.set_size_request(360, 24);
            label.set_wrap(true);
            label.set_line_wrap(true);
            label.set_width_chars(1);
            label.set_max_width_chars(5);
            label.set_margin_bottom(12);
            label.set_wrap_mode(gtk::pango::WrapMode::Char);
            label.set_text(&format!("Score: {}", &data.score));

            grid.attach(&label, 360*i, 0, 360, 24);
            grid.attach(&image, 360*i, 24, 360, 360);
        }
        window1.show_all();
        Continue(true)
    });

    // Handle keypresses
    scrolled_window.clone().connect_key_press_event(move |_, k| {
        let gstring = k.clone().keyval().name().unwrap();
        let val = gstring.as_str();
        match val {
            "q" => application_clone.quit(), // quit
            _ => {}
        }
        Inhibit(false)
    });
    


}
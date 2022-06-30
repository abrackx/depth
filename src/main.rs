use gst::prelude::*;

fn tutorial_main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Build the pipeline
    let uri ="https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gst::parse_launch(&format!("playbin uri={}", uri)).unwrap();
    let x = gst::DeviceMonitor::new();
    let b = x.bus();
    x.start().expect("ok");

    
    for d in x.devices() {
        println!("{} : {}\n --- \n{:?}\n --- \n", d.display_name(), d.name(), d.properties());
    }

    for m in b.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match m.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => { 
                println!("{:?}", m);
                break;
            },
        }
    }
    

    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }
    x.stop();

    // Shutdown pipeline
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    tutorial_main();
}

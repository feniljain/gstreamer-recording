use gst::prelude::ElementExt;
use gst::prelude::GstObjectExt;

pub struct WebrtcSrc {
    peer_id: String,
}

impl WebrtcSrc {
    pub fn new(peer_id: String) -> Self {
        Self { peer_id }
    }

    pub fn launch(&self) {
        gst::init().expect(&format!(
            "could not start webrtsrc for peer id: {}",
            self.peer_id
        ));

        let uri = format!("gstwebrtc://127.0.0.1:8443?peer-id={}", self.peer_id);
        println!("uri: {uri}");
        let pipeline = gst::parse_launch(&format!("playbin uri={uri}")).expect(&format!(
            "could not launch pipeline for peer id: {}",
            self.peer_id
        ));

        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");

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
    }

    // fn stop() -> {
    //     pipeline
    //         .set_state(gst::State::Null)
    //         .expect("Unable to set the pipeline to the `Null` state");
    // }
}

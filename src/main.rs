use anyhow::{anyhow, Context};
use clap::Parser;
use gstreamer as gst;
use gstreamer::prelude::{Cast, ElementExt, GstBinExt, GstObjectExt, ObjectExt};
use gstreamer::{Bin, ClockTime};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    gst::init()?;

    let pipeline = gst::parse::launch(
        r#"
        filesrc name="filesrc"
        filesink name="filesink"
        decodebin name="decodebin"
        oggmux name="oggmux"

        filesrc.
        ! decodebin.

        decodebin.
        ! videoconvert
        ! video/x-raw,format=GRAY8
        ! videoconvert
        ! theoraenc
        ! oggmux.

        decodebin.
        ! vorbisenc
        ! oggmux.

        oggmux.
        ! filesink.
    "#,
    )?;

    let pipeline_as_bin = pipeline
        .clone()
        .downcast::<Bin>()
        .map_err(|_| anyhow!("could not downcast pipeline to bin"))?;

    pipeline_as_bin
        .by_name("filesrc")
        .context("could not find filesrc")?
        .set_property("location", args.input.as_str());

    pipeline_as_bin
        .by_name("filesink")
        .context("could not find filesink")?
        .set_property("location", args.output.as_str());

    pipeline.set_state(gst::State::Playing).context(format!(
        "could not set the pipeline to the `Playing` state\ndoes the input file {} exist?",
        args.input.as_str()
    ))?;

    let bus = pipeline.bus().context("could not get the bus")?;
    for msg in bus.iter_timed(ClockTime::NONE) {
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

    pipeline
        .set_state(gst::State::Null)
        .context("could not set the pipeline to the `Null` state")?;

    Ok(())
}

use std::sync::{Arc, Mutex};

use clap::Parser;
use fs_extra::{copy_items_with_progress, dir, TransitProcess};
use fs_extra::dir::TransitProcessResult;
use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::Args;

mod cli;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let options = dir::CopyOptions::new().overwrite(args.force);

    let bar = Arc::new(Mutex::new(None::<ProgressBar>));

    let handle = |process_info: TransitProcess| {
        let mut bar_lock = bar.lock().unwrap();

        match &*bar_lock {
            Some(progress_bar) => {
                progress_bar.set_position(process_info.copied_bytes);
            }
            None => {
                *bar_lock = {
                    let new_bar = ProgressBar::new(process_info.total_bytes);
                    new_bar.set_style(
                        ProgressStyle::with_template(
                            "[{elapsed_precise} | {eta_precise}] {wide_bar} {decimal_bytes}/{decimal_total_bytes}",
                        )
                        .unwrap()
                        .progress_chars("#$-"),
                    );
                    Some(new_bar)
                }
            }
        }

        TransitProcessResult::ContinueOrAbort
    };

    copy_items_with_progress(&[args.input], &args.output_dir, &options, handle)?;

    let bar_lock = bar.lock().unwrap();

    if let Some(bar) = &*bar_lock {
        bar.finish();
    }

    Ok(())
}

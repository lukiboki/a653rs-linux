use a653rs::partition;
use a653rs::prelude::PartitionExt;
use a653rs_linux::partition::ApexLogger;
use log::LevelFilter;
mod beispiel_bojan;
use beispiel_bojan::src::main;
use main::calc_adv;

fn main() {
    ApexLogger::install_panic_hook();
    ApexLogger::install_logger(LevelFilter::Trace).unwrap();

    hello::Partition.run()
}

#[partition(a653rs_linux::partition::ApexLinuxPartition)]
mod hello {
    use core::time::Duration;
    use std::thread::sleep;
    use crate::calc_adv;
    
    /*
    use a653rs_postcard::prelude::*;
    use humantime::format_duration;
    */
    use log::*;
    use serde::{Deserialize, Serialize};

    #[sampling_out(name = "Hello", msg_size = "10KB")]
    struct HelloSource;

    #[sampling_in(name = "Hello", msg_size = "10KB", refresh_period = "100ms")]
    struct HelloDestination;

    #[start(cold)]
    fn cold_start(mut ctx: start::Context) {
        let ident = ctx.get_partition_status().identifier;
        if ident == 0 {
            ctx.create_hello_source().unwrap();
        } else if ident == 1 {
            ctx.create_hello_destination().unwrap();
        }

        ctx.create_aperiodic_process().unwrap().start().unwrap();
        ctx.create_periodic_process().unwrap().start().unwrap();
    }

    #[start(warm)]
    fn warm_start(ctx: start::Context) {
        cold_start(ctx)
    }

    #[aperiodic(
        time_capacity = "Infinite",
        stack_size = "100KB",
        base_priority = 1,
        deadline = "Soft"
    )]
    fn aperiodic_process(_ctx: aperiodic_process::Context) {
        loop {
            info!("Start Aperiodic");
            sleep(Duration::from_millis(3));
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CustomMessage {
        msg: String,
        when: Duration,
    }

    #[periodic(
        period = "0ms",
        time_capacity = "Infinite",
        stack_size = "100KB",
        base_priority = 1,
        deadline = "Soft"
    )]
    fn periodic_process(ctx: periodic_process::Context) {
        loop {
            info!("Start Periodic");
            info!("{:?}", calc_adv());
            let _ = ctx.periodic_wait();
        }
    }
}

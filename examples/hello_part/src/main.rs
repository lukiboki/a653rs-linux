use a653rs::partition;
use a653rs::prelude::PartitionExt;
use a653rs_linux::partition::ApexLogger;
use log::LevelFilter;
mod beispiel_bojan;
use beispiel_bojan::src::main;

fn main() {
    ApexLogger::install_panic_hook();
    ApexLogger::install_logger(LevelFilter::Trace).unwrap();

    hello::Partition.run()
}

#[partition(a653rs_linux::partition::ApexLinuxPartition)]
mod hello {
    use core::time::Duration;
    use std::thread::sleep;

    /*
    use a653rs_postcard::prelude::*;
    */
    use humantime::format_duration;
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
            sleep(Duration::from_millis(1));
        }
        /* 
        for i in 0..i32::MAX {
            if let SystemTime::Normal(time) = ctx.get_time() {
                let round = Duration::from_millis(time.as_millis() as u64);
                info!("{:?}: AP MSG {i}", format_duration(round).to_string());
            }
            sleep(Duration::from_millis(1))
        }
        */
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
            ctx.periodic_wait();
        }
        /* 
        for i in 0..i32::MAX {
            if let SystemTime::Normal(time) = ctx.get_time() {
                let round = Duration::from_millis(time.as_millis() as u64);
                info!("{:?}: AP MSG {i}", format_duration(round).to_string());
            }
            sleep(Duration::from_millis(1))
        }
        */
        /*
        info!("Start Periodic");
        use crate::main;
        let calc_adv = main::calc_adv();
        calc_adv;
        println!("Test \n\n\n");
        */
    }
}

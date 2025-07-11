// --- job.rs ---

use std::time::Duration;
us
etokio::time;

pub async fn start_job_broadcast() {
    loop {
        println!("[Simulate] New mining.notify sent to clients");
        time::sleep(Duration::from_secs(15)).await;
    }
}

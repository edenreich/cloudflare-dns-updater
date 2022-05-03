use std::{env, sync::Arc};
use tokio::time::Duration;
use anyhow::Result;
use futures::StreamExt;
use kube::{
    api::{Api, ListParams},
    runtime::controller::{Action, Context, Controller},
    Client,
};
use cloudflare::{
    v4::dns::{
        create_record,
        update_record,
        fetch_record
    },
    typings::{
        DNSRecord,
        Error,
    },
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct ReconcilerContext {
    client: Client,
}

async fn reconcile(dns: Arc<DNSRecord>, _ctx: Context<ReconcilerContext>) -> Result<Action, Error> {
    // 1. Check if the DNSRecord was deleted, if it was deleted then delete it also from cloudflare and requeue

    // 2. Check if the DNSRecord is already present in cloudflare, if so, update it, otherwise create it
    let _cloudflare_dns_response = match fetch_record(&dns.spec).await {
        Some(dns_record) => update_record(&dns_record).await,
        None => create_record(&dns.spec).await,
    };

    Ok(Action::requeue(Duration::from_secs(300)))
}

fn error_policy(_error: &Error, _ctx: Context<ReconcilerContext>) -> Action {
    Action::requeue(Duration::from_secs(60))
}

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info,kube-runtime=debug,kube=debug");
    env_logger::init();
    let client = Client::try_default().await?;
    let context = Context::new(ReconcilerContext { client: client.clone() });
    let dns_records: Api<DNSRecord> = Api::<DNSRecord>::all(client.clone());
    Controller::new(dns_records, ListParams::default().timeout(10))
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("Reconciled {:?}", o),
                Err(e) => println!("Reconcile failed: {:?}", e),
            }
        })
        .await;
    return Ok(());
}

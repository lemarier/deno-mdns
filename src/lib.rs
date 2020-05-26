// Deno
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;
use mdns::{Record, RecordKind};
use serde::Deserialize;
use serde::Serialize;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::IpAddr;

thread_local! {
    static DEVICE_MAP: RefCell<HashMap<u32, *mut Any>> = RefCell::new(HashMap::new());
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeviceInfo {
    pub ip_addr: IpAddr,
}

#[derive(Serialize)]
struct MDNSResponse<T> {
    err: Option<String>,
    ok: Option<T>,
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("mdns_discover_all", op_mdns_discover_all);
}

#[derive(Serialize)]
struct MDNSResult {}

#[derive(Deserialize)]
struct DiscoveryParams {
    host: String,
    delay: i16,
}

#[derive(Serialize)]
struct DiscoverAllResult {
    woot: String,
}

fn op_mdns_discover_all(
    _interface: &mut dyn Interface,
    data: &[u8],
    _zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let mut response: MDNSResponse<DiscoverAllResult> = MDNSResponse {
        err: None,
        ok: None,
    };

    let params: DiscoveryParams = serde_json::from_slice(data).unwrap();

    let fut = async move {
        println!("host: {}, delay: {}", &params.host, &params.delay);

        std::thread::spawn(move || println!("Thread"));

        response.ok = Some(DiscoverAllResult {
            woot: "woo0t".to_string(),
        });

        serde_json::to_vec(&response).unwrap().into_boxed_slice()
    };

    Op::Async(fut.boxed())
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

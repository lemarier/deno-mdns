// Deno
use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;
use mdns_discover::{Record, RecordKind};
use serde::Deserialize;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::mpsc;

const SERVICE_NAME: &'static str = "_googlecast._tcp.local";

thread_local! {
    static DEVICE_MAP: RefCell<HashMap<IpAddr, bool>> = RefCell::new(HashMap::new());
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
    interface.register_op("mdns_discover_start", op_mdns_discover_start);
    interface.register_op("mdns_discover_get", op_mdns_discover_get);
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
    running: bool,
}

#[derive(Serialize)]
struct DiscoverGetResult {
    pending: Vec<DeviceInfo>,
}

fn op_mdns_discover_get(
    _interface: &mut dyn Interface,
    _data: &[u8],
    _zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let mut response: MDNSResponse<DiscoverGetResult> = MDNSResponse {
        err: None,
        ok: None,
    };
    println!("op_mdns_discover_get");

    let mut test_vec = Vec::with_capacity(2);
    DEVICE_MAP.with(|cell| {
        println!("inside device map");
        let instance_map = cell.borrow_mut();
        for (ip_address, is_submitted) in instance_map.iter() {
            println!("inside device map {}", ip_address);
            if !is_submitted {
                println!("announcing, {}", ip_address);
                test_vec.push(DeviceInfo {
                    ip_addr: *ip_address,
                });
                instance_map.get(ip_address).replace(&true);
            } else {
                println!("skipping announcing, {} already sent", ip_address);
            }
        }
    });

    response.ok = Some(DiscoverGetResult { pending: test_vec });

    let result: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result)
}

fn op_mdns_discover_start(
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

        std::thread::spawn(move || {
            for response in mdns_discover::discover::all(&params.host).unwrap() {
                match response {
                    Ok(response) => {
                        let addr = response.records().filter_map(self::to_ip_addr).next();
                        if let Some(addr) = addr {
                            DEVICE_MAP.with(|cell| {
                                let mut instance_map = cell.borrow_mut();
                                if !instance_map.contains_key(&addr) {
                                    println!("found new cast device at {}", addr);
                                    // we insert it but mark it as non-emited
                                    instance_map.insert(addr, false);
                                }
                            });
                        }
                    }
                    Err(e) => println!("error : {:?}", e),
                }
            }
        });

        response.ok = Some(DiscoverAllResult { running: true });
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

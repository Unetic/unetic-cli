use std::{env, path::Path};

use anyhow::{Context, Result};
use serde_json::Value;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map_or("status", String::as_str);

    match command {
        "status" => show_status(),
        "switch" => show_switch(),
        "wifi" => {
            let ssid = args.get(2).context("usage: unetic wifi <ssid>")?;
            set_wifi(ssid)
        }
        "json" => {
            let method = args.get(2).map_or("state", String::as_str);
            let payload = call_ubus(method, "{}")?;
            println!("{payload}");
            Ok(())
        }
        "-h" | "--help" | "help" => {
            print_help();
            Ok(())
        }
        other => {
            eprintln!("unknown command: {other}\n");
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("Unetic CLI — OpenWrt Control Plane");
    println!("Usage:");
    println!("  unetic status         Show current router status");
    println!("  unetic switch         Show Switch SoC and hardware capabilities");
    println!("  unetic wifi <ssid>    Change Wi-Fi SSID");
    println!("  unetic json <method>  Call raw ubus method");
}

fn call_ubus(method: &str, request_json: &str) -> Result<String> {
    let socket = Path::new("/var/run/ubus/ubus.sock");
    let mut connection = ubus::Connection::connect(socket)
        .context("failed to connect to /var/run/ubus/ubus.sock")?;

    let response = connection
        .call("unetic", method, request_json)
        .with_context(|| format!("ubus unetic.{method} call failed"))?;

    Ok(response)
}

fn show_status() -> Result<()> {
    let raw = call_ubus("state", "{}")?;
    let val: Value = serde_json::from_str(&raw)?;
    let state = val.get("state").unwrap_or(&val);

    let lifecycle = state
        .get("lifecycle")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let revision = state.get("revision").and_then(Value::as_u64).unwrap_or(0);
    let ssid = state
        .pointer("/wifi/ssid")
        .and_then(Value::as_str)
        .unwrap_or("-");
    let wan_proto = state
        .pointer("/wan/proto")
        .and_then(Value::as_str)
        .unwrap_or("-");
    let wan_status = state
        .pointer("/wan/status")
        .and_then(Value::as_str)
        .unwrap_or("-");
    let wan_ip = state
        .pointer("/wan/ip_address")
        .and_then(Value::as_str)
        .unwrap_or("-");

    println!("── Unetic Router Status ─────────────────────────");
    println!("Lifecycle : {lifecycle} (rev {revision})");
    println!("Wi-Fi SSID: {ssid}");
    println!("WAN       : {wan_proto} | {wan_status} | {wan_ip}");
    println!("─────────────────────────────────────────────────");

    Ok(())
}

fn show_switch() -> Result<()> {
    let raw = call_ubus("switch.get", "{}")?;
    let val: Value = serde_json::from_str(&raw)?;
    let res = val.get("result").unwrap_or(&val);

    let soc = res.get("soc").context("missing soc info in response")?;
    let vendor = soc
        .get("vendor")
        .and_then(Value::as_str)
        .unwrap_or("Unknown");
    let model = soc
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or("Unknown");
    let arch = soc
        .get("architecture")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let driver = soc.get("driver").and_then(Value::as_str).unwrap_or("none");
    let tagging = soc
        .get("tagging_protocol")
        .and_then(Value::as_str)
        .unwrap_or("none");
    let ports = soc
        .get("ports")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let ports_str = ports
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>()
        .join(", ");

    println!("── Switch Hardware & SoC ────────────────────────");
    println!("Model       : {vendor} {model} [{arch}]");
    println!("Driver      : {driver} (tagging: {tagging})");
    println!("Ports ({})   : {}", ports.len(), ports_str);
    println!("─────────────────────────────────────────────────");

    if let Some(feats) = res.get("features").and_then(Value::as_object) {
        println!("Hardware Capabilities:");
        for (name, obj) in feats {
            let supp = obj
                .get("supported")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let en = obj.get("enabled").and_then(Value::as_bool).unwrap_or(false);
            let status_badge = match (supp, en) {
                (true, true) => "[ACTIVE]",
                (true, false) => "[SUPPORTED]",
                (false, _) => "[UNSUPPORTED]",
            };
            println!("  {name:<28} {status_badge}");
        }
    }
    println!("─────────────────────────────────────────────────");

    Ok(())
}

fn set_wifi(ssid: &str) -> Result<()> {
    let raw_state = call_ubus("state", "{}")?;
    let val: Value = serde_json::from_str(&raw_state)?;
    let revision = val
        .pointer("/state/revision")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let req_id = format!("cli-{}", std::process::id());

    let payload = serde_json::json!({
        "ssid": ssid,
        "expected_revision": revision,
        "request_id": req_id,
    });

    println!("Applying Wi-Fi SSID: '{ssid}'...");
    let reply = call_ubus("wifi.set_ssid", &payload.to_string())?;
    println!("Response: {reply}");
    Ok(())
}

//! Operator and test CLI for sending synthetic events, toggling source failures,
//! running rounds and visit preparation, and resetting the staging practice.

#![deny(missing_docs)]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use physician_sample::synthetic_control::{
    ChartEventRecordKind, PatientSelector, PrepareVisitsRequest, RoundsRequest,
    SavedSyntheticEvent, SourceFailureRequest, SourceHealthDirective, SyntheticEventSpec,
    SyntheticSourceKind, VisitChangeStatus,
};
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug)]
struct CliInvocation {
    base_url: String,
    token: Option<String>,
    command: CliCommand,
}

#[derive(Debug)]
enum CliCommand {
    ListPatients,
    Status,
    SendEvent {
        spec: SyntheticEventSpec,
        save_to: Option<PathBuf>,
    },
    SourceFailure(SourceFailureRequest),
    Rounds(RoundsRequest),
    PrepareVisits(PrepareVisitsRequest),
    Reset,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        return ExitCode::SUCCESS;
    }
    let invocation = match parse_cli(&args) {
        Ok(inv) => inv,
        Err(err) => {
            eprintln!("error: {err}");
            print_usage();
            return ExitCode::from(1);
        }
    };
    match execute(invocation).await {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}

fn print_usage() {
    eprintln!(
        "Usage: coda-staging-events [--base-url <URL>] [--token <TOKEN>] <COMMAND> [OPTIONS]\n\
         \n\
         Environment:\n\
           CODA_STAGING_URL    Default staging base URL\n\
           CODA_STAGING_TOKEN  Default physician session / Bearer token\n\
         \n\
         Commands:\n\
           list\n\
           status\n\
           send-chart --patient <pw-...|MRN...|mrn:...> --kind <lab_result|clinical_note|diagnostic_report> \\\n\
                      --title <TITLE> --text <TEXT> --effective-at <ISO_DATE_OR_RFC3339> [--id <ID>] [--save-event <FILE>]\n\
           send-visit --patient <pw-...|MRN...|mrn:...> --appointment-id <ID> \\\n\
                      --status <booked|arrived|fulfilled|cancelled|no_show> \\\n\
                      --starts-at <RFC3339> --ends-at <RFC3339> --label <LABEL> [--save-event <FILE>]\n\
           send-raw   --file <SPEC_JSON_FILE> [--save-event <FILE>]\n\
           replay     --file <SAVED_EVENT_JSON_FILE> [--save-event <FILE>]\n\
           source-failure --source <chart|calendar> --mode <healthy|failing> [--reason <TEXT>] [--workspace <pw-...>]\n\
           rounds     [--now-unix <SECONDS>]\n\
           prepare-visits [--now-unix <SECONDS>]\n\
           reset"
    );
}

fn parse_cli(args: &[String]) -> Result<CliInvocation, String> {
    let mut base_url = env::var("CODA_STAGING_URL")
        .ok()
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty());
    let mut token = env::var("CODA_STAGING_TOKEN")
        .ok()
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty());

    let mut remaining: Vec<String> = Vec::new();
    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--base-url" => {
                idx += 1;
                let val = args
                    .get(idx)
                    .ok_or_else(|| "missing value for --base-url".to_owned())?;
                base_url = Some(val.trim().to_owned());
            }
            "--token" => {
                idx += 1;
                let val = args
                    .get(idx)
                    .ok_or_else(|| "missing value for --token".to_owned())?;
                token = Some(val.trim().to_owned());
            }
            other => remaining.push(other.to_owned()),
        }
        idx += 1;
    }

    let base_url = base_url
        .filter(|u| !u.is_empty())
        .ok_or_else(|| "supply --base-url <URL> or set CODA_STAGING_URL".to_owned())?
        .trim_end_matches('/')
        .to_owned();

    let Some((subcmd, rest)) = remaining.split_first() else {
        return Err("missing subcommand".to_owned());
    };

    let command = match subcmd.as_str() {
        "list" => {
            ensure_no_extra_args("list", rest)?;
            CliCommand::ListPatients
        }
        "status" => {
            ensure_no_extra_args("status", rest)?;
            CliCommand::Status
        }
        "reset" => {
            ensure_no_extra_args("reset", rest)?;
            CliCommand::Reset
        }
        "send-chart" => parse_send_chart(rest)?,
        "send-visit" => parse_send_visit(rest)?,
        "send-raw" => parse_send_raw(rest)?,
        "replay" => parse_replay(rest)?,
        "source-failure" => CliCommand::SourceFailure(parse_source_failure(rest)?),
        "rounds" => CliCommand::Rounds(RoundsRequest {
            now_unix: parse_optional_now_unix("rounds", rest)?,
        }),
        "prepare-visits" => CliCommand::PrepareVisits(PrepareVisitsRequest {
            now_unix: parse_optional_now_unix("prepare-visits", rest)?,
        }),
        unknown => return Err(format!("unknown subcommand `{unknown}`")),
    };

    Ok(CliInvocation {
        base_url,
        token,
        command,
    })
}

fn ensure_no_extra_args(cmd: &str, rest: &[String]) -> Result<(), String> {
    if let Some(unexpected) = rest.first() {
        return Err(format!("unexpected argument `{unexpected}` for `{cmd}`"));
    }
    Ok(())
}

fn parse_optional_now_unix(cmd: &str, rest: &[String]) -> Result<Option<i64>, String> {
    if rest.is_empty() {
        return Ok(None);
    }
    if rest.len() == 2 && rest[0] == "--now-unix" {
        let parsed = rest[1]
            .parse::<i64>()
            .map_err(|err| format!("invalid --now-unix `{}`: {err}", rest[1]))?;
        return Ok(Some(parsed));
    }
    Err(format!(
        "unexpected arguments for `{cmd}`: {}",
        rest.join(" ")
    ))
}

fn parse_patient_selector(raw: &str) -> Result<PatientSelector, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("patient selector must not be empty".to_owned());
    }
    if let Some(ws) = trimmed.strip_prefix("workspace:") {
        return Ok(PatientSelector::WorkspaceId(ws.trim().to_owned()));
    }
    if let Some(src) = trimmed.strip_prefix("source:") {
        return Ok(PatientSelector::SourcePatientId(src.trim().to_owned()));
    }
    if let Some(mrn) = trimmed.strip_prefix("mrn:") {
        return Ok(PatientSelector::MedicalRecordNumber(mrn.trim().to_owned()));
    }
    if trimmed.starts_with("pw-") {
        return Ok(PatientSelector::WorkspaceId(trimmed.to_owned()));
    }
    if trimmed.starts_with("MRN") {
        return Ok(PatientSelector::SourcePatientId(trimmed.to_owned()));
    }
    Ok(PatientSelector::MedicalRecordNumber(trimmed.to_owned()))
}

fn parse_send_chart(rest: &[String]) -> Result<CliCommand, String> {
    let mut patient = None;
    let mut record_id = None;
    let mut kind = None;
    let mut title = None;
    let mut text = None;
    let mut effective_at = None;
    let mut save_to = None;

    let mut idx = 0;
    while idx < rest.len() {
        let flag = rest[idx].as_str();
        idx += 1;
        let value = rest
            .get(idx)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag {
            "--patient" => patient = Some(parse_patient_selector(value)?),
            "--id" => record_id = Some(value.clone()),
            "--kind" => {
                kind = Some(match value.as_str() {
                    "lab_result" | "lab" => ChartEventRecordKind::LabResult,
                    "clinical_note" | "note" => ChartEventRecordKind::ClinicalNote,
                    "diagnostic_report" | "report" => ChartEventRecordKind::DiagnosticReport,
                    other => return Err(format!("unsupported --kind `{other}`")),
                });
            }
            "--title" => title = Some(value.clone()),
            "--text" => text = Some(value.clone()),
            "--effective-at" => effective_at = Some(value.clone()),
            "--save-event" => save_to = Some(PathBuf::from(value)),
            other => return Err(format!("unknown send-chart flag `{other}`")),
        }
        idx += 1;
    }

    Ok(CliCommand::SendEvent {
        spec: SyntheticEventSpec::FreeFormChartRecord {
            patient: patient.ok_or_else(|| "missing --patient".to_owned())?,
            record_id,
            kind: kind.ok_or_else(|| "missing --kind".to_owned())?,
            title: title.ok_or_else(|| "missing --title".to_owned())?,
            text: text.ok_or_else(|| "missing --text".to_owned())?,
            effective_at: effective_at.ok_or_else(|| "missing --effective-at".to_owned())?,
        },
        save_to,
    })
}

fn parse_send_visit(rest: &[String]) -> Result<CliCommand, String> {
    let mut patient = None;
    let mut appointment_id = None;
    let mut status = None;
    let mut starts_at = None;
    let mut ends_at = None;
    let mut label = None;
    let mut save_to = None;

    let mut idx = 0;
    while idx < rest.len() {
        let flag = rest[idx].as_str();
        idx += 1;
        let value = rest
            .get(idx)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag {
            "--patient" => patient = Some(parse_patient_selector(value)?),
            "--appointment-id" => appointment_id = Some(value.clone()),
            "--status" => {
                status = Some(match value.as_str() {
                    "booked" | "scheduled" => VisitChangeStatus::Booked,
                    "arrived" => VisitChangeStatus::Arrived,
                    "fulfilled" => VisitChangeStatus::Fulfilled,
                    "cancelled" => VisitChangeStatus::Cancelled,
                    "no_show" | "noshow" => VisitChangeStatus::NoShow,
                    other => return Err(format!("unsupported --status `{other}`")),
                });
            }
            "--starts-at" => starts_at = Some(value.clone()),
            "--ends-at" => ends_at = Some(value.clone()),
            "--label" => label = Some(value.clone()),
            "--save-event" => save_to = Some(PathBuf::from(value)),
            other => return Err(format!("unknown send-visit flag `{other}`")),
        }
        idx += 1;
    }

    Ok(CliCommand::SendEvent {
        spec: SyntheticEventSpec::VisitChange {
            patient: patient.ok_or_else(|| "missing --patient".to_owned())?,
            appointment_id: appointment_id.ok_or_else(|| "missing --appointment-id".to_owned())?,
            status: status.ok_or_else(|| "missing --status".to_owned())?,
            starts_at: starts_at.ok_or_else(|| "missing --starts-at".to_owned())?,
            ends_at: ends_at.ok_or_else(|| "missing --ends-at".to_owned())?,
            label: label.ok_or_else(|| "missing --label".to_owned())?,
        },
        save_to,
    })
}

fn parse_send_raw(rest: &[String]) -> Result<CliCommand, String> {
    let mut file = None;
    let mut save_to = None;
    let mut idx = 0;
    while idx < rest.len() {
        let flag = rest[idx].as_str();
        idx += 1;
        let value = rest
            .get(idx)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag {
            "--file" => file = Some(PathBuf::from(value)),
            "--save-event" => save_to = Some(PathBuf::from(value)),
            other => return Err(format!("unknown send-raw flag `{other}`")),
        }
        idx += 1;
    }
    let path = file.ok_or_else(|| "missing --file <SPEC_JSON_FILE>".to_owned())?;
    let raw =
        fs::read_to_string(&path).map_err(|err| format!("read `{}`: {err}", path.display()))?;
    let spec: SyntheticEventSpec = serde_json::from_str(&raw)
        .map_err(|err| format!("parse SyntheticEventSpec in `{}`: {err}", path.display()))?;
    Ok(CliCommand::SendEvent { spec, save_to })
}

fn parse_replay(rest: &[String]) -> Result<CliCommand, String> {
    let mut file = None;
    let mut save_to = None;
    let mut idx = 0;
    while idx < rest.len() {
        let flag = rest[idx].as_str();
        idx += 1;
        let value = rest
            .get(idx)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag {
            "--file" => file = Some(PathBuf::from(value)),
            "--save-event" => save_to = Some(PathBuf::from(value)),
            other => return Err(format!("unknown replay flag `{other}`")),
        }
        idx += 1;
    }
    let path = file.ok_or_else(|| "missing --file <SAVED_EVENT_JSON_FILE>".to_owned())?;
    let raw =
        fs::read_to_string(&path).map_err(|err| format!("read `{}`: {err}", path.display()))?;
    let spec = if let Ok(saved) = serde_json::from_str::<SavedSyntheticEvent>(&raw) {
        saved.event
    } else {
        serde_json::from_str::<SyntheticEventSpec>(&raw)
            .map_err(|err| format!("parse saved event in `{}`: {err}", path.display()))?
    };
    Ok(CliCommand::SendEvent { spec, save_to })
}

fn parse_source_failure(rest: &[String]) -> Result<SourceFailureRequest, String> {
    let mut source = None;
    let mut mode = None;
    let mut reason = None;
    let mut workspace = None;

    let mut idx = 0;
    while idx < rest.len() {
        let flag = rest[idx].as_str();
        idx += 1;
        let value = rest
            .get(idx)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag {
            "--source" => {
                source = Some(match value.as_str() {
                    "chart" => SyntheticSourceKind::Chart,
                    "calendar" => SyntheticSourceKind::Calendar,
                    other => return Err(format!("unsupported --source `{other}`")),
                });
            }
            "--mode" => mode = Some(value.clone()),
            "--reason" => reason = Some(value.clone()),
            "--workspace" => workspace = Some(value.clone()),
            other => return Err(format!("unknown source-failure flag `{other}`")),
        }
        idx += 1;
    }

    let source = source.ok_or_else(|| "missing --source <chart|calendar>".to_owned())?;
    let directive = match mode
        .as_deref()
        .ok_or_else(|| "missing --mode <healthy|failing>".to_owned())?
    {
        "healthy" => SourceHealthDirective::Healthy,
        "failing" => SourceHealthDirective::Failing {
            reason: reason.unwrap_or_else(|| "simulated synthetic source outage".to_owned()),
        },
        other => return Err(format!("unsupported --mode `{other}`")),
    };

    Ok(SourceFailureRequest {
        source,
        directive,
        patient_workspace_id: workspace,
    })
}

async fn execute(inv: CliInvocation) -> Result<ExitCode, String> {
    let client = reqwest::Client::builder()
        .build()
        .map_err(|err| format!("build HTTP client: {err}"))?;

    match inv.command {
        CliCommand::ListPatients => {
            send_request(
                &client,
                &inv.base_url,
                inv.token.as_deref(),
                Method::GET,
                "/staging-synthetic/patients",
                Option::<&()>::None,
                None,
            )
            .await
        }
        CliCommand::Status => {
            send_request(
                &client,
                &inv.base_url,
                inv.token.as_deref(),
                Method::GET,
                "/staging-synthetic/status",
                Option::<&()>::None,
                None,
            )
            .await
        }
        CliCommand::SendEvent { spec, save_to } => {
            send_request(
                &client,
                &inv.base_url,
                inv.token.as_deref(),
                Method::POST,
                "/staging-synthetic/events",
                Some(&spec),
                save_to.as_deref(),
            )
            .await
        }
        CliCommand::SourceFailure(payload) => {
            send_request(
                &client,
                &inv.base_url,
                inv.token.as_deref(),
                Method::POST,
                "/staging-synthetic/source-failure",
                Some(&payload),
                None,
            )
            .await
        }
        CliCommand::Rounds(payload) => {
            send_request(
                &client,
                &inv.base_url,
                inv.token.as_deref(),
                Method::POST,
                "/staging-synthetic/rounds",
                Some(&payload),
                None,
            )
            .await
        }
        CliCommand::PrepareVisits(payload) => {
            send_request(
                &client,
                &inv.base_url,
                inv.token.as_deref(),
                Method::POST,
                "/staging-synthetic/visits/prepare",
                Some(&payload),
                None,
            )
            .await
        }
        CliCommand::Reset => {
            send_request(
                &client,
                &inv.base_url,
                inv.token.as_deref(),
                Method::POST,
                "/staging-synthetic/reset",
                Option::<&()>::None,
                None,
            )
            .await
        }
    }
}

async fn send_request<B: Serialize + ?Sized>(
    client: &reqwest::Client,
    base_url: &str,
    token: Option<&str>,
    method: Method,
    path: &str,
    body: Option<&B>,
    save_to: Option<&std::path::Path>,
) -> Result<ExitCode, String> {
    let url = format!("{base_url}{path}");
    let mut req = client.request(method, &url);
    if let Some(tok) = token {
        req = req.bearer_auth(tok).header(
            reqwest::header::COOKIE,
            format!("__Host-coda_session={tok}; coda_session={tok}"),
        );
    }
    if let Some(payload) = body {
        req = req.json(payload);
    }

    let response = req
        .send()
        .await
        .map_err(|err| format!("HTTP request to {url} failed: {err}"))?;
    let status = response.status();
    let raw_text = response
        .text()
        .await
        .map_err(|err| format!("read response from {url}: {err}"))?;

    let formatted = match serde_json::from_str::<Value>(&raw_text) {
        Ok(val) => serde_json::to_string_pretty(&val).unwrap_or(raw_text),
        Err(_) => raw_text,
    };

    if status.is_success() {
        if let Some(dest) = save_to {
            if let Some(parent) = dest.parent().filter(|p| !p.as_os_str().is_empty()) {
                fs::create_dir_all(parent)
                    .map_err(|err| format!("create `{}`: {err}", parent.display()))?;
            }
            fs::write(dest, format!("{formatted}\n"))
                .map_err(|err| format!("write `{}`: {err}", dest.display()))?;
        }
        println!("{formatted}");
        Ok(ExitCode::SUCCESS)
    } else {
        eprintln!("HTTP {status} from {url}");
        println!("{formatted}");
        Ok(ExitCode::from(2))
    }
}

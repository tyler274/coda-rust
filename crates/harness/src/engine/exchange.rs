//! One provider dispatch followed by acknowledged evidence retention. The two
//! public facades supply different presentation policies, not different loops.

use super::*;
use crate::execution as legacy;

trait Exchange {
    type Response;
    type Error;

    async fn dispatch(&mut self) -> Result<Self::Response, Self::Error>;
    async fn retain(
        &mut self,
        result: Result<Self::Response, Self::Error>,
    ) -> Result<Self::Response, Self::Error>;
}

async fn drive<E: Exchange>(exchange: &mut E) -> Result<E::Response, E::Error> {
    let result = exchange.dispatch().await;
    // Evidence retention precedes interpretation/currentness/publication. Each
    // facade retains its existing failure and cancellation acknowledgment policy.
    exchange.retain(result).await
}

pub(super) struct Acknowledged<'a, 'cancel> {
    pub provider: &'a mut dyn Provider,
    pub host: &'a mut dyn Host,
    pub request: Request<'a>,
    pub sequence: &'a mut u64,
    pub gate: &'a mut Gate<'cancel>,
}

impl Exchange for Acknowledged<'_, '_> {
    type Response = Turn;
    type Error = Failure;

    async fn dispatch(&mut self) -> Result<Turn, Failure> {
        let mut sink = OwnerSink {
            host: self.host,
            attempt: self.request.attempt_id,
            sequence: self.sequence,
            ordinal: self.request.ordinal,
            cancel: self.gate.cancel.clone(),
            deadline: self.gate.deadline,
        };
        let request = Request {
            attempt_id: self.request.attempt_id,
            ordinal: self.request.ordinal,
            context: self.request.context,
            limits: self.request.limits,
        };
        self.gate
            .wait(
                "provider dispatch",
                self.provider.complete_stream(request, &mut sink),
            )
            .await
    }

    async fn retain(&mut self, result: Result<Turn, Failure>) -> Result<Turn, Failure> {
        let turn = result?;
        record(
            self.gate,
            self.host,
            self.request.attempt_id,
            self.sequence,
            EventKind::ProviderReturned(turn.clone()),
        )
        .await?;
        Ok(turn)
    }
}

pub(super) async fn acknowledged(exchange: &mut Acknowledged<'_, '_>) -> Result<Turn, Failure> {
    drive(exchange).await
}

const SETTLEMENT_EXPIRED: &str = "provider cancellation settlement deadline exceeded";

/// One cancellation window per phase: transport/callback/drain settlement,
/// followed by final evidence retention with its own single one-second grace.
/// Normal durable writes have no timeout. Dropping a callback at its deadline
/// leaves its write uncertain, never safe to retry.
#[derive(Default)]
struct Settlement {
    deadline: Option<Instant>,
}

impl Settlement {
    fn observe(&mut self, cancel: &legacy::Cancellation) {
        if cancel.is_cancelled() && self.deadline.is_none() {
            self.deadline = Some(Instant::now() + Duration::from_secs(1));
        }
    }

    async fn deadline(&mut self, cancel: &legacy::Cancellation) {
        // The compatibility cancellation token is atomic, not a notification.
        // Poll even while an owner future is pending so it cannot hide a stop.
        while self.deadline.is_none() {
            self.observe(cancel);
            if self.deadline.is_none() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
        tokio::time::sleep_until(self.deadline.expect("observed cancellation")).await;
    }

    async fn wait<T>(
        &mut self,
        cancel: &legacy::Cancellation,
        future: impl Future<Output = Result<T, String>>,
    ) -> Result<T, String> {
        self.observe(cancel);
        tokio::select! {
            biased;
            // Responsive evidence can still acknowledge at the boundary. A
            // pending write gets no additional time and remains uncertain.
            result = future => result,
            () = self.deadline(cancel) => Err(SETTLEMENT_EXPIRED.into()),
        }
    }
}

struct Observed<'a, P, O> {
    provider: &'a mut P,
    owner: &'a mut O,
    request: &'a legacy::Request,
    progress: &'a mut legacy::Progress,
    cancel: &'a legacy::Cancellation,
    emit: &'a mut (dyn FnMut(legacy::StreamEvent) -> Result<(), String> + Send),
    usage: legacy::Usage,
    identity: Value,
    evidence_failed: bool,
    settlement: Settlement,
}

impl<P: legacy::Provider, O: legacy::Owner> Exchange for Observed<'_, P, O> {
    type Response = legacy::Response;
    type Error = String;

    async fn dispatch(&mut self) -> Result<legacy::Response, String> {
        use legacy::{Record, StreamEvent};
        let sequence = self.progress.calls;
        // A synchronous transport cannot await the owner. Bound its bridge and
        // drain observations while the exchange is pending, including after a
        // presentation refusal. Transport settlement remains cooperative.
        let (observations, mut pending_observations) = tokio::sync::mpsc::channel(64);
        let mut deliver = |event: StreamEvent| {
            match &event {
                StreamEvent::Usage(usage) => {
                    self.usage.input_tokens = usage.input_tokens.or(self.usage.input_tokens);
                    self.usage.output_tokens = usage.output_tokens.or(self.usage.output_tokens);
                }
                StreamEvent::ProviderIdentity(identity) => self.identity = identity.clone(),
                StreamEvent::Text(_) => {}
            }
            let pending = match &event {
                StreamEvent::Text(text) => Err(text.clone()),
                _ => Ok(Record::Observation {
                    sequence,
                    usage: self.usage.clone(),
                    provenance: self.identity.clone(),
                }),
            };
            observations
                .try_send(pending)
                .map_err(|_| "provider observation backpressure".to_owned())?;
            self.cancel.check()?;
            (self.emit)(event)
        };
        let exchange = self
            .provider
            .exchange(self.request, self.cancel, &mut deliver);
        tokio::pin!(exchange);
        let mut delivery_error = None;
        let check_interval = Duration::from_millis(100);
        let mut currentness =
            tokio::time::interval_at(Instant::now() + check_interval, check_interval);
        let result = loop {
            tokio::select! {
                biased;
                () = self.settlement.deadline(self.cancel) => break Err(SETTLEMENT_EXPIRED.into()),
                Some(observation) = pending_observations.recv() => match observation {
                    Ok(record) => {
                        if let Err(error) = self.settlement.wait(self.cancel, self.owner.record(record)).await {
                            self.evidence_failed = true;
                            return Err(error);
                        }
                    },
                    Err(text) if delivery_error.is_none() && !self.cancel.is_cancelled() => {
                        if let Err(error) = self.settlement.wait(self.cancel, async {
                            self.cancel.check()?;
                            self.owner.provisional(sequence, &text).await
                        }).await {
                            delivery_error = Some(error);
                            self.cancel.cancel();
                        }
                    },
                    Err(_) => {},
                },
                result = &mut exchange => break result,
                _ = currentness.tick() => {
                    if self.settlement.wait(self.cancel, self.owner.check()).await.is_err() {
                        self.cancel.cancel();
                    }
                }
            }
        };
        while let Ok(observation) = pending_observations.try_recv() {
            match observation {
                Ok(record) => {
                    if let Err(error) = self
                        .settlement
                        .wait(self.cancel, self.owner.record(record))
                        .await
                    {
                        self.evidence_failed = true;
                        return Err(error);
                    }
                }
                Err(text) if delivery_error.is_none() && !self.cancel.is_cancelled() => {
                    if let Err(error) = self
                        .settlement
                        .wait(self.cancel, async {
                            self.cancel.check()?;
                            self.owner.provisional(sequence, &text).await
                        })
                        .await
                    {
                        delivery_error = Some(error);
                        self.cancel.cancel();
                    }
                }
                Err(_) => {}
            }
        }
        // A successful late exchange still carries usage; the caller fences
        // effects/publication only after retain acknowledges that evidence.
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(delivery_error.unwrap_or(error)),
        }
    }

    async fn retain(
        &mut self,
        result: Result<legacy::Response, String>,
    ) -> Result<legacy::Response, String> {
        use legacy::Record;
        // A failed durable acknowledgment stops immediately; it is not a
        // transport failure and must not cause another write to the failed owner.
        if self.evidence_failed {
            // Available evidence still counts even when its write was refused
            // or timed out. Do not retry the uncertain write to obtain an ack.
            account(self.progress, &self.usage);
            return result;
        }
        // Transport settlement may have consumed its entire cancellation
        // window. Give final evidence one bounded grace period, shared by all
        // records below, so a responsive async durable owner can actually ack.
        // This is a phase transition, never a per-record timeout reset.
        let mut retention = Settlement::default();
        retention.observe(self.cancel);
        let sequence = self.progress.calls;
        let response = match result {
            Ok(mut response) => {
                response.usage.input_tokens =
                    response.usage.input_tokens.or(self.usage.input_tokens);
                response.usage.output_tokens =
                    response.usage.output_tokens.or(self.usage.output_tokens);
                response
            }
            Err(reason) => {
                account(self.progress, &self.usage);
                retention
                    .wait(
                        self.cancel,
                        self.owner.record(Record::Observation {
                            sequence,
                            usage: self.usage.clone(),
                            provenance: self.identity.clone(),
                        }),
                    )
                    .await?;
                retention
                    .wait(
                        self.cancel,
                        self.owner.record(Record::Failure {
                            sequence,
                            reason: reason.clone(),
                        }),
                    )
                    .await?;
                return Err(reason);
            }
        };
        // Live counters retain available usage even if final acknowledgment
        // fails. Recovery still reconstructs durable truth from owner records.
        account(self.progress, &response.usage);
        retention
            .wait(
                self.cancel,
                self.owner.record(Record::Exchange {
                    sequence,
                    response: response.clone(),
                }),
            )
            .await?;
        Ok(response)
    }
}

fn account(progress: &mut legacy::Progress, usage: &legacy::Usage) {
    progress.known_tokens = progress
        .known_tokens
        .saturating_add(usage.input_tokens.unwrap_or(0))
        .saturating_add(usage.output_tokens.unwrap_or(0));
    if usage.input_tokens.is_none() || usage.output_tokens.is_none() {
        progress.unknown_usage = progress.unknown_usage.saturating_add(1);
    }
}

pub(crate) async fn observed<P: legacy::Provider, O: legacy::Owner>(
    provider: &mut P,
    owner: &mut O,
    request: &legacy::Request,
    progress: &mut legacy::Progress,
    cancel: &legacy::Cancellation,
    emit: &mut (dyn FnMut(legacy::StreamEvent) -> Result<(), String> + Send),
) -> Result<legacy::Response, String> {
    drive(&mut Observed {
        provider,
        owner,
        request,
        progress,
        cancel,
        emit,
        usage: legacy::Usage::default(),
        identity: Value::Null,
        evidence_failed: false,
        settlement: Settlement::default(),
    })
    .await
}

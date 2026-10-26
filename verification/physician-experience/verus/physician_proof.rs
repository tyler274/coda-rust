// Editable proof starter. Generated behavior stays in physician.rs.
// Strengthen inductive() when safety alone is not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "physician.rs"]
mod chart;
#[path = "physician_steps.rs"]
mod steps;
use chart::*;
use vstd::prelude::*;
verus! {
// Support facts. The chart's safety rules hold in every reachable state, but induction also needs
// these: each is true of every reachable state and is proved here with the rules. They state
// what the rules rely on, in the graph's own terms; none weakens a rule.
// A reply exists only for a message linked to this patient (or while a correction's Undo shows).
pub open spec fn fact_reply_needs_linked_message(s: State) -> bool {
    (s.region_patient_message_reply == RegionPatientMessageReply::StateReplyWorkOpen) ==> (s.region_patient_message_match == RegionPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_patient_message_match == RegionPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_patient_message_match == RegionPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// A concern about a message shows only while the message is linked, unless the re-review is still owed.
pub open spec fn fact_message_concern_needs_linked_message(s: State) -> bool {
    ((s.region_concern_needs_attention_visit_link == RegionConcernNeedsAttentionVisitLink::StateAboutAPatientMessageStaysWithItsMessage) && (s.region_what_relied_on_a_moved_message == RegionWhatReliedOnAMovedMessage::StateNothingWithheld)) ==> (s.region_patient_message_match == RegionPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_patient_message_match == RegionPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_patient_message_match == RegionPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// While the practice's whole records source is down, every patient's source is unavailable.
pub open spec fn fact_practice_down_means_patient_down(s: State) -> bool {
    (s.region_practice_records_source == RegionPracticeRecordsSource::StatePracticeSourceDownOneLineOnToday) ==> (s.region_records_source == RegionRecordsSource::StateSourceUnavailableLastGoodRecordsKept || s.region_records_source == RegionRecordsSource::StateSourceUnavailableNoRecordsYet)
}
// Coda revises a reply only while it is ready to edit or reviewing a new message.
pub open spec fn fact_revision_needs_editable_reply(s: State) -> bool {
    (s.region_coda_s_work_reply_revision_reply_revision_lifecycle == RegionCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionActiveReplyRevisionOwedWaitingToStart || s.region_coda_s_work_reply_revision_reply_revision_lifecycle == RegionCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionActiveCodaIsWorkingOnTheReplyRevision || s.region_coda_s_work_reply_revision_reply_revision_lifecycle == RegionCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionStalledReplyRevisionWaitingForTheNextOpportunityWithTryAgain || s.region_coda_s_work_reply_revision_reply_revision_lifecycle == RegionCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionStalledReplyRevisionCouldnTFinishWithTryAgain) ==> (s.region_patient_message_reply_reply_work_open_reply_status == RegionPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyReadyToSendOrCopy || s.region_patient_message_reply_reply_work_open_reply_status == RegionPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyCouldnTSendYourTextKept || s.region_patient_message_reply_reply_work_open_reply_status == RegionPatientMessageReplyReplyWorkOpenReplyStatus::StateReviewingANewMessage)
}
// A stopped revision exists only while its reply is ready to edit or reviewing a new message.
pub open spec fn fact_held_revision_needs_editable_reply(s: State) -> bool {
    (s.region_coda_s_work_reply_revision_reply_revision_held_by_you == RegionCodaSWorkReplyRevisionReplyRevisionHeldByYou::StateHeldWithResume) ==> (s.region_patient_message_reply_reply_work_open_reply_status == RegionPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyReadyToSendOrCopy || s.region_patient_message_reply_reply_work_open_reply_status == RegionPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyCouldnTSendYourTextKept || s.region_patient_message_reply_reply_work_open_reply_status == RegionPatientMessageReplyReplyWorkOpenReplyStatus::StateReviewingANewMessage)
}
// An asked reply is in hand only for a message linked to this patient.
pub open spec fn fact_ask_needs_linked_message(s: State) -> bool {
    (s.region_coda_s_work_asked_reply_asked_reply_lifecycle == RegionCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyActiveAskedReplyOwedWaitingToStart || s.region_coda_s_work_asked_reply_asked_reply_lifecycle == RegionCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyActiveCodaIsWorkingOnTheAskedReply || s.region_coda_s_work_asked_reply_asked_reply_lifecycle == RegionCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyStalledAskedReplyWaitingForTheNextOpportunityWithTryAgain || s.region_coda_s_work_asked_reply_asked_reply_lifecycle == RegionCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyStalledAskedReplyCouldnTFinishWithTryAgain) ==> (s.region_patient_message_match == RegionPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_patient_message_match == RegionPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_patient_message_match == RegionPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// A stopped asked reply exists only for a message linked to this patient.
pub open spec fn fact_held_ask_needs_linked_message(s: State) -> bool {
    (s.region_coda_s_work_asked_reply_asked_reply_held_by_you == RegionCodaSWorkAskedReplyAskedReplyHeldByYou::StateHeldWithResume) ==> (s.region_patient_message_match == RegionPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_patient_message_match == RegionPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_patient_message_match == RegionPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// An asked reply offering Update exists only for a message linked to this patient.
pub open spec fn fact_stale_ask_needs_linked_message(s: State) -> bool {
    (s.region_coda_s_work_asked_reply_asked_reply_lifecycle == RegionCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyNotInHandAskedReplyFinishedOnOlderRecordsWithUpdate) ==> (s.region_patient_message_match == RegionPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_patient_message_match == RegionPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_patient_message_match == RegionPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// A stopped visit preparation never belongs to a cancelled visit.
pub open spec fn fact_held_prep_needs_live_visit(s: State) -> bool {
    (s.region_coda_s_work_visit_preparation_visit_preparation_held_by_you == RegionCodaSWorkVisitPreparationVisitPreparationHeldByYou::StateHeldWithResume) ==> !(s.region_visit == RegionVisit::StateVisitCancelledCancelledItsTimeNotYetPassed || s.region_visit == RegionVisit::StateVisitCancelledCancelledItsTimeHasPassed)
}
// An archived conversation has no Steer or Send next in flight.
pub open spec fn fact_archived_has_no_steer_or_queue_in_flight(s: State) -> bool {
    (s.region_conversation_list == RegionConversationList::StateArchived) ==> !(s.region_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianSessionSignedInYourCommandOnTheConversation::StateSteerOnTheConversationSendingOtherCommandsOnItWait || s.region_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianSessionSignedInYourCommandOnTheConversation::StateCouldnTConfirmSteerOnTheConversationWithCheckAgain || s.region_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianSessionSignedInYourCommandOnTheConversation::StateSendNextOnTheConversationSendingOtherCommandsOnItWait || s.region_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianSessionSignedInYourCommandOnTheConversation::StateCouldnTConfirmSendNextOnTheConversationWithCheckAgain)
}
pub open spec fn inductive(s: State) -> bool {
    safety(s)
    && fact_reply_needs_linked_message(s)
    && fact_message_concern_needs_linked_message(s)
    && fact_practice_down_means_patient_down(s)
    && fact_revision_needs_editable_reply(s)
    && fact_held_revision_needs_editable_reply(s)
    && fact_ask_needs_linked_message(s)
    && fact_held_ask_needs_linked_message(s)
    && fact_stale_ask_needs_linked_message(s)
    && fact_held_prep_needs_live_visit(s)
    && fact_archived_has_no_steer_or_queue_in_flight(s)
}

pub proof fn initial_is_inductive(s: State)
    requires init(s), ensures inductive(s),
{}
pub proof fn step_preserves_inductive(s: State, t: State)
    requires inductive(s), step(s, t), ensures inductive(t),
{
    steps::every_event_preserves_inductive(s, t);
}
pub proof fn inductive_implies_safety(s: State)
    requires inductive(s), ensures safety(s),
{}

pub open spec fn path_from(s: State, rest: Seq<State>) -> bool
    decreases rest.len(),
{
    rest.len() == 0 || (step(s, rest.first()) && path_from(rest.first(), rest.drop_first()))
}
pub proof fn all_reached_states_are_safe(s: State, rest: Seq<State>)
    requires inductive(s), path_from(s, rest),
    ensures safety(s), forall|i: int| 0 <= i < rest.len() ==> safety(rest[i]),
    decreases rest.len(),
{
    // Each step is discharged by step_preserves_inductive; unfolding it here only costs time.
    hide(step);
    inductive_implies_safety(s);
    if rest.len() > 0 {
        step_preserves_inductive(s, rest.first());
        all_reached_states_are_safe(rest.first(), rest.drop_first());
        assert forall|i: int| 0 <= i < rest.len() implies safety(rest[i]) by {
            if i > 0 { assert(rest[i] == rest.drop_first()[i - 1]); }
        }
    }
}
pub proof fn every_initialized_finite_trace_is_safe(s: State, rest: Seq<State>)
    requires init(s), path_from(s, rest),
    ensures safety(s), forall|i: int| 0 <= i < rest.len() ==> safety(rest[i]),
{
    hide(step);
    initial_is_inductive(s);
    all_reached_states_are_safe(s, rest);
}
}
fn main() {}

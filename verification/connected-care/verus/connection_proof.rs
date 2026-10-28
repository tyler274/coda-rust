// Editable proof starter. Generated behavior stays in connection.rs.
// Strengthen inductive() when safety alone is not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "connection.rs"]
mod chart;
#[path = "connection_steps.rs"]
mod steps;
use chart::*;
use vstd::prelude::*;
verus! {
// Each graph's own support facts, proved again here with the shared events.
// Duet's:
// Duet holds its own message only while a reply is being written; the reply's end posts it.
pub open spec fn fact_duet_waiting_message_only_while_writing(s: State) -> bool {
    ((s.region_duet_your_account_set_up_duet_s_own_message == RegionDuetYourAccountSetUpDuetSOwnMessage::StateAMessageWaitingForTheReplyToEndOnlyDuetSOwnMessageWaiting)
        || (s.region_duet_your_account_set_up_duet_s_own_message == RegionDuetYourAccountSetUpDuetSOwnMessage::StateAMessageWaitingForTheReplyToEndWhatYouAskedForOrYourPracticeSNewsWaiting))
        ==> (s.region_duet_your_account_set_up_duet_s_reply == RegionDuetYourAccountSetUpDuetSReply::StateDuetIsWriting)
}
// The safety line is owed only within the reply being written; any end of that reply settles it.
pub open spec fn fact_duet_safety_line_owed_only_while_writing(s: State) -> bool {
    (s.region_duet_your_account_set_up_fixed_safety_line == RegionDuetYourAccountSetUpFixedSafetyLine::StateSafetyLineOwedInThisReply)
        ==> (s.region_duet_your_account_set_up_duet_s_reply == RegionDuetYourAccountSetUpDuetSReply::StateDuetIsWriting)
}
// Your doctor's plan is current, and your practice's appointment exists, only while you're connected to the practice.
pub open spec fn fact_duet_practice_items_only_while_connected(s: State) -> bool {
    ((s.region_duet_your_account_set_up_your_doctor_s_plan == RegionDuetYourAccountSetUpYourDoctorSPlan::StateYourDoctorSPlanCurrent)
        ==> (s.region_duet_your_practice == RegionDuetYourPractice::StateConnectedToYourPracticeSharing))
    && ((s.region_duet_your_account_set_up_your_practice_s_appointment == RegionDuetYourAccountSetUpYourPracticeSAppointment::StateAppointmentFromYourPractice)
        ==> (s.region_duet_your_practice == RegionDuetYourPractice::StateConnectedToYourPracticeSharing))
}
// An organization being signed in to, or connected, belongs to a live account: one that exists and isn't being deleted.
pub open spec fn fact_duet_organization_only_with_live_account(s: State) -> bool {
    !(s.region_duet_doctors_hospitals_an_organization == RegionDuetDoctorsHospitalsAnOrganization::StateOrganizationNotConnected)
        ==> !(s.region_duet_your_account == RegionDuetYourAccount::StateNoAccount || s.region_duet_your_account == RegionDuetYourAccount::StateDeletingYourAccount)
}
// Your phone, connected or with access turned off, belongs to a live account.
pub open spec fn fact_duet_phone_only_with_live_account(s: State) -> bool {
    !(s.region_duet_your_phone_s_health_data == RegionDuetYourPhoneSHealthData::StatePhoneNotConnected)
        ==> !(s.region_duet_your_account == RegionDuetYourAccount::StateNoAccount || s.region_duet_your_account == RegionDuetYourAccount::StateDeletingYourAccount)
}
// An item opens on Today only for a set-up account; it can stay open while that account is being
// deleted, because this device stays signed in until the deletion finishes.
pub open spec fn fact_duet_today_items_only_when_set_up(s: State) -> bool {
    !(s.region_duet_this_device_signed_in_on_this_device_open_on_today == RegionDuetThisDeviceSignedInOnThisDeviceOpenOnToday::StateNotInSignedInOnThisDevice
        || s.region_duet_this_device_signed_in_on_this_device_open_on_today == RegionDuetThisDeviceSignedInOnThisDeviceOpenOnToday::StateTodayItself)
        ==> (s.region_duet_your_account == RegionDuetYourAccount::StateSetUp || s.region_duet_your_account == RegionDuetYourAccount::StateDeletingYourAccount)
}
// The physician graph's:
// Support facts. The chart's safety rules hold in every reachable state, but induction also needs
// these: each is true of every reachable state and is proved here with the rules. They state
// what the rules rely on, in the graph's own terms; none weakens a rule.
// A reply exists only for a message linked to this patient (or while a correction's Undo shows).
pub open spec fn fact_physician_reply_needs_linked_message(s: State) -> bool {
    (s.region_physician_web_app_patient_message_reply == RegionPhysicianWebAppPatientMessageReply::StateReplyWorkOpen) ==> (s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// A concern about a message shows only while the message is linked, unless the re-review is still owed.
pub open spec fn fact_physician_message_concern_needs_linked_message(s: State) -> bool {
    ((s.region_physician_web_app_concern_needs_attention_visit_link == RegionPhysicianWebAppConcernNeedsAttentionVisitLink::StateAboutAPatientMessageStaysWithItsMessage) && (s.region_physician_web_app_what_relied_on_a_moved_message == RegionPhysicianWebAppWhatReliedOnAMovedMessage::StateNothingWithheld)) ==> (s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// While the practice's whole records source is down, every patient's source is unavailable.
pub open spec fn fact_physician_practice_down_means_patient_down(s: State) -> bool {
    (s.region_physician_web_app_practice_records_source == RegionPhysicianWebAppPracticeRecordsSource::StatePracticeSourceDownOneLineOnToday) ==> (s.region_physician_web_app_records_source == RegionPhysicianWebAppRecordsSource::StateSourceUnavailableLastGoodRecordsKept || s.region_physician_web_app_records_source == RegionPhysicianWebAppRecordsSource::StateSourceUnavailableNoRecordsYet)
}
// Coda revises a reply only while it is ready to edit or reviewing a new message.
pub open spec fn fact_physician_revision_needs_editable_reply(s: State) -> bool {
    (s.region_physician_web_app_coda_s_work_reply_revision_reply_revision_lifecycle == RegionPhysicianWebAppCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionActiveReplyRevisionOwedWaitingToStart || s.region_physician_web_app_coda_s_work_reply_revision_reply_revision_lifecycle == RegionPhysicianWebAppCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionActiveCodaIsWorkingOnTheReplyRevision || s.region_physician_web_app_coda_s_work_reply_revision_reply_revision_lifecycle == RegionPhysicianWebAppCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionStalledReplyRevisionWaitingForTheNextOpportunityWithTryAgain || s.region_physician_web_app_coda_s_work_reply_revision_reply_revision_lifecycle == RegionPhysicianWebAppCodaSWorkReplyRevisionReplyRevisionLifecycle::StateReplyRevisionInHandReplyRevisionStalledReplyRevisionCouldnTFinishWithTryAgain) ==> (s.region_physician_web_app_patient_message_reply_reply_work_open_reply_status == RegionPhysicianWebAppPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyReadyToSendOrCopy || s.region_physician_web_app_patient_message_reply_reply_work_open_reply_status == RegionPhysicianWebAppPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyCouldnTSendYourTextKept || s.region_physician_web_app_patient_message_reply_reply_work_open_reply_status == RegionPhysicianWebAppPatientMessageReplyReplyWorkOpenReplyStatus::StateReviewingANewMessage)
}
// A stopped revision exists only while its reply is ready to edit or reviewing a new message.
pub open spec fn fact_physician_held_revision_needs_editable_reply(s: State) -> bool {
    (s.region_physician_web_app_coda_s_work_reply_revision_reply_revision_held_by_you == RegionPhysicianWebAppCodaSWorkReplyRevisionReplyRevisionHeldByYou::StateHeldWithResume) ==> (s.region_physician_web_app_patient_message_reply_reply_work_open_reply_status == RegionPhysicianWebAppPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyReadyToSendOrCopy || s.region_physician_web_app_patient_message_reply_reply_work_open_reply_status == RegionPhysicianWebAppPatientMessageReplyReplyWorkOpenReplyStatus::StateReplyReadyCouldnTSendYourTextKept || s.region_physician_web_app_patient_message_reply_reply_work_open_reply_status == RegionPhysicianWebAppPatientMessageReplyReplyWorkOpenReplyStatus::StateReviewingANewMessage)
}
// An asked reply is in hand only for a message linked to this patient.
pub open spec fn fact_physician_ask_needs_linked_message(s: State) -> bool {
    (s.region_physician_web_app_coda_s_work_asked_reply_asked_reply_lifecycle == RegionPhysicianWebAppCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyActiveAskedReplyOwedWaitingToStart || s.region_physician_web_app_coda_s_work_asked_reply_asked_reply_lifecycle == RegionPhysicianWebAppCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyActiveCodaIsWorkingOnTheAskedReply || s.region_physician_web_app_coda_s_work_asked_reply_asked_reply_lifecycle == RegionPhysicianWebAppCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyStalledAskedReplyWaitingForTheNextOpportunityWithTryAgain || s.region_physician_web_app_coda_s_work_asked_reply_asked_reply_lifecycle == RegionPhysicianWebAppCodaSWorkAskedReplyAskedReplyLifecycle::StateAskedReplyInHandAskedReplyStalledAskedReplyCouldnTFinishWithTryAgain) ==> (s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// A stopped asked reply exists only for a message linked to this patient.
pub open spec fn fact_physician_held_ask_needs_linked_message(s: State) -> bool {
    (s.region_physician_web_app_coda_s_work_asked_reply_asked_reply_held_by_you == RegionPhysicianWebAppCodaSWorkAskedReplyAskedReplyHeldByYou::StateHeldWithResume) ==> (s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateNoCorrectionWaitingOnUndoLinkedToThePatient || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMovedToAnotherPatientUndoShown || s.region_physician_web_app_patient_message_match == RegionPhysicianWebAppPatientMessageMatch::StateMarkedNotAPatientUndoShown)
}
// A stopped visit preparation never belongs to a cancelled visit.
pub open spec fn fact_physician_held_prep_needs_live_visit(s: State) -> bool {
    (s.region_physician_web_app_coda_s_work_visit_preparation_visit_preparation_held_by_you == RegionPhysicianWebAppCodaSWorkVisitPreparationVisitPreparationHeldByYou::StateHeldWithResume) ==> !(s.region_physician_web_app_visit == RegionPhysicianWebAppVisit::StateVisitCancelledCancelledItsTimeNotYetPassed || s.region_physician_web_app_visit == RegionPhysicianWebAppVisit::StateVisitCancelledCancelledItsTimeHasPassed)
}
// An archived conversation has no Steer or Send next in flight.
pub open spec fn fact_physician_archived_has_no_steer_or_queue_in_flight(s: State) -> bool {
    (s.region_physician_web_app_conversation_list == RegionPhysicianWebAppConversationList::StateArchived) ==> !(s.region_physician_web_app_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianWebAppPhysicianSessionSignedInYourCommandOnTheConversation::StateSteerOnTheConversationSendingOtherCommandsOnItWait || s.region_physician_web_app_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianWebAppPhysicianSessionSignedInYourCommandOnTheConversation::StateCouldnTConfirmSteerOnTheConversationWithCheckAgain || s.region_physician_web_app_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianWebAppPhysicianSessionSignedInYourCommandOnTheConversation::StateSendNextOnTheConversationSendingOtherCommandsOnItWait || s.region_physician_web_app_physician_session_signed_in_your_command_on_the_conversation == RegionPhysicianWebAppPhysicianSessionSignedInYourCommandOnTheConversation::StateCouldnTConfirmSendNextOnTheConversationWithCheckAgain)
}
pub open spec fn inductive(s: State) -> bool {
    safety(s)
    && fact_duet_waiting_message_only_while_writing(s)
    && fact_duet_safety_line_owed_only_while_writing(s)
    && fact_duet_practice_items_only_while_connected(s)
    && fact_duet_organization_only_with_live_account(s)
    && fact_duet_phone_only_with_live_account(s)
    && fact_duet_today_items_only_when_set_up(s)
    && fact_physician_reply_needs_linked_message(s)
    && fact_physician_message_concern_needs_linked_message(s)
    && fact_physician_practice_down_means_patient_down(s)
    && fact_physician_revision_needs_editable_reply(s)
    && fact_physician_held_revision_needs_editable_reply(s)
    && fact_physician_ask_needs_linked_message(s)
    && fact_physician_held_ask_needs_linked_message(s)
    && fact_physician_held_prep_needs_live_visit(s)
    && fact_physician_archived_has_no_steer_or_queue_in_flight(s)
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

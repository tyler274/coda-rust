// Editable proof starter. Generated behavior stays in duet.rs.
// Strengthen inductive() when safety alone is not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "duet.rs"]
mod chart;
#[path = "duet_steps.rs"]
mod steps;
use chart::*;
use vstd::prelude::*;
verus! {
// Duet holds its own message only while a reply is being written; the reply's end posts it.
pub open spec fn fact_waiting_message_only_while_writing(s: State) -> bool {
    ((s.region_your_account_set_up_duet_s_own_message == RegionYourAccountSetUpDuetSOwnMessage::StateAMessageWaitingForTheReplyToEndOnlyDuetSOwnMessageWaiting)
        || (s.region_your_account_set_up_duet_s_own_message == RegionYourAccountSetUpDuetSOwnMessage::StateAMessageWaitingForTheReplyToEndWhatYouAskedForOrYourPracticeSNewsWaiting))
        ==> (s.region_your_account_set_up_duet_s_reply == RegionYourAccountSetUpDuetSReply::StateDuetIsWriting)
}
// The safety line is owed only within the reply being written; any end of that reply settles it.
pub open spec fn fact_safety_line_owed_only_while_writing(s: State) -> bool {
    (s.region_your_account_set_up_fixed_safety_line == RegionYourAccountSetUpFixedSafetyLine::StateSafetyLineOwedInThisReply)
        ==> (s.region_your_account_set_up_duet_s_reply == RegionYourAccountSetUpDuetSReply::StateDuetIsWriting)
}
// Your doctor's plan is current, and your practice's appointment exists, only while you're connected to the practice.
pub open spec fn fact_practice_items_only_while_connected(s: State) -> bool {
    ((s.region_your_account_set_up_your_doctor_s_plan == RegionYourAccountSetUpYourDoctorSPlan::StateYourDoctorSPlanCurrent)
        ==> (s.region_your_practice == RegionYourPractice::StateConnectedToYourPracticeSharing))
    && ((s.region_your_account_set_up_your_practice_s_appointment == RegionYourAccountSetUpYourPracticeSAppointment::StateAppointmentFromYourPractice)
        ==> (s.region_your_practice == RegionYourPractice::StateConnectedToYourPracticeSharing))
}
// An organization being signed in to, or connected, belongs to a live account: one that exists and isn't being deleted.
pub open spec fn fact_organization_only_with_live_account(s: State) -> bool {
    !(s.region_doctors_hospitals_an_organization == RegionDoctorsHospitalsAnOrganization::StateOrganizationNotConnected)
        ==> !(s.region_your_account == RegionYourAccount::StateNoAccount || s.region_your_account == RegionYourAccount::StateDeletingYourAccount)
}
// Your phone, connected or with access turned off, belongs to a live account.
pub open spec fn fact_phone_only_with_live_account(s: State) -> bool {
    !(s.region_your_phone_s_health_data == RegionYourPhoneSHealthData::StatePhoneNotConnected)
        ==> !(s.region_your_account == RegionYourAccount::StateNoAccount || s.region_your_account == RegionYourAccount::StateDeletingYourAccount)
}
// An item opens on Today only for a set-up account; it can stay open while that account is being
// deleted, because this device stays signed in until the deletion finishes.
pub open spec fn fact_today_items_only_when_set_up(s: State) -> bool {
    !(s.region_this_device_signed_in_on_this_device_open_on_today == RegionThisDeviceSignedInOnThisDeviceOpenOnToday::StateNotInSignedInOnThisDevice
        || s.region_this_device_signed_in_on_this_device_open_on_today == RegionThisDeviceSignedInOnThisDeviceOpenOnToday::StateTodayItself)
        ==> (s.region_your_account == RegionYourAccount::StateSetUp || s.region_your_account == RegionYourAccount::StateDeletingYourAccount)
}
pub open spec fn inductive(s: State) -> bool {
    safety(s)
    && fact_waiting_message_only_while_writing(s)
    && fact_safety_line_owed_only_while_writing(s)
    && fact_practice_items_only_while_connected(s)
    && fact_organization_only_with_live_account(s)
    && fact_phone_only_with_live_account(s)
    && fact_today_items_only_when_set_up(s)
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

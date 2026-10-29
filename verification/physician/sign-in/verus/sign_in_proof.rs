// Editable proof starter. Generated behavior stays in sign_in.rs.
// Strengthen inductive() when safety alone is not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "sign_in.rs"]
mod chart;
#[path = "sign_in_steps.rs"]
mod steps;
use chart::*;
use vstd::prelude::*;
verus! {
// The tab that is signing in has navigated away from any patient link it held, and is not
// yet signed in: a sign-in from another tab moves it on. A linked patient is only where
// the workspace opens while the tab shows that workspace or the prompt over it; every way
// out of those screens opens Today.
pub open spec fn inductive(s: State) -> bool {
    safety(s)
    && (s.region_this_tab_shows == RegionThisTabShows::StateSigningYouIn
        ==> s.region_a_patient_link_this_tab_holds == RegionAPatientLinkThisTabHolds::StateNoPatientLink
            && s.region_your_session != RegionYourSession::StateSignedIn)
    && (s.region_where_the_workspace_opens == RegionWhereTheWorkspaceOpens::StateTheLinkedPatient
        ==> s.region_this_tab_shows == RegionThisTabShows::StateYourWorkspace
            || s.region_this_tab_shows == RegionThisTabShows::StateSignInPromptOverYourWork)
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

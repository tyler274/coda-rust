// Editable proof. Generated behavior, rules and facts stay in sign_in.rs.
// Strengthen the model's facts when they are not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "sign_in.rs"]
mod chart;
#[path = "sign_in_steps.rs"]
mod steps;
use chart::*;
use vstd::prelude::*;
verus! {
// The model's facts (generated with it) and its rules hold at the start, survive every step, and
// so hold in every state the model can reach.
pub open spec fn inductive(s: State) -> bool {
    facts(s) && safety(s)
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

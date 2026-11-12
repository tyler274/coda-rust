//! A swarm is one append-only message board plus a roster of members and
//! channels. This crate owns their data model, the rules that admit a post,
//! and storage behind the [`Store`] trait with a locked file backend.

mod file_store;
pub mod mcp;
mod mention;
mod message;
mod names;
pub mod policy;
mod roster;
mod store;

pub use file_store::FileStore;
pub use mention::{Mention, parse_mentions};
pub use message::{Draft, Message, MessageKind, RequestId, Seq};
pub use names::{ChannelName, MemberName, NameError};
pub use policy::Policy;
pub use roster::{
    Channel, ChannelError, Member, MemberState, Role, Roster, RosterError, Topic, TopicError,
    WakeMode,
};
pub use store::{
    AppendError, Backend, Rejection, Store, StoreError, SwarmId, SwarmMeta, UpdateError,
};

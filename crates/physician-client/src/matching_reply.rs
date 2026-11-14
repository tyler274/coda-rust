//! Typed client calls for Spruce `Messages to match`, `Filtered`, unreadable
//! channel deliveries, and patient-scoped Spruce reply cards, private drafts,
//! revisions, copy attributions, and conversation moves (`Wrong patient?` / `Undo`).

use physician_wire::matching_reply::{
    ChooseCodaReplyUpdateBody, ConfirmMessageLinkBody, ConfirmMessageLinkResponseWire,
    CopyReplyBody, CopyReplyResponseWire, DisconnectedReplyDoneBody, ExpectedDraftBaseWire,
    FilteredMessagesResponseWire, MatchingActionCommandBody, MatchingMessageStatusResponseWire,
    MessagesToMatchResponseWire, MoveDestinationWire, MoveSpruceConversationBody,
    NoReplyNeededBody, PatientReplyConversationWire, PatientReplyListResponseWire,
    ReplyCommandKeyBody, RequestReplyProposalRevisionBody, RestoreMyReplyVersionBody,
    SaveReplyProposalDraftBody, SenderConfirmationPolicyWire, ThisIsAPatientOutcomeWire,
};

use crate::{HttpError, PhysicianClient, path_escape};

impl PhysicianClient {
    /// Reads `Messages to match` and unreadable Spruce channel deliveries (`GET /spruce-matching/messages`).
    pub async fn messages_to_match(&self) -> Result<MessagesToMatchResponseWire, HttpError> {
        self.get("/spruce-matching/messages").await
    }

    /// Reads classified non-patient mail in `Filtered` (`GET /spruce-matching/filtered`).
    pub async fn filtered_spruce_messages(
        &self,
    ) -> Result<FilteredMessagesResponseWire, HttpError> {
        self.get("/spruce-matching/filtered").await
    }

    /// Confirms a patient link (`Confirm` or `Choose another patient`) for a waiting message.
    pub async fn confirm_spruce_message_link(
        &self,
        message_id: &str,
        patient_workspace_id: &str,
        sender_policy: SenderConfirmationPolicyWire,
        idempotency_key: &str,
    ) -> Result<ConfirmMessageLinkResponseWire, HttpError> {
        let body = ConfirmMessageLinkBody {
            patient_workspace_id: patient_workspace_id.to_owned(),
            sender_policy,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/spruce-matching/messages/{}/confirm",
                path_escape(message_id)
            ),
            &body,
        )
        .await
    }

    /// Leaves a waiting message unlinked (`Leave unlinked`).
    pub async fn leave_spruce_message_unlinked(
        &self,
        message_id: &str,
        idempotency_key: &str,
    ) -> Result<MatchingMessageStatusResponseWire, HttpError> {
        let body = MatchingActionCommandBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/spruce-matching/messages/{}/leave-unlinked",
                path_escape(message_id)
            ),
            &body,
        )
        .await
    }

    /// Marks a filtered message `"This is a patient"`, returning it to routing.
    pub async fn mark_spruce_message_as_patient(
        &self,
        message_id: &str,
        idempotency_key: &str,
    ) -> Result<ThisIsAPatientOutcomeWire, HttpError> {
        let body = MatchingActionCommandBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/spruce-matching/filtered/{}/this-is-a-patient",
                path_escape(message_id)
            ),
            &body,
        )
        .await
    }

    /// Marks an unreadable Spruce delivery `Done`.
    pub async fn mark_unreadable_spruce_message_done(
        &self,
        message_id: &str,
        idempotency_key: &str,
    ) -> Result<MatchingMessageStatusResponseWire, HttpError> {
        let body = MatchingActionCommandBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/spruce-matching/unreadable/{}/done",
                path_escape(message_id)
            ),
            &body,
        )
        .await
    }

    /// Lists all linked Spruce reply conversations on an authorized patient workspace.
    pub async fn patient_spruce_replies(
        &self,
        patient_workspace_id: &str,
    ) -> Result<PatientReplyListResponseWire, HttpError> {
        self.get(&format!(
            "/patient-workspaces/{}/spruce-replies",
            path_escape(patient_workspace_id)
        ))
        .await
    }

    /// Reads a single linked Spruce reply conversation surface on an authorized patient workspace.
    pub async fn patient_spruce_reply(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        self.get(&format!(
            "/patient-workspaces/{}/spruce-replies/{}",
            path_escape(patient_workspace_id),
            path_escape(conversation_id)
        ))
        .await
    }

    /// Saves a physician's private draft edit against `expected_base`.
    pub async fn save_reply_proposal_draft(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        expected_base: ExpectedDraftBaseWire,
        text: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = SaveReplyProposalDraftBody {
            expected_base,
            text: text.to_owned(),
        };
        self.put(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/draft",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Requests a Coda revision on this physician's reply draft.
    pub async fn request_reply_proposal_revision(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        body: &RequestReplyProposalRevisionBody,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/revise",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            body,
        )
        .await
    }

    /// Restores the physician's pre-revision wording (`Restore my version`) over a `CodaRevision`.
    pub async fn restore_my_reply_version(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        expected_draft_revision: u64,
        idempotency_key: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = RestoreMyReplyVersionBody {
            expected_draft_revision,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/restore-my-version",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Adopts a newer Coda proposal (`Choose update`) over the physician's private draft.
    pub async fn choose_coda_reply_update(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        expected_draft_revision: u64,
        expected_proposal_version: u64,
        idempotency_key: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = ChooseCodaReplyUpdateBody {
            expected_draft_revision,
            expected_proposal_version,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/choose-update",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Records `Copy reply` on the displayed version without closing the reply card.
    pub async fn copy_spruce_reply(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        expected_base: ExpectedDraftBaseWire,
        idempotency_key: &str,
    ) -> Result<CopyReplyResponseWire, HttpError> {
        let body = CopyReplyBody {
            expected_base,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/copy",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Closes an open reply card as `No reply needed` (`Undo` remains available until new channel activity).
    pub async fn close_reply_no_reply_needed(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        expected_proposal_version: u64,
        idempotency_key: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = NoReplyNeededBody {
            expected_proposal_version,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/no-reply-needed",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Reopens a closed reply card via `Undo` after `No reply needed`.
    pub async fn undo_close_no_reply_needed(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        idempotency_key: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = ReplyCommandKeyBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/undo-no-reply-needed",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Requests Coda to draft a reply (`Ask Coda to reply`) when a linked conversation has no open card.
    pub async fn ask_coda_to_reply(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        idempotency_key: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = ReplyCommandKeyBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/ask-coda",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Initiates a `Wrong patient?` move (`AnotherPatient` or `NotAPatient`) with an `Undo` window.
    pub async fn move_spruce_conversation(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        destination: MoveDestinationWire,
        undo_window_seconds: Option<i64>,
        idempotency_key: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = MoveSpruceConversationBody {
            destination,
            undo_window_seconds,
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/move",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Cancels a pending `Wrong patient?` move during its `Undo` window.
    pub async fn undo_spruce_conversation_move(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        idempotency_key: &str,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        let body = ReplyCommandKeyBody {
            idempotency_key: idempotency_key.to_owned(),
        };
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/undo-move",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            &body,
        )
        .await
    }

    /// Marks an open reply card `Done` after the Spruce channel was disconnected.
    pub async fn mark_disconnected_reply_done(
        &self,
        patient_workspace_id: &str,
        conversation_id: &str,
        body: &DisconnectedReplyDoneBody,
    ) -> Result<PatientReplyConversationWire, HttpError> {
        self.post(
            &format!(
                "/patient-workspaces/{}/spruce-replies/{}/disconnected-done",
                path_escape(patient_workspace_id),
                path_escape(conversation_id)
            ),
            body,
        )
        .await
    }
}

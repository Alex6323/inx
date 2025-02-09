// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::proto;

use bee_message_stardust as stardust;

#[allow(missing_docs)]
#[derive(PartialEq, Debug)]
pub enum LedgerInclusionState {
    NoTransaction,
    Included,
    Conflicting,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug)]
pub enum ConflictReason {
    None,
    InputAlreadySpent,
    InputAlreadySpentInThisMilestone,
    InputNotFound,
    InputOutputSumMismatch,
    InvalidSignature,
    InvalidNetworkId,
    SemanticValidationFailed,
}

/// The metadata for a message with a given [`MessageId`](stardust::MessageId).
#[derive(PartialEq, Debug)]
pub struct MessageMetadata {
    /// The id of the message.
    pub message_id: stardust::MessageId,
    /// The parents of the messsage.
    pub parents: Vec<stardust::MessageId>,
    /// Status of the solidification process.
    pub is_solid: bool,
    /// Indicates that the message should be promoted.
    pub should_promote: bool,
    /// Indicates that the message should be reattached.
    pub should_reattach: bool,
    /// The milestone that referenced the message.
    pub referenced_by_milestone_index: u32,
    /// The corresponding milestone index.
    pub milestone_index: u32,
    /// Indicates if a message is part of the ledger state or not.
    pub ledger_inclusion_state: LedgerInclusionState,
    /// Indicates if a conflict occured, and if so holds information about the reason for the conflict.
    pub conflict_reason: ConflictReason,
}

impl TryFrom<proto::MessageMetadata> for MessageMetadata {
    type Error = Error;

    fn try_from(value: proto::MessageMetadata) -> Result<Self, Self::Error> {
        let ledger_inclusion_state = value.ledger_inclusion_state().into();
        let conflict_reason = value.conflict_reason().into();

        let mut parents = Vec::with_capacity(value.parents.len());
        for parent in value.parents {
            parents.push(parent.try_into()?);
        }

        Ok(MessageMetadata {
            message_id: value.message_id.ok_or(Error::MissingField("message_id"))?.try_into()?,
            parents,
            is_solid: value.solid,
            should_promote: value.should_promote,
            should_reattach: value.should_reattach,
            referenced_by_milestone_index: value.referenced_by_milestone_index,
            milestone_index: value.milestone_index,
            ledger_inclusion_state,
            conflict_reason,
        })
    }
}

impl From<proto::message_metadata::LedgerInclusionState> for LedgerInclusionState {
    fn from(value: proto::message_metadata::LedgerInclusionState) -> Self {
        match value {
            proto::message_metadata::LedgerInclusionState::NoTransaction => LedgerInclusionState::NoTransaction,
            proto::message_metadata::LedgerInclusionState::Included => LedgerInclusionState::Included,
            proto::message_metadata::LedgerInclusionState::Conflicting => LedgerInclusionState::Conflicting,
        }
    }
}

impl From<proto::message_metadata::ConflictReason> for ConflictReason {
    fn from(value: proto::message_metadata::ConflictReason) -> Self {
        match value {
            proto::message_metadata::ConflictReason::None => ConflictReason::None,
            proto::message_metadata::ConflictReason::InputAlreadySpent => ConflictReason::InputAlreadySpent,
            proto::message_metadata::ConflictReason::InputAlreadySpentInThisMilestone => {
                ConflictReason::InputAlreadySpentInThisMilestone
            }
            proto::message_metadata::ConflictReason::InputNotFound => ConflictReason::InputNotFound,
            proto::message_metadata::ConflictReason::InputOutputSumMismatch => ConflictReason::InputOutputSumMismatch,
            proto::message_metadata::ConflictReason::InvalidSignature => ConflictReason::InvalidSignature,
            proto::message_metadata::ConflictReason::SemanticValidationFailed => {
                ConflictReason::SemanticValidationFailed
            }
        }
    }
}

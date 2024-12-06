// Copyright (C) 2024 Subspace Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Fetching pieces of the archived history of Subspace Network.

use crate::object_fetcher::Error;
use crate::piece_getter::PieceGetter;
use futures::StreamExt;
use subspace_core_primitives::pieces::{Piece, PieceIndex};
use tracing::{debug, trace};

/// Concurrently downloads the exact pieces in `piece_indexes`, returning them in that order.
/// Each piece index must be unique.
///
/// If any piece can't be downloaded, returns an error.
// This code was copied and modified from subspace_service::sync_from_dsn::download_and_reconstruct_blocks():
// <https://github.com/autonomys/subspace/blob/d71ca47e45e1b53cd2e472413caa23472a91cd74/crates/subspace-service/src/sync_from_dsn/import_blocks.rs#L236-L322>
pub async fn download_pieces<PG>(
    piece_indexes: Vec<PieceIndex>,
    piece_getter: &PG,
) -> anyhow::Result<Vec<Piece>>
where
    PG: PieceGetter,
{
    debug!(
        count = piece_indexes.len(),
        ?piece_indexes,
        "Retrieving exact pieces"
    );

    // TODO:
    // - if we're close to the number of pieces in a segment, or we can't find a piece, use segment downloading and piece
    //   reconstruction instead
    // Currently most objects are limited to 4 pieces, so this isn't needed yet.
    let mut received_pieces = piece_getter.get_pieces(piece_indexes.clone()).await?;

    let mut pieces = Vec::new();
    pieces.resize(piece_indexes.len(), Piece::default());

    while let Some((piece_index, maybe_piece)) = received_pieces.next().await {
        // We want exact pieces, so any errors are fatal.
        let piece = maybe_piece?.ok_or(Error::PieceNotFound { piece_index })?;
        let index_position = piece_indexes
            .iter()
            .position(|i| *i == piece_index)
            .expect("get_pieces only returns indexes it was supplied; qed");
        pieces[index_position] = piece;
    }

    trace!(
        count = piece_indexes.len(),
        ?piece_indexes,
        "Successfully retrieved exact pieces"
    );

    Ok(pieces)
}

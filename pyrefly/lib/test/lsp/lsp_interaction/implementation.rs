/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::test::lsp::lsp_interaction::object_model::InitializeSettings;
use crate::test::lsp::lsp_interaction::object_model::LspInteraction;
use crate::test::lsp::lsp_interaction::util::get_test_files_root;

#[test]
fn test_implementation_basic() {
    let root = get_test_files_root();
    let mut interaction = LspInteraction::new();
    interaction.set_root(root.path().to_path_buf());
    interaction.initialize(InitializeSettings {
        ..Default::default()
    });
    
    // For now, since our implementation is minimal and returns empty results,
    // we'll test that the LSP method is properly handled and returns an empty array
    interaction.server.did_open("foo.py");
    interaction.server.implementation("foo.py", 5, 7);
    interaction.client.expect_empty_implementation_response();
}
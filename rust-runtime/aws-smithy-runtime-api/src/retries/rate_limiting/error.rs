/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::fmt;

/// Errors related to a token bucket.
#[derive(Debug)]
pub struct RateLimitingError {
    kind: ErrorKind,
}

impl RateLimitingError {
    pub fn no_tokens() -> Self {
        Self {
            kind: ErrorKind::NoTokens,
        }
    }

    pub fn bug(s: impl ToString) -> Self {
        Self {
            kind: ErrorKind::Bug(s.to_string()),
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    /// A token was requested but there were no tokens left in the bucket.
    NoTokens,
    /// This error should never occur and is a bug. Please report it.
    Bug(String),
}

impl fmt::Display for RateLimitingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match &self.kind {
            NoTokens => write!(f, "No more tokens are left in the bucket."),
            Bug(msg) => write!(f, "you've encountered a bug that needs reporting: {}", msg),
        }
    }
}

impl std::error::Error for RateLimitingError {}

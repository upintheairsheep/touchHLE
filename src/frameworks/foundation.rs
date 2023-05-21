/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The Foundation framework.
//!
//! A concept that Foundation really likes is "class clusters": abstract classes
//! with private concrete implementations. Apple has their own explanation of it
//! in [Cocoa Core Competencies](https://developer.apple.com/library/archive/documentation/General/Conceptual/DevPedia-CocoaCore/ClassCluster.html).
//! Being aware of this concept will make common types like `NSArray` and
//! `NSString` easier to understand.

pub mod ns_array;
pub mod ns_autorelease_pool;
pub mod ns_bundle;
pub mod ns_character_set;
pub mod ns_coder;
pub mod ns_data;
pub mod ns_dictionary;
pub mod ns_enumerator;
pub mod ns_file_manager;
pub mod ns_keyed_unarchiver;
pub mod ns_locale;
pub mod ns_log;
pub mod ns_notification;
pub mod ns_notification_center;
pub mod ns_null;
pub mod ns_object;
pub mod ns_process_info;
pub mod ns_run_loop;
pub mod ns_set;
pub mod ns_string;
pub mod ns_thread;
pub mod ns_timer;
pub mod ns_url;
pub mod ns_user_defaults;
pub mod ns_util;
pub mod ns_value;

#[derive(Default)]
pub struct State {
    ns_autorelease_pool: ns_autorelease_pool::State,
    ns_bundle: ns_bundle::State,
    ns_file_manager: ns_file_manager::State,
    ns_locale: ns_locale::State,
    ns_notification_center: ns_notification_center::State,
    ns_null: ns_null::State,
    ns_run_loop: ns_run_loop::State,
    ns_string: ns_string::State,
    ns_user_defaults: ns_user_defaults::State,
}

pub type NSInteger = i32;
pub type NSUInteger = u32;

pub type NSComparisonResult = NSInteger;
pub const NSOrderedAscending: NSComparisonResult = -1;
pub const NSOrderedSame: NSComparisonResult = 0;
pub const NSOrderedDescending: NSComparisonResult = 1;

/// Number of seconds.
pub type NSTimeInterval = f64;

/// Utility to help with implementing the `hash` method, which various classes
/// in Foundation have to do.
fn hash_helper<T: std::hash::Hash>(hashable: &T) -> NSUInteger {
    use std::hash::Hasher;

    // Rust documentation says DefaultHasher::new() should always return the
    // same instance, so this should give consistent hashes.
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hashable.hash(&mut hasher);
    let hash_u64: u64 = hasher.finish();
    (hash_u64 as u32) ^ ((hash_u64 >> 32) as u32)
}

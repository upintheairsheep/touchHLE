/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSURLRequest and NSMutableURLRequest`.

use super::{NSTimeInterval, NSUInteger};
use crate::frameworks::foundation::ns_string::to_rust_string;
use crate::msg;
use crate::objc::{autorelease, id, nil, objc_classes, release, ClassExports};

type NSURLRequestCachePolicy = NSUInteger;
const NSURLRequestUseProtocolCachePolicy: NSURLRequestCachePolicy = 0;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSURLRequest: NSObject

+ (id)requestWithURL:(id)url {
    msg![env; this requestWithURL:url
                      cachePolicy:NSURLRequestUseProtocolCachePolicy
                  timeoutInterval:60.0]
}

+ (id)requestWithURL:(id)url
         cachePolicy:(NSURLRequestCachePolicy)cache_policy
     timeoutInterval:(NSTimeInterval)timeout_interval {
    let new: id = msg![env; this alloc];
    let new: id = msg![env; new initWithURL:url
                                cachePolicy:cache_policy
                            timeoutInterval:timeout_interval];
    autorelease(env, new)
}

- (id)initWithURL:(id)url
        cachePolicy:(NSURLRequestCachePolicy)cache_policy
    timeoutInterval:(NSTimeInterval)timeout_interval {
    if url == nil {
        return nil;
    }
    let url_desc: id = msg![env; url description];
    log!(
        "TODO: [(NSURLRequest *){:?} requestWithURL:{} cachePolicy:{} timeoutInterval:{}]",
        this,
        to_rust_string(env, url_desc),
        cache_policy,
        timeout_interval,
    );
    release(env, this);
    nil
}

@end

@implementation NSMutableURLRequest: NSURLRequest
//TODO
@end

};

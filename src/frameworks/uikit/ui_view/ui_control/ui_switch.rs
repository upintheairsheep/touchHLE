/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UISwitch`.

use crate::objc::{id, impl_HostObject_with_superclass, objc_classes, ClassExports, NSZonePtr};

#[derive(Default)]
pub struct UISwitchHostObject {
    superclass: super::UIControlHostObject,
}
impl_HostObject_with_superclass!(UISwitchHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UISwitch: UIControl

+ (id)allocWithZone:(NSZonePtr)_zone {
    todo!("TODO: Not implemented yet.");
}

@end

};

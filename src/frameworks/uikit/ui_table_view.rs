use crate::objc::{
    id, msg, msg_super, nil, objc_classes, retain, release,
    ClassExports, HostObject, NSZonePtr,
};
use crate::frameworks::core_graphics::CGRect;

pub struct UITableViewHostObject {
    /// UITableViewDataSource
    pub data_source: id,

    /// UITableViewDelegate
    pub delegate: id,

    /// UITableViewStyle (Plain / Grouped)
    pub style: i32,
}

impl HostObject for UITableViewHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableView : UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host = UITableViewHostObject {
        data_source: nil,
        delegate: nil,
        style: 0,
    };
    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame style:(i32)style {
    let this: id = msg_super![env; this initWithFrame:frame];
    let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
    host.style = style;
    this
}

- (())setDataSource:(id)data_source {
    let data_source = retain(env, data_source);
    let old = {
        let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
        std::mem::replace(&mut host.data_source, data_source)
    };
    if old != nil {
        release(env, old);
    }
}

- (())setDelegate:(id)delegate {
    let delegate = retain(env, delegate);
    let old = {
        let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
        std::mem::replace(&mut host.delegate, delegate)
    };
    if old != nil {
        release(env, old);
    }
}

- (id)dataSource {
    env.objc.borrow::<UITableViewHostObject>(this).data_source
}

- (id)delegate {
    env.objc.borrow::<UITableViewHostObject>(this).delegate
}

- (())reloadData {
    // Stub: real UIKit would ask dataSource for rows & cells
    log_dbg!("[(UITableView*){:?} reloadData]", this);
}

- (id)dequeueReusableCellWithIdentifier:(id)_identifier {
    // Stub: no reuse pool yet
    nil
}

- (())dealloc {
    let (data_source, delegate) = {
        let host = env.objc.borrow::<UITableViewHostObject>(this);
        (host.data_source, host.delegate)
    };

    if data_source != nil {
        release(env, data_source);
    }
    if delegate != nil {
        release(env, delegate);
    }

    msg_super![env; this dealloc]
}

@end

};

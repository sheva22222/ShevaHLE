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

- (())setDataSource:(id)dataSource {
    let old = {
        let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
        let old = host.data_source;
        host.data_source = dataSource;
        old
    };
    if old != nil {
        release(env, old);
    }
    if dataSource != nil {
        retain(env, dataSource);
    }
}

- (id)dataSource {
    env.objc.borrow::<UITableViewHostObject>(this).data_source
}

- (())setDelegate:(id)delegate {
    let old = {
        let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
        let old = host.delegate;
        host.delegate = delegate;
        old
    };
    if old != nil {
        release(env, old);
    }
    if delegate != nil {
        retain(env, delegate);
    }
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
    let host = env.objc.borrow::<UITableViewHostObject>(this);
    if host.data_source != nil {
        release(env, host.data_source);
    }
    if host.delegate != nil {
        release(env, host.delegate);
    }
    msg_super![env; this dealloc]
}

@end

};

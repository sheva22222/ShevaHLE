use crate::objc::{
    id, msg, msg_super, nil, objc_classes, retain, release,
    ClassExports, HostObject, NSZonePtr,
};
use crate::frameworks::core_graphics::{CGFloat, CGRect};
use crate::frameworks::foundation::NSInteger;

pub struct UITableViewHostObject {
    delegate: id,
    data_source: id,
    style: i32,      // UITableViewStyle
    row_height: f32,
    allows_selection: bool,
}
impl HostObject for UITableViewHostObject {}


pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableView : NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host = UITableViewHostObject {
        delegate: nil,
        data_source: nil,
        style: 0,
        row_height: 44.0,
        allows_selection: true,
    };
    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
}


- (id)view {
    this
}

- (id)indexPathForSelectedRow {
    nil
}

- (())scrollToRowAtIndexPath:(NSInteger)_path atScrollPosition:(bool)_pos animated:(bool)_animated {
    log!("UITableView scrollToRowAtIndexPath");
}

- (())deselectRowAtIndexPath:(NSInteger)_path animated:(bool)_animated {
    log!("UITableView deselectRowAtIndexPath");
}

- (id)initWithFrame:(CGRect)frame {
    // Call UIView’s designated initializer
    let this: id = msg_super![env; this initWithFrame:frame];
    if this == nil {
        return nil;
    }

    // Initialize UITableView-specific state
    let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
    host.row_height = 44.0;
    host.allows_selection = true;
    host.delegate = nil;
    host.data_source = nil;

    this
}

- (id)initWithFrame:(CGRect)_frame style:(NSInteger)style {
    let this: id = msg_super![env; this init];
    let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
    host.style = style as i32;
    this
}

- (())setRowHeight:(CGFloat)height {
    let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
    host.row_height = height as f32;
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

- (())setAllowsSelection:(bool)selection {
    env.objc.borrow_mut::<UITableViewHostObject>(this).allows_selection = selection;
}

- (())setShowsVerticalScrollIndicator:(bool)_show {}
- (())setShowsHorizontalScrollIndicator:(bool)_show {}
- (())setAllowsSelectionDuringEditing:(bool)_editing {}
- (())setAccessoryType:(bool)_accessory {}
- (())setSeparatorColor:(bool)_color {}
- (())setSeparatorStyle:(bool)_style {}
- (())setSectionHeaderHeight:(bool)_height {}
- (())setSectionFooterHeight:(bool)_height {}
- (())setSectionIndexMinimumDisplayRowCount:(bool)_count {}
- (())setEditing:(bool)_editing {}
- (())setTableHeaderView:(bool)_view {}

@end

};

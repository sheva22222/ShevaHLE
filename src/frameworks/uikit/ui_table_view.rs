/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! UITableView / UITableViewController / UITableViewCell / NSIndexPath
//!
//! Minimal, stateful implementations to satisfy common Objective-C usage.
//! These are intentionally conservative stubs that store state and forward
//! data-source / delegate messages. Rendering is not handled here.

use crate::frameworks::foundation::{NSInteger, NSUInteger};
use crate::objc::{
    id, msg, msg_class, nil, release, retain, todo_objc_setter, ClassExports, HostObject, NSZonePtr,
};
use crate::objc_classes;
use crate::Environment;

/// NSIndexPath host object used by table view APIs.
struct NSIndexPathHostObject {
    row: NSInteger,
    section: NSInteger,
}
impl HostObject for NSIndexPathHostObject {}

/// UITableViewCell host object.
struct UITableViewCellHostObject {
    reuse_identifier: Option<id>,
    // Minimal content storage (apps often use textLabel etc. but we keep it simple).
    // If needed, UI elements can be added here.
}
impl HostObject for UITableViewCellHostObject {}

/// UITableView host object.
struct UITableViewHostObject {
    data_source: Option<id>,
    delegate: Option<id>,
    // Reuse pool: a simple vector of cells available for reuse.
    reuse_pool: Vec<id>,
    // Visible cells (for bookkeeping).
    visible_cells: Vec<id>,
    style: NSInteger,
}
impl HostObject for UITableViewHostObject {}

/// UITableViewController host object.
struct UITableViewControllerHostObject {
    table_view: Option<id>,
    style: NSInteger,
}
impl HostObject for UITableViewControllerHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// NSIndexPath implementation
@implementation NSIndexPath: NSObject

+ (id)indexPathForRow:(NSInteger)row inSection:(NSInteger)section {
    let host = NSIndexPathHostObject { row, section };
    let obj = env.objc.alloc_object(this, Box::new(host), &mut env.mem);
    // Keep it alive for callers.
    retain(env, obj);
    obj
}

- (NSInteger)row {
    env.objc.borrow::<NSIndexPathHostObject>(this).row
}

- (NSInteger)section {
    env.objc.borrow::<NSIndexPathHostObject>(this).section
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

// UITableViewCell implementation
@implementation UITableViewCell: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host = UITableViewCellHostObject {
        reuse_identifier: None
    };
    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
}

- (id)initWithStyle:(NSInteger)style reuseIdentifier:(id)identifier {
    // Store the reuse identifier (retain it).
    if !identifier.is_null() {
        retain(env, identifier);
        env.objc.borrow_mut::<UITableViewCellHostObject>(this).reuse_identifier = Some(identifier);
    }
    // Typical pattern: return self
    retain(env, this);
    this
}

- (id)reuseIdentifier {
    if let Some(idf) = env.objc.borrow::<UITableViewCellHostObject>(this).reuse_identifier {
        idf
    } else {
        nil
    }
}

- (())prepareForReuse {
    // No-op in the stub, but apps may override - this call is present.
}

- (())dealloc {
    let &UITableViewCellHostObject { reuse_identifier } = env.objc.borrow(this);
    if let Some(idf) = reuse_identifier {
        release(env, idf);
    }
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

// UITableView implementation
@implementation UITableView: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host = UITableViewHostObject {
        data_source: None,
        delegate: None,
        reuse_pool: Vec::new(),
        visible_cells: Vec::new(),
        style: 0,
    };
    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
}

// initWithFrame:style: - many apps call init, initWithFrame: or initWithFrame:style:. We support initWithFrame:style: lightly.
- (id)initWithFrame:(CGRect)frame style:(NSInteger)style {
    env.objc.borrow_mut::<UITableViewHostObject>(this).style = style;
    retain(env, this);
    this
}

- (id)init {
    // default style 0
    msg![env; this initWithFrame: Ptr::null() style: 0]
}

// Data source / delegate setters
- (())setDataSource:(id)dataSource {
    // release previous
    {
        let host = env.objc.borrow::<UITableViewHostObject>(this);
        if let Some(prev) = host.data_source {
            release(env, prev);
        }
    }
    if !dataSource.is_null() {
        retain(env, dataSource);
        env.objc.borrow_mut::<UITableViewHostObject>(this).data_source = Some(dataSource);
    } else {
        env.objc.borrow_mut::<UITableViewHostObject>(this).data_source = None;
    }
}

- (id)dataSource {
    if let Some(ds) = env.objc.borrow::<UITableViewHostObject>(this).data_source {
        ds
    } else {
        nil
    }
}

- (())setDelegate:(id)delegate {
    todo_objc_setter!(this, delegate);
}

- (id)delegate {
    if let Some(d) = env.objc.borrow::<UITableViewHostObject>(this).delegate {
        d
    } else {
        nil
    }
}

// Register / dequeue reuse helpers (very small subset)
- (id)dequeueReusableCellWithIdentifier:(id)identifier {
    if identifier.is_null() {
        return nil;
    }
    let mut host = env.objc.borrow_mut::<UITableViewHostObject>(this);
    // Try to find a cell in the reuse pool with matching reuseIdentifier pointer.
    for i in (0..host.reuse_pool.len()).rev() {
        let cell = host.reuse_pool[i];
        let cell_id: id = msg![env; cell reuseIdentifier];
        // Pointer equality is used for simplicity.
        if cell_id == identifier {
            // remove from pool and return
            host.reuse_pool.remove(i);
            // Prepare for reuse
            () = msg![env; cell prepareForReuse];
            return cell;
        }
    }
    // No reusable cell: instantiate a new one using UITableViewCell initWithStyle:reuseIdentifier:
    let cell: id = msg_class![env; UITableViewCell alloc];
    let cell: id = msg![env; cell initWithStyle: 0 reuseIdentifier: identifier];
    cell
}

// reloadData: ask the data source for content and call cellForRowAtIndexPath:
- (())reloadData {
    let host = env.objc.borrow_mut::<UITableViewHostObject>(this);
    host.visible_cells.clear();

    let data_source = match host.data_source {
        Some(ds) => ds,
        None => return, // nothing to do
    };

    // numberOfSectionsInTableView: optional, default to 1
    let num_sections: NSInteger = {
        // Attempt to call data source method; if it returns 0 we treat as 1.
        let v: NSInteger = msg![env; data_source numberOfSectionsInTableView: this];
        if v == 0 { 1 } else { v }
    };

    for section in 0..num_sections {
        let num_rows: NSInteger = msg![env; data_source tableView: this numberOfRowsInSection: section];
        for row in 0..num_rows {
            let index_path: id = msg_class![env; NSIndexPath indexPathForRow: row inSection: section];
            let cell: id = msg![env; data_source tableView: this cellForRowAtIndexPath: index_path];
            if !cell.is_null() {
                // Keep cell alive in visible_cells
                retain(env, cell);
                host.visible_cells.push(cell);
            }
            // release the temporary index path if alloc retained it
            release(env, index_path);
        }
    }
}

// selectRowAtIndexPath:animated:scrollPosition:
- (())selectRowAtIndexPath:(id)indexPath animated:(bool)animated scrollPosition:(NSInteger)scrollPosition {
    // Notify delegate if present
    if let Some(delegate) = env.objc.borrow::<UITableViewHostObject>(this).delegate {
        () = msg![env; delegate tableView: this didSelectRowAtIndexPath: indexPath];
    }
}

// cellForRowAtIndexPath: return a visible cell if present (linear search).
- (id)cellForRowAtIndexPath:(id)indexPath {
    let host = env.objc.borrow::<UITableViewHostObject>(this);
    // We cannot directly map back the index path to cell without bookkeeping.
    // For a simple implementation we return the first visible cell.
    host.visible_cells.get(0).copied().unwrap_or(nil)
}

// numberOfRowsInSection: convenience that forwards to data source if asked (some apps may call)
- (NSInteger)numberOfRowsInSection:(NSInteger)section {
    let host = env.objc.borrow::<UITableViewHostObject>(this);
    if let Some(ds) = host.data_source {
        msg![env; ds tableView: this numberOfRowsInSection: section]
    } else {
        0
    }
}

// dealloc - release retained resources and pool cells.
- (())dealloc {
    let &UITableViewHostObject { data_source, delegate, reuse_pool, visible_cells, .. } = env.objc.borrow(this);
    if let Some(ds) = data_source {
        release(env, ds);
    }
    if let Some(d) = delegate {
        release(env, d);
    }
    // release pooled cells
    for c in reuse_pool.iter() {
        release(env, *c);
    }
    // release visible cells
    for c in visible_cells.iter() {
        release(env, *c);
    }
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

// UITableViewController implementation
@implementation UITableViewController: UIViewController

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host = UITableViewControllerHostObject { table_view: None, style: 0 };
    env.objc.alloc_object(this, Box::new(host), &mut env.mem)
}

- (id)initWithStyle:(NSInteger)style {
    env.objc.borrow_mut::<UITableViewControllerHostObject>(this).style = style;
    retain(env, this);
    this
}

- (id)tableView {
    let mut host = env.objc.borrow_mut::<UITableViewControllerHostObject>(this);
    if let Some(tv) = host.table_view {
        tv
    } else {
        let tv: id = msg_class![env; UITableView alloc];
        let tv: id = msg![env; tv init]; // use our simple init
        retain(env, tv);
        host.table_view = Some(tv);
        tv
    }
}

// loadView - ensure a view is present (some apps rely on this)
- (())loadView {
    let tv = msg![env; this tableView];
    // Attempt to set the controller's view to the table view if setter exists.
    () = msg![env; this setView: tv];
}

// viewDidLoad - default behavior is no-op; apps override.
- (())viewDidLoad {
    // no-op
}

- (())dealloc {
    let &UITableViewControllerHostObject { table_view, .. } = env.objc.borrow(this);
    if let Some(tv) = table_view {
        release(env, tv);
    }
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};

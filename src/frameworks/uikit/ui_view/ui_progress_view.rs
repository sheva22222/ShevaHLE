use crate::objc::{id, msg_super, nil, objc_classes, ClassExports, NSZonePtr};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIProgressView: UIView

- (id)initWithProgressViewStyle:(i32)style {
    let this: id = msg_super![env; this init];
    log_dbg!("[(UIProgressView*){:?} initWithProgressViewStyle:{}]", this, style);
    this
}

- (())setProgress:(f32)progress {
    log_dbg!("[(UIProgressView*){:?} setProgress:{}]", this, progress);
}

- (())setProgress:(f32)progress animated:(bool)animated {
    log_dbg!(
        "[(UIProgressView*){:?} setProgress:{} animated:{}]",
        this,
        progress,
        animated
    );
}

- (())setProgressTintColor:(id)color {
    log_dbg!("[(UIProgressView*){:?} setProgressTintColor:{:?}]", this, color);
}

- (())setTrackTintColor:(id)color {
    log_dbg!("[(UIProgressView*){:?} setTrackTintColor:{:?}]", this, color);
}

@end

}

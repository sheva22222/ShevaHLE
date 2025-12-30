/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `AudioServices.h` (Audio Services)

use crate::dyld::{export_c_func, FunctionExports};
use crate::frameworks::carbon_core::OSStatus;
use crate::frameworks::core_audio_types::fourcc;
use crate::mem::{MutPtr, MutVoidPtr};
use crate::Environment;
use std::sync::atomic::{AtomicU32, Ordering};

/// Usually a FourCC.
type AudioServicesPropertyID = u32;
type SystemSoundID = u32;

const kAudioServicesUnsupportedPropertyError: OSStatus = fourcc(b"pty?") as _;
const kSystemSoundID_Vibrate: SystemSoundID = 0x00000FFF;

static SOUND_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

fn AudioServicesGetProperty(
    _env: &mut Environment,
    in_property_id: AudioServicesPropertyID,
    _in_specifier_size: u32,
    _in_specifier: crate::mem::ConstVoidPtr,
    _io_property_data_size: MutPtr<u32>,
    _out_property_data: MutVoidPtr,
) -> OSStatus {
    // Crash Bandicoot Nitro Kart 3D tries to use this property ID, which does
    // not seem to be documented anywhere? Assuming this is a bug.
    if in_property_id == 0xfff {
        kAudioServicesUnsupportedPropertyError
    } else {
        unimplemented!();
    }
}

fn AudioServicesPlaySystemSound(_env: &mut Environment, in_system_sound_id: SystemSoundID) {
    // assert_eq!(in_system_sound_id, kSystemSoundID_Vibrate);
    log!("TODO: vibration (AudioServicesPlaySystemSound)");
    // TODO: implement other system sounds
}

fn AudioServicesPlayAlertSound(env: &mut Environment, in_system_sound_id: SystemSoundID) {
    // Alerts usually behave similarly to PlaySystemSound but may also provide
    // UI/visual feedback on some platforms. For now, delegate to
    // AudioServicesPlaySystemSound so vibration / basic behavior works.
    log!("AudioServicesPlayAlertSound: playing alert sound id {:#x}", in_system_sound_id);
    AudioServicesPlaySystemSound(env, in_system_sound_id);
}

fn AudioServicesCreateSystemSoundID(
    env: &mut Environment,
    _in_file_url: crate::mem::ConstVoidPtr,
    out_system_sound_id: MutPtr<SystemSoundID>,
) -> OSStatus {
    // Minimal implementation: allocate a new non-zero SystemSoundID and return it.
    // The real implementation would parse the URL, load the sound, and fail if unsupported.
    if out_system_sound_id.is_null() {
        return kAudioServicesUnsupportedPropertyError;
    }

    let id = SOUND_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    // Write the generated id into guest memory via the environment's Mem API.
    env.mem.write(out_system_sound_id, id);
    0 // noErr
}

fn AudioServicesDisposeSystemSoundID(_env: &mut Environment, in_system_sound_id: SystemSoundID) {
    // In a full implementation we'd free any resources associated with the ID.
    // For now just log and ignore (the OS sound ID space is small).
    log!("AudioServicesDisposeSystemSoundID: disposing sound id {:#x}", in_system_sound_id);
}

fn AudioServicesAddSystemSoundCompletion(
    _env: &mut Environment,
    _in_system_sound_id: SystemSoundID,
    _in_run_loop: crate::mem::ConstVoidPtr,
    _in_run_loop_mode: crate::mem::ConstVoidPtr,
    _in_completion_proc: crate::mem::ConstVoidPtr,
    _in_client_data: crate::mem::MutVoidPtr,
) -> OSStatus {
    // Stub: completions are not currently supported. Accept the call but do nothing.
    // Returning noErr so callers that expect a success code won't fail.
    log!("AudioServicesAddSystemSoundCompletion: registration is a no-op in this implementation");
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(AudioServicesGetProperty(_, _, _, _, _)),
    export_c_func!(AudioServicesPlaySystemSound(_)),
    export_c_func!(AudioServicesPlayAlertSound(_)),
    export_c_func!(AudioServicesCreateSystemSoundID(_, _)),
    export_c_func!(AudioServicesDisposeSystemSoundID(_)),
    export_c_func!(AudioServicesAddSystemSoundCompletion(_, _, _, _, _)),
];

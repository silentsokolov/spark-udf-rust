use std::{result, thread};

use jni::{errors::Error, JNIEnv};

type ExcResult<T> = thread::Result<result::Result<T, Error>>;

pub fn unwrap_exc_or<T>(env: &JNIEnv, res: ExcResult<T>, default: T) -> T {
    let exception_class = env.find_class("java/lang/RuntimeException").unwrap();

    match res {
        Ok(jni_result) => match jni_result {
            Ok(v) => v,
            Err(jni_error) => {
                if !env.exception_check().unwrap() {
                    env.throw_new(exception_class, &jni_error.to_string())
                        .unwrap()
                }
                default
            }
        },
        // Rust panic
        Err(e) => {
            env.throw_new(exception_class, &e.downcast_ref::<&str>().unwrap())
                .unwrap();
            default
        }
    }
}

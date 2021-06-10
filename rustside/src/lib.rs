extern crate jni;

mod errors;

use std::{panic, ptr};

use jni::objects::{JClass, JObject, JString};
use jni::sys::jobjectArray;
use jni::JNIEnv;

use url::{Position, Url};

use crate::errors::unwrap_exc_or;

const JAVA_CLASS_STRING: &str = "java/lang/String";

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_silentsokolov_App_nativeUDF(
    env: JNIEnv,
    _class: JClass,
    url: JString,
) -> jobjectArray {
    let res = panic::catch_unwind(|| {
        if url.is_null() {
            return Ok(ptr::null_mut());
        }
        let url: String = env.get_string(url.into())?.into();
        let parse_result = match Url::parse(&url) {
            Ok(data) => data,
            Err(_) => {
                return Ok(ptr::null_mut());
            }
        };

        let element_class = env.find_class(JAVA_CLASS_STRING)?;
        let arr = env.new_object_array(3 as i32, element_class, JObject::null())?;

        env.set_object_array_element(arr, 0, env.new_string(parse_result.scheme())?)?;
        env.set_object_array_element(
            arr,
            1,
            env.new_string(
                parse_result
                    .host_str()
                    .unwrap_or_default()
                    .replacen("www.", "", 1),
            )?,
        )?;
        env.set_object_array_element(
            arr,
            2,
            env.new_string(&parse_result[Position::BeforePath..])?,
        )?;

        Ok(arr)
    });

    unwrap_exc_or(&env, res, ptr::null_mut())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jni::{InitArgsBuilder, JNIVersion, JavaVM};
    use std::sync::{Arc, Once};

    const JAVA_CLASS_OBJECT: &str = "java/lang/Object";

    pub fn jvm() -> &'static Arc<JavaVM> {
        static mut JVM: Option<Arc<JavaVM>> = None;
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            let jvm_args = InitArgsBuilder::new()
                .version(JNIVersion::V8)
                .option("-Xcheck:jni")
                .build()
                .unwrap_or_else(|e| panic!("{:#?}", e));

            let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));

            unsafe {
                JVM = Some(Arc::new(jvm));
            }
        });

        unsafe { JVM.as_ref().unwrap() }
    }

    #[test]
    fn parse_success_test() {
        let env = jvm()
            .attach_current_thread()
            .expect("failed to attach jvm thread");
        let any_class = env.find_class(JAVA_CLASS_OBJECT).unwrap();

        let out = Java_com_github_silentsokolov_App_nativeUDF(
            *env,
            any_class,
            env.new_string(
                "https://www.github.com/rust-lang/rust/issues?labels=E-easy&state=open#hash",
            )
            .unwrap(),
        );

        assert_ne!(out, ptr::null_mut(), "result is null");

        let scheme = env.get_object_array_element(out, 0).unwrap();
        let host = env.get_object_array_element(out, 1).unwrap();
        let path = env.get_object_array_element(out, 2).unwrap();

        let scheme: String = env
            .get_string(scheme.into())
            .expect("invalid result")
            .into();
        let host: String = env.get_string(host.into()).expect("invalid result").into();
        let path: String = env.get_string(path.into()).expect("invalid result").into();

        assert_eq!(scheme, "https");
        assert_eq!(host, "github.com");
        assert_eq!(path, "/rust-lang/rust/issues?labels=E-easy&state=open#hash");
    }

    #[test]
    fn parse_with_parseerr_test() {
        let env = jvm()
            .attach_current_thread()
            .expect("failed to attach jvm thread");
        let any_class = env.find_class(JAVA_CLASS_OBJECT).unwrap();

        let out = Java_com_github_silentsokolov_App_nativeUDF(
            *env,
            any_class,
            env.new_string("bad url").unwrap(),
        );

        assert_eq!(out, ptr::null_mut(), "result is not null");
    }

    #[test]
    fn parse_with_null_value_test() {
        let env = jvm()
            .attach_current_thread()
            .expect("failed to attach jvm thread");
        let any_class = env.find_class(JAVA_CLASS_OBJECT).unwrap();

        let out =
            Java_com_github_silentsokolov_App_nativeUDF(*env, any_class, JObject::null().into());

        assert_eq!(out, ptr::null_mut(), "result is not null");
    }
}

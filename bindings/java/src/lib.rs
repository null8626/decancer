use core::mem::transmute;
use jni::{
  objects::{JClass, JObject, JString},
  sys::{jboolean, jlong, jstring},
  JNIEnv,
};

macro_rules! jni_unwrap {
  ($env:ident, $value:expr, $return_value:expr) => {
    match $value {
      Ok(output) => output,
      Err(error) => {
        let _ = $env.throw_new("java/lang/RuntimeException", error.to_string());

        return $return_value;
      }
    }
  };

  ($env:ident, $value:expr) => {
    jni_unwrap!($env, $value, 0 as _)
  };
}

macro_rules! get_inner_field {
  ($env:ident, $this:ident, $return_value:expr) => {{
    let inner = jni_unwrap!(
      $env,
      $env
        .get_field(&$this, "inner", "J")
        .and_then(|field| field.j()),
      $return_value
    ) as *mut decancer::CuredString;

    if inner.is_null() {
      let _ = $env.throw_new(
        "java/lang/NullPointerException",
        "destroy() has been called prior to this.",
      );

      return $return_value;
    }

    inner
  }};

  ($env:ident, $this:ident) => {
    get_inner_field!($env, $this, 0 as _)
  };
}

#[no_mangle]
pub extern "system" fn Java_com_github_null8626_decancer_CuredString_cure<'local>(
  mut env: JNIEnv<'local>,
  _: JClass<'local>,
  input: JString<'local>,
) -> jlong {
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  match decancer::cure(&input) {
    Ok(output) => Box::into_raw(Box::new(output)) as _,

    Err(error) => {
      let _ = env.throw_new(
        "java/lang/IllegalArgumentException",
        <decancer::Error as AsRef<str>>::as_ref(&error),
      );

      0 as _
    }
  }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_equals<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner_field!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner) == &input)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_startsWith<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner_field!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner).starts_with(&input))
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_endsWith<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner_field!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner).ends_with(&input))
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_contains<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner_field!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner).contains(&input))
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_toString<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) -> jstring {
  let inner = get_inner_field!(env, this);

  jni_unwrap!(env, env.new_string((*inner).as_str())).into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_destroy<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) {
  let _ = Box::from_raw(get_inner_field!(env, this, ()));
}

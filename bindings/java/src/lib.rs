use core::mem::transmute;
use jni::{
  objects::{JClass, JObject, JString},
  sys::{jboolean, jlong, jstring},
  JNIEnv,
};

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_cure<'local>(
  mut env: JNIEnv<'local>,
  _: JClass<'local>,
  input: JString<'local>,
) -> jlong {
  let input: String = env.get_string(&input).unwrap().into();

  match decancer::cure(&input) {
    Ok(output) => Box::into_raw(Box::new(output)) as _,

    Err(error) => {
      env
        .throw_new("java/lang/IllegalArgumentException", error.to_string())
        .unwrap();

      0 as _
    }
  }
}

macro_rules! nullable {
  ($env:ident,$inner:expr,$fallback:expr) => {{
    let value = $inner;

    if value.is_null() {
      $env
        .throw_new(
          "java/lang/NullPointerException",
          "This object is already destroyed.",
        )
        .unwrap();

      return $fallback;
    }

    value
  }};
}

unsafe fn this_cured_string<'local>(
  env: &mut JNIEnv<'local>,
  this: JObject<'local>,
) -> *mut decancer::CuredString {
  env.get_field(this, "inner", "J").unwrap().as_jni().j as _
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_equals<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  other: JString<'local>,
) -> jboolean {
  let cured_string = nullable!(env, this_cured_string(&mut env, this), 0);
  let other: String = env.get_string(&other).unwrap().into();

  transmute((*cured_string) == &other)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_startsWith<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  other: JString<'local>,
) -> jboolean {
  let cured_string = nullable!(env, this_cured_string(&mut env, this), 0);
  let other: String = env.get_string(&other).unwrap().into();

  transmute((*cured_string).starts_with(&other))
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_endsWith<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  other: JString<'local>,
) -> jboolean {
  let cured_string = nullable!(env, this_cured_string(&mut env, this), 0);
  let other: String = env.get_string(&other).unwrap().into();

  transmute((*cured_string).ends_with(&other))
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_contains<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  other: JString<'local>,
) -> jboolean {
  let cured_string = nullable!(env, this_cured_string(&mut env, this), 0);
  let other: String = env.get_string(&other).unwrap().into();

  transmute((*cured_string).contains(&other))
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_toString<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) -> jstring {
  let cured_string = nullable!(env, this_cured_string(&mut env, this), 0 as _);

  env.new_string((*cured_string).as_str()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_null8626_decancer_CuredString_destroy<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) {
  let cured_string = nullable!(env, this_cured_string(&mut env, this), ());
  let _ = Box::from_raw(cured_string);
}

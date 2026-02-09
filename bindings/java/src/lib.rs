// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use jni::{
  JNIEnv,
  objects::{JClass, JObject, JObjectArray, JString, JValue},
  sys::{jboolean, jchar, jint, jlong, jobject, jstring},
};

mod util;

const CUREDSTRING_CLASS: &str = "io/github/null8626/decancer/CuredString";
const MATCH_CLASS: &str = "io/github/null8626/decancer/Match";

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_cure<'local>(
  mut env: JNIEnv<'local>,
  _: JClass<'local>,
  input: JString<'local>,
  options: jint,
) -> jlong {
  let input: String = util::jni_unwrap!(env, env.get_string(&input)).into();

  match decancer::cure(&input, options.cast_unsigned().into()) {
    Ok(output) => Box::into_raw(Box::new(output)) as _,

    Err(error) => {
      let _ = env.throw_new(
        "java/lang/IllegalArgumentException",
        <decancer::Error as AsRef<str>>::as_ref(&error),
      );

      0.into()
    },
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_disableLeetspeak<
  'local,
>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  switch: jboolean,
) {
  let inner = util::get_inner!(env, this, ());
  let inner_ref = unsafe { &mut *inner };

  inner_ref.disable_leetspeak(switch != 0);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_find<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jobject {
  let inner = util::get_inner!(env, this);
  let inner_ref = unsafe { &*inner };

  let input: String = util::jni_unwrap!(env, env.get_string(&input)).into();

  util::jni_unwrap!(
    env,
    util::get_matches_array(&mut env, inner_ref, inner_ref.find(&input).collect())
  )
  .into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_findMultiple<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
) -> jobject {
  let inner = util::get_inner!(env, this);
  let inner_ref = unsafe { &*inner };

  let inputs = util::get_string_array!(env, input);

  util::jni_unwrap!(
    env,
    util::get_matches_array(&mut env, inner_ref, inner_ref.find_multiple(inputs))
  )
  .into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_censor<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
  with: jchar,
) {
  let inner = util::get_inner!(env, this, ());
  let input: String = util::jni_unwrap!(env, env.get_string(&input), ()).into();

  match char::from_u32(with.into()) {
    Some(with) => unsafe {
      (*inner).censor(&input, with);
    },

    None => {
      let _ = env.throw_new(
        "java/lang/IllegalArgumentException",
        "Replacement character is a surrogate.",
      );
    },
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_censorMultiple<
  'local,
>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
  with: jchar,
) {
  let inner = util::get_inner!(env, this, ());

  match char::from_u32(with.into()) {
    Some(with) => unsafe {
      (*inner).censor_multiple(util::get_string_array!(env, input, ()), with);
    },

    None => {
      let _ = env.throw_new(
        "java/lang/IllegalArgumentException",
        "Replacement character is a surrogate.",
      );
    },
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_replace<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
  with: JString<'local>,
) {
  let inner = util::get_inner!(env, this, ());
  let input: String = util::jni_unwrap!(env, env.get_string(&input), ()).into();
  let with: String = util::jni_unwrap!(env, env.get_string(&with), ()).into();

  unsafe {
    (*inner).replace(&input, &with);
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_replaceMultiple<
  'local,
>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
  with: JString<'local>,
) {
  let inner = util::get_inner!(env, this, ());
  let with: String = util::jni_unwrap!(env, env.get_string(&with), ()).into();

  unsafe {
    (*inner).replace_multiple(util::get_string_array!(env, input, ()), &with);
  }
}

util::native_comparison_methods! {
  Java_io_github_null8626_decancer_CuredString_equals(inner, input) => (*inner) == input,

  Java_io_github_null8626_decancer_CuredString_startsWith(inner, input) => (*inner).starts_with(&input),

  Java_io_github_null8626_decancer_CuredString_endsWith(inner, input) => (*inner).ends_with(&input),

  Java_io_github_null8626_decancer_CuredString_contains(inner, input) => (*inner).contains(&input)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_toString<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) -> jstring {
  let inner = util::get_inner!(env, this);

  util::jni_unwrap!(env, env.new_string(unsafe { &**inner })).into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_close<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) {
  let inner = util::get_inner_unchecked!(env, this, ());

  if !inner.is_null() {
    let _ = unsafe { Box::from_raw(inner) };
    let descriptor = util::jni_unwrap!(env, env.get_field_id(CUREDSTRING_CLASS, "inner", "J"), ());

    util::jni_unwrap!(
      env,
      env.set_field_unchecked(this, descriptor, JValue::Long(0)),
      ()
    );
  }
}

// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use jni::{
  EnvUnowned, jni_sig, jni_str,
  objects::{JClass, JObject, JObjectArray, JString, JValue},
  strings::JNIStr,
  sys::{jboolean, jchar, jint, jlong, jobject, jstring},
};

mod errors;
mod util;

use errors::Error;

const CUREDSTRING_CLASS: &JNIStr = jni_str!("io/github/null8626/decancer/CuredString");
const MATCH_CLASS: &JNIStr = jni_str!("io/github/null8626/decancer/Match");

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_cure<'local>(
  mut unowned_env: EnvUnowned<'local>,
  _: JClass<'local>,
  input: JString<'local>,
  options: jint,
) -> jlong {
  let outcome = unowned_env.with_env(|_| {
    let input = input.to_string();

    match decancer::cure(&input, options.cast_unsigned().into()) {
      Ok(output) => Ok(Box::into_raw(Box::new(output)) as _),

      Err(error) => Err(Error::IllegalArgument(error.to_string())),
    }
  });

  outcome.resolve::<Error>()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_disableLeetspeak<
  'local,
>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  switch: jboolean,
) {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);
    let inner_ref = unsafe { &mut *inner };

    inner_ref.disable_leetspeak(switch);

    Ok(())
  });

  outcome.resolve::<Error>();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_disableAlphabeticalLeetspeak<
  'local,
>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  switch: jboolean,
) {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);
    let inner_ref = unsafe { &mut *inner };

    inner_ref.disable_alphabetical_leetspeak(switch);

    Ok(())
  });

  outcome.resolve::<Error>();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_find<'local>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jobject {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);
    let inner_ref = unsafe { &*inner };

    let input = input.to_string();

    Ok(
      util::jni_unwrap!(
        env,
        util::get_matches_array(env, inner_ref, inner_ref.find(&input).collect())
      )
      .into_raw(),
    )
  });

  outcome.resolve::<Error>()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_findMultiple<'local>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
) -> jobject {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);
    let inner_ref = unsafe { &*inner };

    let inputs = util::get_string_array!(env, input);

    Ok(
      util::jni_unwrap!(
        env,
        util::get_matches_array(env, inner_ref, inner_ref.find_multiple(inputs))
      )
      .into_raw(),
    )
  });

  outcome.resolve::<Error>()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_censor<'local>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  input: JString<'local>,
  with: jchar,
) {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);
    let input = input.to_string();

    char::from_u32(with.into()).map_or_else(
      || {
        Err(Error::IllegalArgument(
          "Replacement character is a surrogate.".into(),
        ))
      },
      |with| {
        unsafe { (*inner).censor(&input, with) }

        Ok(())
      },
    )
  });

  outcome.resolve::<Error>();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_censorMultiple<
  'local,
>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
  with: jchar,
) {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);

    char::from_u32(with.into()).map_or_else(
      || {
        Err(Error::IllegalArgument(
          "Replacement character is a surrogate.".into(),
        ))
      },
      |with| {
        unsafe {
          (*inner).censor_multiple(util::get_string_array!(env, input), with);
        }

        Ok(())
      },
    )
  });

  outcome.resolve::<Error>();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_replace<'local>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  input: JString<'local>,
  with: JString<'local>,
) {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);

    let input = input.to_string();
    let with = with.to_string();

    unsafe {
      (*inner).replace(&input, &with);
    }

    Ok(())
  });

  outcome.resolve::<Error>();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_replaceMultiple<
  'local,
>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
  with: JString<'local>,
) {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);
    let with = with.to_string();

    unsafe {
      (*inner).replace_multiple(util::get_string_array!(env, input), &with);
    }

    Ok(())
  });

  outcome.resolve::<Error>();
}

util::native_comparison_methods! {
  Java_io_github_null8626_decancer_CuredString_equals(inner, input) => (*inner) == input,

  Java_io_github_null8626_decancer_CuredString_startsWith(inner, input) => (*inner).starts_with(&input),

  Java_io_github_null8626_decancer_CuredString_endsWith(inner, input) => (*inner).ends_with(&input),

  Java_io_github_null8626_decancer_CuredString_contains(inner, input) => (*inner).contains(&input)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_toString<'local>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
) -> jstring {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner!(env, this);

    Ok(util::jni_unwrap!(env, env.new_string(unsafe { &**inner })).into_raw())
  });

  outcome.resolve::<Error>()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_close<'local>(
  mut unowned_env: EnvUnowned<'local>,
  this: JObject<'local>,
) {
  let outcome = unowned_env.with_env(|env| {
    let inner = util::get_inner_unchecked!(env, this);

    if !inner.is_null() {
      let _ = unsafe { Box::from_raw(inner) };
      let descriptor = util::jni_unwrap!(
        env,
        env.get_field_id(CUREDSTRING_CLASS, jni_str!("inner"), jni_sig!("J"))
      );

      util::jni_unwrap!(env, unsafe {
        env.set_field_unchecked(this, descriptor, JValue::Long(0))
      });
    }

    Ok(())
  });

  outcome.resolve::<Error>();
}

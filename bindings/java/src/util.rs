// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::Error;
use std::ops::Range;

use decancer::CuredString;
use jni::{
  Env, JValue, JValueOwned,
  errors::Result,
  jni_sig, jni_str,
  objects::{JObject, JObjectArray, JString},
  signature::{Primitive, ReturnType},
};

macro_rules! jni_unwrap {
  ($env:ident, $value:expr) => {
    match $value {
      Ok(output) => output,

      Err(error) => return Err(Error::Runtime(error.to_string())),
    }
  };
}

pub(super) use jni_unwrap;

pub fn get_inner_unchecked_inner<'a, 'local>(
  env: &'a mut Env<'local>,
  this: &'a JObject<'local>,
) -> Result<*mut decancer::CuredString> {
  let descriptor = env.get_field_id(super::CUREDSTRING_CLASS, jni_str!("inner"), jni_sig!("J"))?;

  unsafe { env.get_field_unchecked(this, descriptor, ReturnType::Primitive(Primitive::Long)) }
    .and_then(JValueOwned::j)
    .map(|value| value as _)
}

macro_rules! get_inner_unchecked {
  ($env:ident, $this:ident) => {{ $crate::util::jni_unwrap!($env, $crate::util::get_inner_unchecked_inner($env, &$this)) }};
}

pub(super) use get_inner_unchecked;

macro_rules! get_inner {
  ($env:ident, $this:ident) => {{
    let inner = $crate::util::get_inner_unchecked!($env, $this);

    if inner.is_null() {
      return Err(Error::NullPointer("close() has been called prior to this."));
    }

    inner
  }};
}

pub(super) use get_inner;

pub fn get_string_array_inner<'a, 'local>(
  env: &'a mut Env<'local>,
  object: &'a JObjectArray<'local>,
) -> Result<Vec<String>> {
  let input_len = object.len(env)?;
  let mut objects: Vec<String> = Vec::with_capacity(input_len);

  for i in 0..input_len {
    let obj = object.get_element(env, i)?;

    if !obj.is_null() {
      objects.push(env.as_cast::<JString>(&obj)?.to_string());
    }
  }

  Ok(objects)
}

macro_rules! get_string_array {
  ($env:ident, $object:ident) => {{ $crate::util::jni_unwrap!($env, $crate::util::get_string_array_inner($env, &$object)) }};
}

pub(super) use get_string_array;

pub fn get_matches_array<'local>(
  env: &mut Env<'local>,
  inner: &CuredString,
  matches: Vec<Range<usize>>,
) -> std::result::Result<JObjectArray<'local>, Error> {
  let array_len = i32::try_from(matches.len())
    .map_err(|err| Error::IllegalArgument(format!("Invalid matches array: {err}")))?;

  let array = env.new_object_array(array_len as _, super::MATCH_CLASS, JObject::null())?;

  for (idx, result) in matches.into_iter().enumerate() {
    let portion = env.new_string(&inner[result.clone()])?;

    let element = env.new_object(
      super::MATCH_CLASS,
      jni_sig!("(JJLjava/lang/String;)V"),
      &[
        JValue::Long(result.start.cast_signed() as _),
        JValue::Long(result.end.cast_signed() as _),
        JValue::Object(&portion.into()),
      ],
    )?;

    array.set_element(env, idx, element)?;
  }

  Ok(array)
}

macro_rules! native_comparison_methods {
  ($($method_name:ident($inner:ident, $string:ident) => $process:expr),*) => {
    $(
      #[unsafe(no_mangle)]
      pub unsafe extern "system" fn $method_name<'local>(
        mut unowned_env: jni::EnvUnowned<'local>,
        this: jni::objects::JObject<'local>,
        input: jni::objects::JString<'local>,
      ) -> jni::sys::jboolean {
        let outcome = unowned_env.with_env(|env| {
          let $inner = $crate::util::get_inner!(env, this);
          let $string = input.to_string();

          Ok(unsafe { $process })
        });

        outcome.resolve::<$crate::Error>()
      }
    )*
  }
}

pub(super) use native_comparison_methods;

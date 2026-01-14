// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use std::ops::Range;

use decancer::CuredString;
use jni::{
  JNIEnv,
  errors::Result,
  objects::{JObject, JObjectArray, JString, JValueGen},
  signature::{Primitive, ReturnType},
};

macro_rules! jni_unwrap {
  ($env:ident, $value:expr, $return_value:expr) => {
    match $value {
      Ok(output) => output,

      Err(error) => {
        let _ = $env.throw_new("java/lang/RuntimeException", error.to_string());

        return $return_value;
      },
    }
  };

  ($env:ident, $value:expr) => {
    $crate::util::jni_unwrap!($env, $value, 0 as _)
  };
}

pub(super) use jni_unwrap;

pub(super) fn get_inner_unchecked_inner<'a, 'local>(
  env: &'a mut JNIEnv<'local>,
  this: &'a JObject<'local>,
) -> Result<*mut decancer::CuredString> {
  let descriptor = env.get_field_id(super::CUREDSTRING_CLASS, "inner", "J")?;

  env
    .get_field_unchecked(this, descriptor, ReturnType::Primitive(Primitive::Long))
    .and_then(|field| field.j())
    .map(|value| value as _)
}

macro_rules! get_inner_unchecked {
  ($env:ident, $this:ident, $return_value:expr) => {{
    $crate::util::jni_unwrap!(
      $env,
      $crate::util::get_inner_unchecked_inner(&mut $env, &$this),
      $return_value
    )
  }};

  ($env:ident, $this:ident) => {
    $crate::util::get_inner_unchecked!($env, $this, 0 as _)
  };
}

pub(super) use get_inner_unchecked;

macro_rules! get_inner {
  ($env:ident, $this:ident, $return_value:expr) => {{
    let inner = $crate::util::get_inner_unchecked!($env, $this, $return_value);

    if inner.is_null() {
      let _ = $env.throw_new(
        "java/lang/NullPointerException",
        "close() has been called prior to this.",
      );

      return $return_value;
    }

    inner
  }};

  ($env:ident, $this:ident) => {
    $crate::util::get_inner!($env, $this, 0 as _)
  };
}

pub(super) use get_inner;

pub(super) fn get_string_array_inner<'a, 'local>(
  env: &'a mut JNIEnv<'local>,
  object: &'a JObjectArray<'local>,
) -> Result<Vec<String>> {
  let input_len = env.get_array_length(object)?;
  let mut objects: Vec<String> = Vec::with_capacity(input_len as _);

  for i in 0..input_len {
    let obj = env.get_object_array_element(object, i)?;

    objects.push(env.get_string(&JString::from(obj))?.into());
  }

  Ok(objects)
}

macro_rules! get_string_array {
  ($env:ident, $object:ident, $return_value:expr) => {{
    $crate::util::jni_unwrap!(
      $env,
      $crate::util::get_string_array_inner(&mut $env, &$object),
      $return_value
    )
  }};

  ($env:ident, $object:ident) => {
    $crate::util::get_string_array!($env, $object, 0 as _)
  };
}

pub(super) use get_string_array;

pub(super) fn get_matches_array<'local>(
  env: &mut JNIEnv<'local>,
  inner: &CuredString,
  matches: Vec<Range<usize>>,
) -> Result<JObjectArray<'local>> {
  let array = env.new_object_array(matches.len() as _, super::MATCH_CLASS, JObject::null())?;

  for (idx, result) in matches.into_iter().enumerate() {
    let element = env.new_object(
      super::MATCH_CLASS,
      "(JJLjava/lang/String;)V",
      &[
        JValueGen::Long(result.start as _),
        JValueGen::Long(result.end as _),
        JValueGen::Object(&env.new_string(&inner[result])?.into()),
      ],
    )?;

    env.set_object_array_element(&array, idx as _, element)?;
  }

  Ok(array)
}

macro_rules! native_comparison_methods {
  ($($method_name:ident($inner:ident, $string:ident) => $process:expr),*) => {
    $(
      #[unsafe(no_mangle)]
      pub unsafe extern "system" fn $method_name<'local>(
        mut env: jni::JNIEnv<'local>,
        this: jni::objects::JObject<'local>,
        input: jni::objects::JString<'local>,
      ) -> jni::sys::jboolean {
        let $inner = $crate::util::get_inner!(env, this);
        let $string: String = $crate::util::jni_unwrap!(env, env.get_string(&input)).into();

        unsafe { $process }.into()
      }
    )*
  }
}

pub(super) use native_comparison_methods;

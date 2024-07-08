#![allow(clippy::missing_safety_doc, clippy::unused_unit, clippy::missing_transmute_annotations)]

use jni::{
  errors::Result,
  objects::{JClass, JObject, JObjectArray, JString, JValue, JValueGen},
  signature::{Primitive, ReturnType},
  sys::{jboolean, jchar, jint, jlong, jobject, jstring},
  JNIEnv,
};
use std::mem::transmute;

const CUREDSTRING_CLASS: &str = "io/github/null8626/decancer/CuredString";
const MATCH_CLASS: &str = "io/github/null8626/decancer/Match";

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
    jni_unwrap!($env, $value, 0 as _)
  };
}

fn get_inner_unchecked<'a, 'local>(
  env: &'a mut JNIEnv<'local>,
  this: &'a JObject<'local>,
) -> Result<*mut decancer::CuredString> {
  let descriptor = env.get_field_id(CUREDSTRING_CLASS, "inner", "J")?;

  env
    .get_field_unchecked(this, descriptor, ReturnType::Primitive(Primitive::Long))
    .and_then(|field| field.j())
    .map(|value| value as _)
}

macro_rules! get_inner_unchecked {
  ($env:ident, $this:ident, $return_value:expr) => {{
    jni_unwrap!($env, get_inner_unchecked(&mut $env, &$this), $return_value)
  }};

  ($env:ident, $this:ident) => {
    get_inner_unchecked!($env, $this, 0 as _)
  };
}

macro_rules! get_inner {
  ($env:ident, $this:ident, $return_value:expr) => {{
    let inner = get_inner_unchecked!($env, $this, $return_value);

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
    get_inner!($env, $this, 0 as _)
  };
}

fn get_string_array<'a, 'local>(
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
    jni_unwrap!($env, get_string_array(&mut $env, &$object), $return_value)
  }};

  ($env:ident, $object:ident) => {
    get_string_array!($env, $object, 0 as _)
  };
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_cure<'local>(
  mut env: JNIEnv<'local>,
  _: JClass<'local>,
  input: JString<'local>,
  options: jint,
) -> jlong {
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  match decancer::cure(&input, transmute(options)) {
    Ok(output) => Box::into_raw(Box::new(output)) as _,

    Err(error) => {
      let _ = env.throw_new(
        "java/lang/IllegalArgumentException",
        <decancer::Error as AsRef<str>>::as_ref(&error),
      );

      0 as _
    },
  }
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_find<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jobject {
  let inner = get_inner!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  let matches = (*inner).find(&input).collect::<Vec<_>>();
  let array = jni_unwrap!(
    env,
    env.new_object_array(matches.len() as _, MATCH_CLASS, JObject::null())
  );

  for (idx, result) in matches.into_iter().enumerate() {
    let element = jni_unwrap!(
      env,
      env.new_object(
        MATCH_CLASS,
        "(JJLjava/lang/String;)V",
        &[
          JValueGen::Long(result.start as _),
          JValueGen::Long(result.end as _),
          JValueGen::Object(&jni_unwrap!(env, env.new_string(unsafe { &(*inner)[result] })).into()),
        ]
      )
    );

    jni_unwrap!(env, env.set_object_array_element(&array, idx as _, element));
  }

  array.into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_findMultiple<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
) -> jobject {
  let inner = get_inner!(env, this);

  let matches = (*inner).find_multiple(get_string_array!(env, input));
  let array = jni_unwrap!(
    env,
    env.new_object_array(matches.len() as _, MATCH_CLASS, JObject::null())
  );

  for (idx, result) in matches.into_iter().enumerate() {
    let element = jni_unwrap!(
      env,
      env.new_object(
        MATCH_CLASS,
        "(JJLjava/lang/String;)V",
        &[
          JValueGen::Long(result.start as _),
          JValueGen::Long(result.end as _),
          JValueGen::Object(&jni_unwrap!(env, env.new_string(unsafe { &(*inner)[result] })).into()),
        ]
      )
    );

    jni_unwrap!(env, env.set_object_array_element(&array, idx as _, element));
  }

  array.into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_censor<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
  with: jchar,
) {
  let inner = get_inner!(env, this, ());
  let input: String = jni_unwrap!(env, env.get_string(&input), ()).into();

  match char::from_u32(with as _) {
    Some(with) => {
      (*inner).censor(&input, with);
    },

    None => {
      let _ = env.throw_new(
        "java/lang/IllegalArgumentException",
        "Replacement character is a surrogate.",
      );
    },
  };
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_censorMultiple<
  'local,
>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
  with: jchar,
) {
  let inner = get_inner!(env, this, ());

  match char::from_u32(with as _) {
    Some(with) => {
      (*inner).censor_multiple(get_string_array!(env, input, ()), with);
    },

    None => {
      let _ = env.throw_new(
        "java/lang/IllegalArgumentException",
        "Replacement character is a surrogate.",
      );
    },
  };
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_replace<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
  with: JString<'local>,
) {
  let inner = get_inner!(env, this, ());
  let input: String = jni_unwrap!(env, env.get_string(&input), ()).into();
  let with: String = jni_unwrap!(env, env.get_string(&with), ()).into();

  (*inner).replace(&input, &with);
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_replaceMultiple<
  'local,
>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JObjectArray<'local>,
  with: JString<'local>,
) {
  let inner = get_inner!(env, this, ());
  let with: String = jni_unwrap!(env, env.get_string(&with), ()).into();

  (*inner).replace_multiple(get_string_array!(env, input, ()), &with);
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_equals<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner) == input)
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_startsWith<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner).starts_with(&input))
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_endsWith<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner).ends_with(&input))
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_contains<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
  input: JString<'local>,
) -> jboolean {
  let inner = get_inner!(env, this);
  let input: String = jni_unwrap!(env, env.get_string(&input)).into();

  transmute((*inner).contains(&input))
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_toString<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) -> jstring {
  let inner = get_inner!(env, this);

  jni_unwrap!(env, env.new_string((*inner).as_str())).into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_github_null8626_decancer_CuredString_destroy<'local>(
  mut env: JNIEnv<'local>,
  this: JObject<'local>,
) {
  let inner = get_inner_unchecked!(env, this, ());

  if !inner.is_null() {
    let _ = Box::from_raw(inner);
    let descriptor = jni_unwrap!(env, env.get_field_id(CUREDSTRING_CLASS, "inner", "J"), ());

    jni_unwrap!(
      env,
      env.set_field_unchecked(this, descriptor, JValue::Long(0)),
      ()
    );
  }
}

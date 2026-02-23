// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use std::{
  any::Any,
  error::Error as StdError,
  fmt::{self, Display, Formatter},
};

use jni::{
  Env,
  errors::{Error as JniError, ErrorPolicy, Result, ThrowRuntimeExAndDefault},
  jni_str,
  strings::{JNIStr, JNIString},
};

const RUNTIME_EXCEPTION_CLASS: &JNIStr = jni_str!("java/lang/RuntimeException");
const NULL_POINTER_EXCEPTION_CLASS: &JNIStr = jni_str!("java/lang/NullPointerException");
const ILLEGAL_ARGUMENT_EXCEPTION_CLASS: &JNIStr = jni_str!("java/lang/IllegalArgumentException");

#[derive(Debug)]
pub enum Error {
  Jni(JniError),
  IllegalArgument(String),
  NullPointer(&'static str),
  Runtime(String),
}

impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      Self::Jni(error) => Some(error),

      _ => None,
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Jni(error) => write!(f, "JNI: {error}"),

      Self::IllegalArgument(message) => write!(f, "IllegalArgumentException: {message}"),

      Self::NullPointer(message) => write!(f, "NullPointerException: {message}"),

      Self::Runtime(message) => write!(f, "RuntimeException: {message}"),
    }
  }
}

impl<T> ErrorPolicy<T, Self> for Error
where
  T: Default,
{
  type Captures<'unowned_env_local, 'native_method>
    = ()
  where
    'unowned_env_local: 'native_method;

  fn on_error<'unowned_env_local, 'native_method>(
    env: &mut Env<'unowned_env_local>,
    _cap: &mut Self::Captures<'unowned_env_local, 'native_method>,
    err: Self,
  ) -> Result<T>
  where
    'unowned_env_local: 'native_method,
  {
    if !env.exception_check() {
      let exception_class;
      let message;

      match err {
        Self::Jni(error) => {
          exception_class = RUNTIME_EXCEPTION_CLASS;
          message = JNIString::new(format!("JNI: {error}"));
        },

        Self::IllegalArgument(rust_message) => {
          exception_class = ILLEGAL_ARGUMENT_EXCEPTION_CLASS;
          message = JNIString::new(rust_message);
        },

        Self::NullPointer(rust_message) => {
          exception_class = NULL_POINTER_EXCEPTION_CLASS;
          message = JNIString::new(rust_message);
        },

        Self::Runtime(rust_message) => {
          exception_class = RUNTIME_EXCEPTION_CLASS;
          message = JNIString::new(rust_message);
        },
      }

      env
        .find_class(exception_class)
        .and_then(|cls| env.throw_new(cls, message))?;
    }

    Ok(T::default())
  }

  fn on_panic<'unowned_env_local, 'native_method>(
    env: &mut Env<'unowned_env_local>,
    cap: &mut Self::Captures<'unowned_env_local, 'native_method>,
    payload: Box<dyn Any + Send + 'static>,
  ) -> Result<T>
  where
    'unowned_env_local: 'native_method,
  {
    <ThrowRuntimeExAndDefault as ErrorPolicy<T, Self>>::on_panic(env, cap, payload)
  }
}

impl From<JniError> for Error {
  fn from(error: JniError) -> Self {
    Self::Jni(error)
  }
}

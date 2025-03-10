//! Utilities to be used with the [`get_generic_class!`](unity::get_generic_class) macro.
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};

#[repr(C)]
#[unity::class("App", "SingletonProcInst`1")]
pub struct SingletonProcInst { }

#[repr(C)]
#[unity::class("App", "SingletonClass`1")]
pub struct SingletonClass { }

#[repr(C)]
#[unity::class("App", "SingletonMonoBehaviour`1")]
pub struct SingletonMonoBehaviour { }

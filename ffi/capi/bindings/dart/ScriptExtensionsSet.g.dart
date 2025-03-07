// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An object that represents the Script_Extensions property for a single character
///
/// See the [Rust documentation for `ScriptExtensionsSet`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html) for more information.
final class ScriptExtensionsSet implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ScriptExtensionsSet._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XScriptExtensionsSet_destroy'));

  /// Check if the Script_Extensions property of the given code point covers the given script
  ///
  /// See the [Rust documentation for `contains`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html#method.contains) for more information.
  bool contains(int script) {
    final result = _ICU4XScriptExtensionsSet_contains(_underlying, script);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XScriptExtensionsSet_contains =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Uint16)>>('ICU4XScriptExtensionsSet_contains')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Get the number of scripts contained in here
  ///
  /// See the [Rust documentation for `iter`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter) for more information.
  int get count {
    final result = _ICU4XScriptExtensionsSet_count(_underlying);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XScriptExtensionsSet_count =
    _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XScriptExtensionsSet_count')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get script at index, returning an error if out of bounds
  ///
  /// See the [Rust documentation for `iter`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter) for more information.
  ///
  /// Throws [VoidError] on failure.
  int scriptAt(int index) {
    final result = _ICU4XScriptExtensionsSet_script_at(_underlying, index);
    if (!result.isOk) {
      throw VoidError();
    }
    return result.union.ok;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XScriptExtensionsSet_script_at =
    _capi<ffi.NativeFunction<_ResultUint16Void Function(ffi.Pointer<ffi.Opaque>, ffi.Size)>>('ICU4XScriptExtensionsSet_script_at')
      .asFunction<_ResultUint16Void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

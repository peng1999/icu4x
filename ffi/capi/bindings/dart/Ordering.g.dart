// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `Ordering`](https://docs.rs/core/latest/core/cmp/enum.Ordering.html) for more information.
enum Ordering {
  less,

  equal,

  greater;

  int get _underlying {
    switch (this) {
      case less:
        return -1;
      case equal:
        return 0;
      case greater:
        return 1;
    }
  }
}
